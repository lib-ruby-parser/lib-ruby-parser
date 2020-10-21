use std::convert::TryInto;

use crate::lexer::*;
use crate::maybe_byte::*;
use crate::parser::TokenValue;
use crate::parser::{Loc, Token};
use crate::source::buffer::*;
use crate::source::InputError;
use crate::str_term::{str_types, HeredocEnd, StrTerm, StringLiteral};
use crate::Context;
use crate::StackState;
use crate::StaticEnvironment;
use crate::TokenBuf;
use crate::{lex_states::*, LexState};

#[derive(Debug, Clone, Default)]
pub struct Lexer {
    pub buffer: Buffer,
    pub debug: bool,

    pub(crate) lval: Option<TokenValue>,
    pub(crate) lval_start: Option<usize>,
    pub(crate) lval_end: Option<usize>,

    pub(crate) strterm: Option<StrTerm>,
    pub(crate) state: LexState,
    pub(crate) paren_nest: i32,
    pub(crate) lpar_beg: i32,
    pub(crate) brace_nest: i32,

    cond_stack: StackState,
    cmdarg_stack: StackState,

    pub(crate) tokenbuf: TokenBuf,

    cur_arg: String,

    node_id: usize,

    max_numparam: usize,

    pub(crate) context: Context,
    pub(crate) in_kwarg: bool,

    pub(crate) command_start: bool,
    pub(crate) has_shebang: bool,
    token_seen: bool,
    token_info_enabled: bool,

    error_p: usize,

    do_print: usize,
    do_loop: usize,
    do_chomp: usize,
    do_split: usize,

    pub static_env: StaticEnvironment,
}

impl Lexer {
    pub(crate) const NULL_CHAR: u8 = 0x00;
    pub(crate) const CTRL_D_CHAR: u8 = 0x04;
    pub(crate) const CTRL_Z_CHAR: u8 = 0x1a;
    pub(crate) const LF_CHAR: u8 = 0x0c;
    pub(crate) const VTAB_CHAR: u8 = 0x0b;

    pub fn new(
        bytes: &Vec<u8>,
        name: &str,
        known_encoding: Option<String>,
    ) -> Result<Self, InputError> {
        Ok(Self {
            cond_stack: StackState::new("cond"),
            cmdarg_stack: StackState::new("cmdarg"),
            lpar_beg: -1, /* make lambda_beginning_p() == FALSE at first */
            buffer: Buffer::new(name, bytes.to_owned(), known_encoding)?,
            context: Context::new(),
            ..Self::default()
        })
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
        self.buffer.debug = debug;
    }

    pub fn tokenize_until_eof(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        loop {
            let token = self.yylex();
            match token {
                Token {
                    token_type: Self::END_OF_INPUT,
                    ..
                } => break,
                _ => tokens.push(token),
            }
        }

        tokens
    }

    pub(crate) fn yylex(&mut self) -> Token {
        self.lval = None;
        // println!("before yylex: {:#?}", self);

        let token_type = self.parser_yylex();

        let begin = std::mem::replace(&mut self.lval_start, None).unwrap_or(self.buffer.ptok);
        let mut end = std::mem::replace(&mut self.lval_end, None).unwrap_or(self.buffer.pcur);

        let mut token_value = self
            .lval
            .clone()
            .or_else(||
                // take raw value if nothing was manually captured
                self.buffer.substr_at(begin, end).map(|s| TokenValue::String(String::from_utf8(s.to_vec()).unwrap())))
            .unwrap_or(TokenValue::String("".to_owned()));

        // match self.strterm {
        //     Some(StrTerm::Heredoc(_)) => {
        //         // RUBY_SET_YYLLOC_FROM_STRTERM_HEREDOC
        //     },
        //     _ => {
        //         // RUBY_SET_YYLLOC
        //     }
        // };

        if token_type == Self::tNL {
            token_value = TokenValue::String("\n".to_owned());
            end = begin + 1;
        }

        let token = Token {
            token_type,
            token_value,
            loc: Loc { begin, end },
        };
        if self.debug {
            println!(
                "yylex ({:?}, {:?}, {:?})",
                Self::token_name(&token),
                token.token_value,
                token.loc
            );
        }
        token
    }

    pub(crate) fn nextc(&mut self) -> MaybeByte {
        self.buffer.nextc()
    }
    pub(crate) fn char_at(&self, idx: usize) -> MaybeByte {
        self.buffer.byte_at(idx)
    }
    pub(crate) fn token_flush(&mut self) {
        self.buffer.token_flush()
    }

    pub fn token_name(token: &Token) -> String {
        let id = token.token_type;
        let first_token = Self::YYerror;
        if id > first_token + 1 {
            let pos: usize = (id - first_token + 1).try_into().unwrap();
            Self::TOKEN_NAMES[pos].to_owned()
        } else if id == Self::END_OF_INPUT {
            "EOF".to_owned()
        } else {
            panic!(
                "token_name fails, {:?} (first token = {})",
                token, first_token
            )
        }
    }

    pub(crate) fn parser_yylex(&mut self) -> i32 {
        let mut c: MaybeByte;
        let mut space_seen: bool = false;
        let cmd_state: bool;
        let label: usize;
        let mut last_state: LexState;
        let token_seen = self.token_seen;

        let strterm = self.strterm.clone();
        if let Some(strterm) = strterm {
            match strterm {
                StrTerm::HeredocLiteral(heredoc) => {
                    return self.here_document(heredoc);
                }

                StrTerm::StringLiteral(string) => {
                    self.token_flush();
                    return self.parse_string(string);
                }
            }
        }

        cmd_state = self.command_start;
        self.command_start = false;
        self.token_seen = true;

        'retrying: loop {
            last_state = self.state.clone();
            self.token_flush();

            // handle EOF
            c = self.nextc();

            if c.is_eof() {
                return Self::END_OF_INPUT;
            }

            match c.to_option() {
                None
                | Some(Self::NULL_CHAR)
                | Some(Self::CTRL_D_CHAR)
                | Some(Self::CTRL_Z_CHAR) => return Self::END_OF_INPUT,

                // whitespaces
                Some(b'\r') => {
                    if !self.buffer.cr_seen {
                        self.buffer.cr_seen = true;
                        self.warn("encountered \\r in middle of line, treated as a mere space");
                    }
                }

                Some(b' ') | Some(b'\t') | Some(Self::LF_CHAR) | Some(Self::VTAB_CHAR) => {
                    space_seen = true;
                    continue 'retrying;
                }

                Some(b'#') | Some(b'\n') => {
                    if c == '#' {
                        // it's a comment
                        self.token_seen = token_seen;
                        // no magic_comment in shebang line
                        if !self.parser_magic_comment(
                            self.buffer.pcur,
                            self.buffer.pend - self.buffer.pcur,
                        ) {
                            if self.comment_at_top() {
                                self.set_file_encoding(self.buffer.pcur, self.buffer.pend)
                            }
                        }
                        self.buffer.goto_eol();
                    }
                    self.token_seen = token_seen;
                    let cc = self.is_lex_state_some(EXPR_BEG | EXPR_CLASS | EXPR_FNAME | EXPR_DOT)
                        && !self.is_lex_state_some(EXPR_LABELED);
                    if cc || self.is_lex_state_all(EXPR_ARG | EXPR_LABELED) {
                        if !cc && self.in_kwarg {
                            self.command_start = true;
                            self.set_lex_state(EXPR_BEG);
                            return Self::tNL;
                        }
                        continue 'retrying;
                    }

                    loop {
                        c = self.nextc();

                        match c.to_option() {
                            Some(b' ')
                            | Some(b'\t')
                            | Some(Self::LF_CHAR)
                            | Some(b'\r')
                            | Some(Self::VTAB_CHAR) => {
                                space_seen = true;
                            }
                            Some(b'#') => {
                                self.buffer.pushback(&c);
                                continue 'retrying;
                            }
                            Some(b'&') | Some(b'.') => {
                                if self.buffer.peek(b'.') == (c == '&') {
                                    self.buffer.pushback(&c);
                                    continue 'retrying;
                                }
                                self.buffer.ruby_sourceline -= 1;
                                self.buffer.nextline = self.buffer.lastline;
                            }
                            None => {
                                // EOF no decrement
                                self.buffer.eof_no_decrement();
                                self.command_start = true;
                                self.set_lex_state(EXPR_BEG);
                                return Self::tNL;
                            }
                            _ => {
                                self.buffer.ruby_sourceline -= 1;
                                self.buffer.nextline = self.buffer.lastline;
                                self.buffer.eof_no_decrement();
                                self.command_start = true;
                                self.set_lex_state(EXPR_BEG);
                                return Self::tNL;
                            }
                        }
                    }
                }

                Some(b'*') => {
                    let result: i32;

                    c = self.nextc();

                    if c == '*' {
                        c = self.nextc();
                        if c == '=' {
                            self.set_yylval_id("**=");
                            self.set_lex_state(EXPR_BEG);
                            return Self::tOP_ASGN;
                        }
                        self.buffer.pushback(&c);
                        if self.is_spacearg(&c, space_seen) {
                            self.warn("`**' interpreted as argument prefix");
                            result = Self::tDSTAR;
                        } else if self.is_beg() {
                            result = Self::tDSTAR;
                        } else {
                            result = self.warn_balanced(
                                Self::tPOW,
                                "**",
                                "argument prefix",
                                &c,
                                space_seen,
                                &last_state,
                            );
                        }
                    } else {
                        if c == '=' {
                            self.set_yylval_id("*=");
                            self.set_lex_state(EXPR_BEG);
                            return Self::tOP_ASGN;
                        }
                        self.buffer.pushback(&c);
                        if self.is_spacearg(&c, space_seen) {
                            self.warn("`*' interpreted as argument prefix");
                            result = Self::tSTAR;
                        } else if self.is_beg() {
                            result = Self::tSTAR;
                        } else {
                            result = self.warn_balanced(
                                Self::tSTAR2,
                                "*",
                                "argument prefix",
                                &c,
                                space_seen,
                                &last_state,
                            );
                        }
                    }

                    self.set_lex_state(if self.is_after_operator() {
                        EXPR_ARG
                    } else {
                        EXPR_BEG
                    });
                    return result;
                }

                Some(b'!') => {
                    c = self.nextc();
                    if self.is_after_operator() {
                        self.set_lex_state(EXPR_ARG);
                        if c == '@' {
                            return Self::tBANG;
                        }
                    } else {
                        self.set_lex_state(EXPR_BEG);
                    }
                    if c == '=' {
                        return Self::tNEQ;
                    }
                    if c == '~' {
                        return Self::tNMATCH;
                    }
                    self.buffer.pushback(&c);
                    return Self::tBANG;
                }

                Some(b'=') => {
                    if self.buffer.was_bol() {
                        // skip embedded rd document
                        if self.buffer.is_word_match("begin") {
                            self.buffer.goto_eol();
                            loop {
                                self.buffer.goto_eol();
                                c = self.nextc();
                                if c.is_eof() {
                                    self.compile_error("embedded document meets end of file");
                                    return Self::END_OF_INPUT;
                                }
                                if c == '=' && self.buffer.is_word_match("end") {
                                    break;
                                }
                                self.buffer.pushback(&c);
                            }
                            self.buffer.goto_eol();
                            continue 'retrying;
                        }
                    }

                    self.set_lex_state(if self.is_after_operator() {
                        EXPR_ARG
                    } else {
                        EXPR_BEG
                    });
                    c = self.nextc();
                    if c == '=' {
                        c = self.nextc();
                        if c == '=' {
                            return Self::tEQQ;
                        }
                        self.buffer.pushback(&c);
                        return Self::tEQ;
                    }
                    if c == '~' {
                        return Self::tMATCH;
                    } else if c == '>' {
                        return Self::tASSOC;
                    }
                    self.buffer.pushback(&c);
                    return Self::tEQL;
                }

                Some(b'<') => {
                    c = self.nextc();
                    if c == '<'
                        && !self.is_lex_state_some(EXPR_DOT | EXPR_CLASS)
                        && !self.is_end()
                        && (!self.is_arg() || self.is_lex_state_some(EXPR_LABELED) || space_seen)
                    {
                        if let Some(token) = self.heredoc_identifier() {
                            return token;
                        }
                    }
                    if self.is_after_operator() {
                        self.set_lex_state(EXPR_ARG);
                    } else {
                        if self.is_lex_state_some(EXPR_CLASS) {
                            self.command_start = true;
                        }
                        self.set_lex_state(EXPR_BEG);
                    }
                    if c == '=' {
                        c = self.nextc();
                        if c == '>' {
                            return Self::tCMP;
                        }
                        self.buffer.pushback(&c);
                        return Self::tLEQ;
                    }
                    if c == '<' {
                        c = self.nextc();
                        if c == '=' {
                            self.set_yylval_id("<<=");
                            self.set_lex_state(EXPR_BEG);
                            return Self::tOP_ASGN;
                        }
                        self.buffer.pushback(&c);
                        return self.warn_balanced(
                            Self::tLSHFT,
                            "<<",
                            "here document",
                            &c,
                            space_seen,
                            &last_state,
                        );
                    }
                    self.buffer.pushback(&c);
                    return Self::tLT;
                }

                Some(b'>') => {
                    self.set_lex_state(if self.is_after_operator() {
                        EXPR_ARG
                    } else {
                        EXPR_BEG
                    });

                    c = self.nextc();
                    if c == '=' {
                        return Self::tGEQ;
                    }

                    if c == '>' {
                        c = self.nextc();
                        if c == '=' {
                            self.set_yylval_id(">>=");
                            self.set_lex_state(EXPR_BEG);
                            return Self::tOP_ASGN;
                        }
                        self.buffer.pushback(&c);
                        return Self::tRSHFT;
                    }
                    self.buffer.pushback(&c);
                    return Self::tGT;
                }

                Some(b'"') => {
                    label = if self.is_label_possible(cmd_state) {
                        str_types::str_label
                    } else {
                        0
                    };
                    self.strterm =
                        self.new_strterm(str_types::str_dquote | label, b'"', None, None);
                    self.buffer.set_ptok(self.buffer.pcur - 1);
                    return Self::tSTRING_BEG;
                }

                Some(b'`') => {
                    if self.is_lex_state_some(EXPR_FNAME) {
                        self.set_lex_state(EXPR_ENDFN);
                        return Self::tBACK_REF2;
                    }
                    if self.is_lex_state_some(EXPR_DOT) {
                        if cmd_state {
                            self.set_lex_state(EXPR_CMDARG);
                        } else {
                            self.set_lex_state(EXPR_ARG);
                        }
                        return Self::tBACK_REF2;
                    }
                    self.strterm = self.new_strterm(str_types::str_xquote, b'`', None, None);
                    return Self::tXSTRING_BEG;
                }

                Some(b'\'') => {
                    label = if self.is_label_possible(cmd_state) {
                        str_types::str_label
                    } else {
                        0
                    };
                    self.strterm =
                        self.new_strterm(str_types::str_squote | label, b'\'', None, None);
                    self.buffer.set_ptok(self.buffer.pcur - 1);
                    return Self::tSTRING_BEG;
                }

                Some(b'?') => {
                    return self.parse_qmark(space_seen).unwrap_or(-1);
                }

                Some(b'&') => {
                    let result: i32;

                    c = self.nextc();
                    if c == '&' {
                        self.set_lex_state(EXPR_BEG);
                        c = self.nextc();
                        if c == '=' {
                            self.set_yylval_id("&&=");
                            self.set_lex_state(EXPR_BEG);
                            return Self::tOP_ASGN;
                        }
                        self.buffer.pushback(&c);
                        return Self::tANDOP;
                    } else if c == '=' {
                        self.set_yylval_id("&=");
                        self.set_lex_state(EXPR_BEG);
                        return Self::tOP_ASGN;
                    } else if c == '.' {
                        self.set_yylval_id("&.");
                        self.set_lex_state(EXPR_DOT);
                        return Self::tANDDOT;
                    }
                    self.buffer.pushback(&c);
                    if self.is_spacearg(&c, space_seen) {
                        // TODO: check for some warnings here
                        result = Self::tAMPER;
                    } else if self.is_beg() {
                        result = Self::tAMPER;
                    } else {
                        result = self.warn_balanced(
                            Self::tAMPER2,
                            "&",
                            "argument prefix",
                            &c,
                            space_seen,
                            &last_state,
                        );
                    }
                    self.set_lex_state(if self.is_after_operator() {
                        EXPR_ARG
                    } else {
                        EXPR_BEG
                    });
                    return result;
                }

                Some(b'|') => {
                    c = self.nextc();
                    if c == '|' {
                        self.set_lex_state(EXPR_BEG);
                        c = self.nextc();
                        if c == '=' {
                            self.set_yylval_id("||=");
                            self.set_lex_state(EXPR_BEG);
                            return Self::tOP_ASGN;
                        }
                        self.buffer.pushback(&c);
                        if last_state.is_some(EXPR_BEG) {
                            self.buffer.pushback(&Some(b'|'));
                            return Self::tPIPE;
                        }
                        return Self::tOROP;
                    }
                    if c == '=' {
                        self.set_yylval_id("|=");
                        self.set_lex_state(EXPR_BEG);
                        return Self::tOP_ASGN;
                    }
                    self.set_lex_state(if self.is_after_operator() {
                        EXPR_ARG
                    } else {
                        EXPR_BEG | EXPR_LABEL
                    });
                    self.buffer.pushback(&c);
                    return Self::tPIPE;
                }

                Some(b'+') => {
                    c = self.nextc();
                    if self.is_after_operator() {
                        self.set_lex_state(EXPR_ARG);
                        if c == '@' {
                            return Self::tUPLUS;
                        }
                        self.buffer.pushback(&c);
                        return Self::tPLUS;
                    }
                    if c == '=' {
                        self.set_yylval_id("+=");
                        self.set_lex_state(EXPR_BEG);
                        return Self::tOP_ASGN;
                    }
                    if self.is_beg()
                        || (self.is_spacearg(&c, space_seen) && self.arg_ambiguous(b'+'))
                    {
                        self.set_lex_state(EXPR_BEG);
                        self.buffer.pushback(&c);
                        if !c.is_eof() && c.is_digit() {
                            return self.parse_numeric(b'+');
                        }
                        return Self::tUPLUS;
                    }
                    self.set_lex_state(EXPR_BEG);
                    self.buffer.pushback(&c);
                    return self.warn_balanced(
                        Self::tPLUS,
                        "+",
                        "unary operator",
                        &c,
                        space_seen,
                        &last_state,
                    );
                }

                Some(b'-') => {
                    c = self.nextc();
                    if self.is_after_operator() {
                        self.set_lex_state(EXPR_ARG);
                        if c == '@' {
                            return Self::tUMINUS;
                        }
                        self.buffer.pushback(&c);
                        return Self::tMINUS;
                    }
                    if c == '=' {
                        self.set_yylval_id("-=");
                        self.set_lex_state(EXPR_BEG);
                        return Self::tOP_ASGN;
                    }
                    if c == '>' {
                        self.set_lex_state(EXPR_ENDFN);
                        return Self::tLAMBDA;
                    }
                    if self.is_beg()
                        || (self.is_spacearg(&c, space_seen) && self.arg_ambiguous(b'-'))
                    {
                        self.set_lex_state(EXPR_BEG);
                        self.buffer.pushback(&c);
                        if !c.is_eof() && c.is_digit() {
                            return Self::tUMINUS_NUM;
                        }
                        return Self::tUMINUS;
                    }
                    self.set_lex_state(EXPR_BEG);
                    self.buffer.pushback(&c);
                    return self.warn_balanced(
                        Self::tMINUS,
                        "-",
                        "unary operator",
                        &c,
                        space_seen,
                        &last_state,
                    );
                }

                Some(b'.') => {
                    let is_beg = self.is_beg();
                    self.set_lex_state(EXPR_BEG);
                    c = self.nextc();
                    if c == '.' {
                        c = self.nextc();
                        if c == '.' {
                            if self.paren_nest == 0 && self.buffer.is_looking_at_eol() {
                                self.warn("... at EOL, should be parenthesized?");
                            } else if self.lpar_beg >= 0 && self.lpar_beg + 1 == self.paren_nest {
                                if last_state.is_some(EXPR_LABEL) {
                                    return Self::tDOT3;
                                }
                            }
                            return if is_beg { Self::tBDOT3 } else { Self::tDOT3 };
                        }
                        self.buffer.pushback(&c);
                        return if is_beg { Self::tBDOT2 } else { Self::tDOT2 };
                    }
                    self.buffer.pushback(&c);
                    if !c.is_eof() && c.is_digit() {
                        let prev = if self.buffer.pcur - 1 > self.buffer.pbeg {
                            self.buffer.byte_at(self.buffer.pcur - 2)
                        } else {
                            MaybeByte::EndOfInput
                        };
                        self.parse_numeric(c.unwrap());
                        if prev.is_digit() {
                            self.yyerror0("unexpected fraction part after numeric literal");
                        } else {
                            self.yyerror0("no .<digit> floating literal anymore; put 0 before dot");
                        }
                        self.set_lex_state(EXPR_END);
                        self.buffer.set_ptok(self.buffer.pcur);
                        continue 'retrying;
                    }
                    self.set_yylval_id(".");
                    self.set_lex_state(EXPR_DOT);
                    return Self::tDOT;
                }

                Some(b'0') | Some(b'1') | Some(b'2') | Some(b'3') | Some(b'4') | Some(b'5')
                | Some(b'6') | Some(b'7') | Some(b'8') | Some(b'9') => {
                    return self.parse_numeric(c.unwrap());
                }

                Some(b')') => {
                    self.cond_pop();
                    self.cmdarg_pop();
                    self.set_lex_state(EXPR_ENDFN);
                    self.paren_nest -= 1;

                    return Self::tRPAREN;
                }

                Some(b']') => {
                    self.cond_pop();
                    self.cmdarg_pop();
                    self.set_lex_state(EXPR_END);
                    self.paren_nest -= 1;

                    return Self::tRBRACK;
                }

                Some(b'}') => {
                    // tSTRING_DEND does COND_POP and CMDARG_POP in the yacc's rule (lalrpop here)
                    if self.brace_nest == 0 {
                        self.brace_nest -= 1;
                        return Self::tSTRING_DEND;
                    }
                    self.brace_nest -= 1;
                    self.cond_pop();
                    self.cmdarg_pop();
                    self.set_lex_state(EXPR_END);
                    self.paren_nest -= 1;

                    return Self::tRCURLY;
                }

                Some(b':') => {
                    c = self.nextc();
                    if c == ':' {
                        if self.is_beg()
                            || self.is_lex_state_some(EXPR_CLASS)
                            || self.is_spacearg(&MaybeByte::EndOfInput, space_seen)
                        {
                            self.set_lex_state(EXPR_BEG);
                            return Self::tCOLON3;
                        }
                        self.set_yylval_id("::");
                        self.set_lex_state(EXPR_DOT);
                        return Self::tCOLON2;
                    }
                    if self.is_end() || c.is_space() || c == Some(b'#') {
                        self.buffer.pushback(&c);
                        let result = self.warn_balanced(
                            Self::tCOLON,
                            ":",
                            "symbol literal",
                            &c,
                            space_seen,
                            &last_state,
                        );
                        self.set_lex_state(EXPR_BEG);
                        return result;
                    }
                    match c.to_option() {
                        Some(b'\'') => {
                            self.strterm =
                                self.new_strterm(str_types::str_ssym, c.unwrap(), None, None)
                        }
                        Some(b'"') => {
                            self.strterm =
                                self.new_strterm(str_types::str_dsym, c.unwrap(), None, None)
                        }
                        _ => self.buffer.pushback(&c),
                    }
                    self.set_lex_state(EXPR_FNAME);
                    return Self::tSYMBEG;
                }

                Some(b'/') => {
                    if self.is_beg() {
                        self.strterm = self.new_strterm(str_types::str_regexp, b'/', None, None);
                        return Self::tREGEXP_BEG;
                    }
                    c = self.nextc();
                    if c == '=' {
                        self.set_yylval_id("/=");
                        self.set_lex_state(EXPR_BEG);
                        return Self::tOP_ASGN;
                    }
                    self.buffer.pushback(&c);
                    if self.is_spacearg(&c, space_seen) {
                        self.arg_ambiguous(b'/');
                        self.strterm = self.new_strterm(str_types::str_regexp, b'/', None, None);
                        return Self::tREGEXP_END;
                    }
                    self.set_lex_state(if self.is_after_operator() {
                        EXPR_ARG
                    } else {
                        EXPR_END
                    });
                    return self.warn_balanced(
                        Self::tDIVIDE,
                        "/",
                        "regexp literal",
                        &c,
                        space_seen,
                        &last_state,
                    );
                }

                Some(b'^') => {
                    c = self.nextc();
                    if c == '=' {
                        self.set_yylval_id("^=");
                        self.set_lex_state(EXPR_BEG);
                        return Self::tOP_ASGN;
                    }
                    self.set_lex_state(if self.is_after_operator() {
                        EXPR_ARG
                    } else {
                        EXPR_BEG
                    });
                    self.buffer.pushback(&c);
                    return Self::tCARET;
                }

                Some(b';') => {
                    self.set_lex_state(EXPR_BEG);
                    self.command_start = true;
                    return Self::tSEMI;
                }

                Some(b',') => {
                    self.set_lex_state(EXPR_BEG | EXPR_LABEL);
                    return Self::tCOMMA;
                }

                Some(b'~') => {
                    if self.is_after_operator() {
                        c = self.nextc();
                        if c != '@' {
                            self.buffer.pushback(&c);
                        }
                        self.set_lex_state(EXPR_ARG);
                    } else {
                        self.set_lex_state(EXPR_BEG);
                    }

                    return Self::tTILDE;
                }

                Some(b'(') => {
                    let mut result: i32 = Self::tLPAREN2;

                    if self.is_beg() {
                        result = Self::tLPAREN;
                    } else if !space_seen {
                        // foo( ... ) => method call, no ambiguity
                    } else if self.is_arg() || self.is_lex_state_all(EXPR_END | EXPR_LABEL) {
                        result = Self::tLPAREN_ARG;
                    } else if self.is_lex_state_some(EXPR_ENDFN) && !self.is_lambda_beginning() {
                        self.warn("parentheses after method name is interpreted as an argument list, not a decomposed argument");
                    }

                    self.paren_nest += 1;
                    self.cond_push(false);
                    self.cmdarg_push(false);
                    self.set_lex_state(EXPR_BEG | EXPR_LABEL);

                    return result;
                }

                Some(b'[') => {
                    let mut result: i32 = Self::tLBRACK2;

                    self.paren_nest += 1;
                    if self.is_after_operator() {
                        c = self.nextc();
                        if c == ']' {
                            self.set_lex_state(EXPR_ARG);
                            c = self.nextc();
                            if c == '=' {
                                return Self::tASET;
                            }
                            self.buffer.pushback(&c);
                            return Self::tAREF;
                        }
                        self.buffer.pushback(&c);
                        self.set_lex_state(EXPR_ARG | EXPR_LABEL);
                        return Self::tLBRACK2;
                    } else if self.is_beg() {
                        result = Self::tLBRACK;
                    } else if self.is_arg() && (space_seen || self.is_lex_state_some(EXPR_LABELED))
                    {
                        result = Self::tLBRACK;
                    }
                    self.set_lex_state(EXPR_BEG | EXPR_LABEL);
                    self.cond_push(false);
                    self.cmdarg_push(false);
                    return result;
                }

                Some(b'{') => {
                    self.brace_nest += 1;

                    let result: i32;

                    if self.is_lambda_beginning() {
                        result = Self::tLAMBEG;
                    } else if self.is_lex_state_some(EXPR_LABELED) {
                        result = Self::tLBRACE;
                    } else if self.is_lex_state_some(EXPR_ARG_ANY | EXPR_END | EXPR_ENDFN) {
                        result = Self::tLCURLY;
                    } else if self.is_lex_state_some(EXPR_ENDARG) {
                        result = Self::tLBRACE_ARG;
                    } else {
                        result = Self::tLBRACE;
                    }

                    if result != Self::tLBRACE {
                        self.command_start = true;
                        self.set_lex_state(EXPR_BEG);
                    } else {
                        self.set_lex_state(EXPR_BEG | EXPR_LABEL);
                    }

                    self.paren_nest += 1;
                    self.cond_push(false);
                    self.cmdarg_push(false);
                    return result;
                }

                Some(b'\\') => {
                    c = self.nextc();
                    if c == '\n' {
                        space_seen = true;
                        continue 'retrying; /* skip \\n */
                    }
                    if c == ' ' {
                        return Self::tSP;
                    }
                    if c.is_space() {
                        match c.to_option() {
                            Some(b'\t') => return Self::tSLASH_T,
                            Some(Self::LF_CHAR) => return Self::tSLASH_F,
                            Some(b'\r') => return Self::tSLASH_R,
                            Some(Self::VTAB_CHAR) => return Self::tVTAB,
                            Some(other) => unreachable!("unsupported space char {:?}", other),
                            None => {}
                        }
                    }
                    self.buffer.pushback(&c);
                    return Self::tBACKSLASH;
                }

                Some(b'%') => {
                    return self.parse_percent(space_seen, last_state);
                }

                Some(b'$') => {
                    return self.parse_gvar(last_state);
                }

                Some(b'@') => {
                    return self.parse_atmark(last_state);
                }

                Some(b'_') => {
                    if self.buffer.was_bol() && self.buffer.is_whole_match(b"__END__", 0) {
                        self.buffer.eofp = true;
                        return Self::END_OF_INPUT;
                    }
                    self.newtok();
                }

                _ => {
                    if !self.parser_is_identchar() {
                        self.compile_error(&format!("Invalid u8 `{}' in expression", c.unwrap()));
                        self.token_flush();
                        continue 'retrying;
                    }

                    self.newtok();
                }
            }

            break;
        }

        return self.parse_ident(&c, cmd_state);
    }

    pub fn set_lex_state(&mut self, states: i32) {
        self.state.set(states)
    }

    pub(crate) fn is_lex_state_some(&self, states: i32) -> bool {
        self.state.is_some(states)
    }

    pub(crate) fn is_lex_state_all(&self, states: i32) -> bool {
        self.state.is_all(states)
    }

    pub(crate) fn warn(&self, message: &str) {
        if self.debug {
            println!("WARNING: {}", message)
        }
    }

    pub(crate) fn set_yylval_id(&mut self, id: &str) {
        if self.debug {
            println!("set_yylval_id({})", id);
        }
        self.lval = Some(TokenValue::String(id.to_owned()));
    }

    pub(crate) fn is_spacearg(&self, c: &MaybeByte, space_seen: bool) -> bool {
        self.is_arg() && space_seen && !c.is_space()
    }

    pub(crate) fn is_beg(&self) -> bool {
        self.is_lex_state_some(EXPR_BEG_ANY) || self.is_lex_state_all(EXPR_ARG | EXPR_LABELED)
    }

    pub(crate) fn warn_balanced(
        &self,
        token_type: i32,
        op: &str,
        syn: &str,
        c: &MaybeByte,
        space_seen: bool,
        last_state: &LexState,
    ) -> i32 {
        if !last_state.is_some(EXPR_CLASS | EXPR_DOT | EXPR_FNAME | EXPR_ENDFN)
            && space_seen & !c.is_space()
        {
            self.warn(&format!("`{}' after local variable or literal is interpreted as binary operator even though it seems like {}", op, syn));
        }
        token_type
    }

    pub(crate) fn is_after_operator(&self) -> bool {
        self.is_lex_state_some(EXPR_FNAME | EXPR_DOT)
    }

    pub(crate) fn compile_error(&self, message: &str) {
        if self.debug {
            println!("Compile error: {}", message)
        }
    }

    pub(crate) fn is_end(&self) -> bool {
        self.is_lex_state_some(EXPR_END_ANY)
    }

    pub(crate) fn is_arg(&self) -> bool {
        self.is_lex_state_some(EXPR_ARG_ANY)
    }

    pub(crate) fn is_label_possible(&self, cmd_state: bool) -> bool {
        (self.is_lex_state_some(EXPR_LABEL | EXPR_ENDFN) && !cmd_state) || self.is_arg()
    }

    pub(crate) fn new_strterm(
        &self,
        func: usize,
        term: u8,
        paren: Option<u8>,
        heredoc_end: Option<HeredocEnd>,
    ) -> Option<StrTerm> {
        Some(StrTerm::new_literal(StringLiteral::new(
            0,
            func,
            paren,
            term,
            heredoc_end,
        )))
    }

    pub(crate) fn escaped_control_code(&self, c: &MaybeByte) -> Option<u8> {
        if *c == ' ' {
            return Some(b's');
        }
        if *c == '\n' {
            return Some(b'n');
        }
        if *c == '\t' {
            return Some(b't');
        }
        if *c == 0x0b as u8 {
            return Some(b'v');
        }
        if *c == '\r' {
            return Some(b'r');
        }
        if *c == 0x0c as u8 {
            return Some(b'f');
        }
        None
    }

    pub(crate) fn parse_qmark_ternary(&mut self, c: &MaybeByte) -> Result<i32, ()> {
        self.buffer.pushback(c);
        self.set_lex_state(EXPR_VALUE);
        Ok(Self::tEH)
    }

    pub(crate) fn warn_space_char(&mut self, c: u8, prefix: &str) {
        self.warn(&format!(
            "invalid character syntax; use \"{}\"\\{}",
            prefix, c
        ))
    }

    pub(crate) fn parse_qmark(&mut self, space_seen: bool) -> Result<i32, ()> {
        // let enc;
        let mut c;

        if self.is_end() {
            self.set_lex_state(EXPR_VALUE);
            return Ok(Self::tEH);
        }
        c = self.nextc();
        if c.is_eof() {
            self.compile_error("incomplete character syntax");
            return Ok(Self::END_OF_INPUT);
        }
        if c.is_space() {
            if !self.is_arg() {
                if let Some(c2) = self.escaped_control_code(&c) {
                    self.warn_space_char(c2, "?");
                }
            }
            return self.parse_qmark_ternary(&c);
        }
        self.newtok();
        // enc = self.enc;

        if !self.parser_is_ascii() {
            if self.tokadd_mbchar(&c).is_err() {
                return Ok(Self::END_OF_INPUT);
            }
        } else if (c.is_alnum() || c == '_')
            && self.buffer.pcur < self.buffer.pend
            && self.is_identchar(self.buffer.pcur, self.buffer.pend)
        {
            if space_seen {
                let start = self.buffer.pcur - 1;
                let mut ptr = start;
                loop {
                    if let Some(n) = self.parser_precise_mbclen(ptr) {
                        ptr += n;
                    } else {
                        return Err(());
                    }

                    if !(ptr < self.buffer.pend && self.is_identchar(ptr, self.buffer.pend)) {
                        break;
                    }
                }
                // rb_warn2("`?' just followed by `%.*s' is interpreted as" \
                //  " a conditional operator, put a space after `?'",
                //  WARN_I((int)(ptr - start)), WARN_S_L(start, (ptr - start)));
            }
            return self.parse_qmark_ternary(&c);
        } else if c == '\\' {
            if self.buffer.peek(b'u') {
                self.nextc();
                self.tokadd_utf8(None, 0, 0);
            } else if !self.buffer.is_eol() && !self.char_at(self.buffer.pcur).is_ascii() {
                c = self.char_at(self.buffer.pcur);
                self.nextc();
                if self.tokadd_mbchar(&c).is_err() {
                    return Ok(Self::END_OF_INPUT);
                }
            } else {
                let byte = self.read_escape(0);
                self.tokadd(&byte);
            }
        } else {
            self.tokadd(&c);
        }
        self.tokfix();
        self.set_yylval_str(self.tokenbuf.clone());
        self.set_lex_state(EXPR_END);
        return Ok(Self::tCHAR);
    }

    pub(crate) fn arg_ambiguous(&self, c: u8) -> bool {
        self.warn(&format!(
            "ambiguous first argument; put parentheses or a space even after `{}' operator",
            c
        ));
        true
    }

    // pub(crate) fn tokadd(&mut self, c: &LexChar) {
    //     let c = c.unwrap();
    //     self.tokenbuf.push(c);
    // }

    // pub(crate) fn tokadd_byte(&mut self, byte: u8) {
    //     unimplemented!()
    // }

    pub(crate) fn toklen(&self) -> usize {
        self.tokenbuf.len()
    }

    pub(crate) fn tokfix(&self) {
        // nop
    }

    pub(crate) fn yyerror0(&self, message: &str) {
        if self.debug {
            println!("yyerror0: {}", message)
        }
        panic!("{}", message)
    }

    pub(crate) fn is_lambda_beginning(&self) -> bool {
        self.lpar_beg == self.paren_nest
    }

    pub fn cond_push(&mut self, value: bool) {
        self.cond_stack.push(value)
    }

    pub fn cond_pop(&mut self) {
        self.cond_stack.pop()
    }

    pub fn is_cond_active(&self) -> bool {
        self.cond_stack.is_active()
    }

    pub fn cmdarg_push(&mut self, value: bool) {
        self.cmdarg_stack.push(value)
    }

    pub fn cmdarg_pop(&mut self) {
        self.cmdarg_stack.pop()
    }

    pub fn is_cmdarg_active(&self) -> bool {
        self.cmdarg_stack.is_active()
    }

    pub(crate) fn percent_unknown(&mut self, term: &MaybeByte) -> i32 {
        self.buffer.pushback(term);
        let len = self.parser_precise_mbclen(self.buffer.pcur);
        if let Some(len) = len {
            self.buffer.pcur += len;
            self.yyerror0("unknown type of %string");
        }
        return Self::END_OF_INPUT;
    }

    pub(crate) fn percent_quatation(&mut self, c: &MaybeByte, ptok: usize) -> i32 {
        let mut c = c.clone();
        let mut term: MaybeByte;
        let mut paren: Option<u8>;

        if c.is_eof() || !c.is_alnum() {
            term = c.clone();
            if !c.is_ascii() {
                return self.percent_unknown(&term);
            }
            c = MaybeByte::new('Q');
        } else {
            term = self.nextc();
            if term.is_alnum() {
                return self.percent_unknown(&term);
            }
        }

        if term.is_eof() {
            self.compile_error("unterminated quoted string meets end of file");
            return Self::END_OF_INPUT;
        }

        paren = term.to_option();
        if term == '(' {
            term = MaybeByte::new(')')
        } else if term == '[' {
            term = MaybeByte::new(']')
        } else if term == '{' {
            term = MaybeByte::new('}')
        } else if term == '<' {
            term = MaybeByte::new('>')
        } else {
            paren = None
        }

        self.buffer.ptok = ptok - 1;
        match c.to_option() {
            Some(b'Q') => {
                self.strterm = self.new_strterm(str_types::str_dquote, term.unwrap(), paren, None);
                return Self::tSTRING_BEG;
            }
            Some(b'q') => {
                self.strterm = self.new_strterm(str_types::str_squote, term.unwrap(), paren, None);
                return Self::tSTRING_BEG;
            }
            Some(b'W') => {
                self.strterm = self.new_strterm(str_types::str_dword, term.unwrap(), paren, None);
                return Self::tWORDS_BEG;
            }
            Some(b'w') => {
                self.strterm = self.new_strterm(str_types::str_sword, term.unwrap(), paren, None);
                return Self::tQWORDS_BEG;
            }
            Some(b'I') => {
                self.strterm = self.new_strterm(str_types::str_dword, term.unwrap(), paren, None);
                return Self::tSYMBOLS_BEG;
            }
            Some(b'i') => {
                self.strterm = self.new_strterm(str_types::str_sword, term.unwrap(), paren, None);
                return Self::tQSYMBOLS_BEG;
            }
            Some(b'x') => {
                self.strterm = self.new_strterm(str_types::str_xquote, term.unwrap(), paren, None);
                return Self::tXSTRING_BEG;
            }
            Some(b'r') => {
                self.strterm = self.new_strterm(str_types::str_regexp, term.unwrap(), paren, None);
                return Self::tREGEXP_BEG;
            }
            Some(b's') => {
                self.strterm = self.new_strterm(str_types::str_ssym, term.unwrap(), paren, None);
                self.set_lex_state(EXPR_FNAME | EXPR_FITEM);
                return Self::tSYMBEG;
            }
            _ => {
                self.yyerror0("unknown type of %string");
                return Self::END_OF_INPUT;
            }
        }
    }

    pub(crate) fn parse_percent(&mut self, space_seen: bool, last_state: LexState) -> i32 {
        let c: MaybeByte;
        let ptok = self.buffer.pcur;

        if self.is_beg() {
            c = self.nextc();
            return self.percent_quatation(&c, ptok);
        }

        c = self.nextc();
        if c == '=' {
            self.set_yylval_id("%=");
            self.set_lex_state(EXPR_BEG);
            return Self::tOP_ASGN;
        }
        if self.is_spacearg(&c, space_seen) || (self.is_lex_state_some(EXPR_FITEM) && c == 's') {
            return self.percent_quatation(&c, ptok);
        }
        self.set_lex_state(if self.is_after_operator() {
            EXPR_ARG
        } else {
            EXPR_BEG
        });
        self.buffer.pushback(&c);
        return self.warn_balanced(
            Self::tPERCENT,
            "%%",
            "string literal",
            &c,
            space_seen,
            &last_state,
        );
    }

    pub(crate) fn parse_gvar(&mut self, last_state: LexState) -> i32 {
        let ptr = self.buffer.pcur;
        let mut c;

        self.set_lex_state(EXPR_END);
        self.buffer.ptok = ptr - 1; // from '$'
        self.newtok();
        c = self.nextc();
        match c.to_option() {
            Some(b'_') => { /* $_: last read line string */
                c = self.nextc();
                if self.parser_is_identchar() {
                    self.tokadd(b'$');
                    self.tokadd(b'_');
                } else {
                    self.buffer.pushback(&c);
                    c = MaybeByte::new('_');
                    self.tokadd(b'$');
                    self.tokadd(&c);
                    return Self::tGVAR;
                }
            },
            Some(b'~')          /* $~: match-data */
            | Some(b'*')        /* $*: argv */
            | Some(b'$')        /* $$: pid */
            | Some(b'?')        /* $?: last status */
            | Some(b'!')        /* $!: error string */
            | Some(b'@')        /* $@: error position */
            | Some(b'/')        /* $/: input record separator */
            | Some(b'\\')       /* $\: output record separator */
            | Some(b';')        /* $;: field separator */
            | Some(b',')        /* $,: output field separator */
            | Some(b'.')        /* $.: last read line number */
            | Some(b'=')        /* $=: ignorecase */
            | Some(b':')        /* $:: load path */
            | Some(b'<')        /* $<: reading filename */
            | Some(b'>')        /* $>: default output handle */
            | Some(b'\"') => {  /* $": already loaded files */
                self.tokadd(b'$');
                self.tokadd(&c);
                return Self::tGVAR;
            },
            Some(b'-') => {
                self.tokadd(b'$');
                self.tokadd(&c);
                c = self.nextc();
                if self.parser_is_identchar() {
                    if self.tokadd_mbchar(&c).is_err() { return Self::END_OF_INPUT }
                } else {
                    self.buffer.pushback(&c);
                    self.buffer.pushback(&'-');
                    return Self::tCHAR;
                }
                return Self::tGVAR;
            },
            Some(b'&')         /* $&: last match */
            | Some(b'`')       /* $`: string before last match */
            | Some(b'\'')      /* $': string after last match */
            | Some(b'+') => {  /* $+: string matches last paren. */
                if last_state.is_some(EXPR_FNAME) {
                    self.tokadd(b'$');
                    self.tokadd(&c);
                    return Self::tGVAR
                }
                return Self::tBACK_REF;
            },
            Some(b'1')
            | Some(b'2')
            | Some(b'3')
            | Some(b'4')
            | Some(b'5')
            | Some(b'6')
            | Some(b'7')
            | Some(b'8')
            | Some(b'9') => {
                self.tokadd(b'$');
                loop {
                    self.tokadd(&c);
                    c = self.nextc();

                    if c.is_eof() || !c.is_digit() {
                        break;
                    }
                }
                self.buffer.pushback(&c);
                if last_state.is_some(EXPR_FNAME) {
                    return Self::tGVAR
                }
                self.tokfix();
                return Self::tNTH_REF;
            }
            _ => {
                if !self.parser_is_identchar() {
                    if c.is_eof() || c.is_space() {
                        self.compile_error("`$' without identifiers is not allowed as a global variable name");
                    } else {
                        self.buffer.pushback(&c);
                        self.compile_error(&format!("`${}' is not allowed as a global variable name", c.unwrap()));
                    }
                    return Self::tGVAR
                }

                self.tokadd(b'$');
            }
        }

        if self.tokadd_ident(&c) {
            return Self::END_OF_INPUT;
        }
        self.set_lex_state(EXPR_END);
        self.tokenize_ident(&last_state);
        return Self::tGVAR;
    }

    pub(crate) fn parse_atmark(&mut self, last_state: LexState) -> i32 {
        let ptr = self.buffer.pcur;
        let mut result: i32 = Self::tIVAR;
        let mut c = self.nextc();

        self.buffer.ptok = ptr - 1; // from '@'
        self.newtok();
        self.tokadd(b'@');
        if c == '@' {
            result = Self::tCVAR;
            self.tokadd(b'@');
            c = self.nextc()
        }
        self.set_lex_state(if last_state.is_some(EXPR_FNAME) {
            EXPR_ENDFN
        } else {
            EXPR_END
        });
        if c.is_eof() || !self.parser_is_identchar() {
            self.buffer.pushback(&c);
            if result == Self::tIVAR {
                self.compile_error(
                    "`@' without identifiers is not allowed as an instance variable name",
                );
            } else {
                self.compile_error(
                    "`@@' without identifiers is not allowed as a class variable name",
                );
            }
            self.set_lex_state(EXPR_END);
            return result;
        } else if c.is_digit() {
            self.buffer.pushback(&c);
            if result == Self::tIVAR {
                self.compile_error(&format!(
                    "`@{}' is not allowed as an instance variable name",
                    c.unwrap()
                ));
            } else {
                self.compile_error(&format!(
                    "`@@{}' is not allowed as a class variable name",
                    c.unwrap()
                ));
            }
            self.set_lex_state(EXPR_END);
            return result;
        }

        if self.tokadd_ident(&c) {
            return Self::END_OF_INPUT;
        }
        self.tokenize_ident(&last_state);
        return result;
    }

    pub(crate) fn tokadd_ident(&mut self, c: &MaybeByte) -> bool {
        let mut c = c.clone();
        loop {
            if self.tokadd_mbchar(&c).is_err() {
                return true;
            }
            c = self.nextc();

            if !self.parser_is_identchar() {
                break;
            }
        }

        self.buffer.pushback(&c);
        return false;
    }

    pub(crate) fn newtok(&mut self) {
        self.buffer.tokidx = 0;
        self.buffer.tokline = self.buffer.ruby_sourceline;
        self.tokenbuf = TokenBuf::default();
    }

    pub(crate) fn is_identchar(&self, begin: usize, _end: usize) -> bool {
        self.buffer.input.bytes[begin].is_ascii_alphanumeric()
            || self.buffer.input.bytes[begin] == b'_'
            || !self.buffer.input.bytes[begin].is_ascii()
    }

    pub(crate) fn literal_flush(&mut self, ptok: usize) {
        self.buffer.set_ptok(ptok);
    }

    pub(crate) fn set_yylval_literal(&mut self, value: &TokenBuf) {
        if self.debug {
            println!(
                "set_yylval_literal({:#?}) ptok = {}, pcur = {}",
                value, self.buffer.ptok, self.buffer.pcur
            );
        }
        self.lval = Some(value.clone().to_token_value());
    }

    pub(crate) fn tokadd_mbchar(&mut self, c: &MaybeByte) -> Result<(), ()> {
        match c {
            MaybeByte::EndOfInput => Err(()),
            _ => {
                self.tokadd(c);
                Ok(())
            }
        }
    }

    pub(crate) fn parser_precise_mbclen(&mut self, _ptr: usize) -> Option<usize> {
        // FIXME: mbc = multibyte u8, so we need to do some byte work once we take String instead of String
        Some(1)
    }

    pub(crate) fn is_label_suffix(&mut self, n: usize) -> bool {
        self.buffer.peek_n(b':', n) && !self.buffer.peek_n(b':', n + 1)
    }

    pub(crate) fn set_yyval_name(&mut self) {
        if self.debug {
            println!("set_yyval_name({:#?})", self.tokenbuf);
        }
        self.lval = Some(self.tokenbuf.clone().to_token_value());
    }

    pub(crate) fn is_lvar_defined(&self, name: &str) -> bool {
        self.static_env.is_declared(name)
    }
}
