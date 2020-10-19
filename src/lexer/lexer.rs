use std::convert::TryInto;

use crate::lex_char::*;
use crate::lexer::*;
use crate::parser::TokenValue;
use crate::parser::{Loc, Token};
use crate::source::buffer::*;
use crate::source::InputError;
use crate::str_term::{str_types, StrTerm, StringLiteral};
use crate::Context;
use crate::StackState;
use crate::StaticEnvironment;
use crate::TokenBuf;
use crate::{lex_states::*, LexState};

#[derive(Debug, Clone, Default)]
pub struct Lexer {
    pub buffer: Buffer,
    pub debug: bool,

    pub lval: Option<TokenValue>,
    pub lval_start: Option<usize>,
    pub lval_end: Option<usize>,

    pub strterm: Option<StrTerm>,
    pub state: LexState,
    pub paren_nest: i32,
    pub lpar_beg: i32,
    pub brace_nest: i32,

    cond_stack: StackState,
    cmdarg_stack: StackState,

    pub tokenbuf: TokenBuf,

    // enc: Encoding,
    // token_info: TokenInfo,
    // case_labels: VALUE,
    // compile_option: VALUE,

    // debug_buffer: VALUE,
    // debug_output: VALUE,
    cur_arg: String,

    // ast: AST,
    node_id: usize,

    max_numparam: usize,

    pub context: Context,
    pub in_kwarg: bool,

    pub command_start: bool,
    pub has_shebang: bool,
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
    pub const NULL_CHAR: char = 0x00 as char;
    pub const CTRL_D_CHAR: char = 0x04 as char;
    pub const CTRL_Z_CHAR: char = 0x1a as char;
    pub const LF_CHAR: char = 0x0c as char;
    pub const VTAB_CHAR: char = 0x0b as char;

    pub fn new(bytes: &Vec<u8>, known_encoding: Option<String>) -> Result<Self, InputError> {
        Ok(Self {
            cond_stack: StackState::new("cond"),
            cmdarg_stack: StackState::new("cmdarg"),
            lpar_beg: -1, /* make lambda_beginning_p() == FALSE at first */
            buffer: Buffer::new("(eval)", bytes.to_owned(), known_encoding)?,
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
                (Self::END_OF_INPUT, _, _) => break,
                _ => tokens.push(token),
            }
        }

        tokens
    }

    pub fn yylex(&mut self) -> Token {
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
                self.buffer.substr_at(begin, end).map(|s| TokenValue::String(s.to_owned())))
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

        let token = (token_type, token_value, Loc { begin, end });
        if self.debug {
            println!(
                "yylex ({:?}, {:?}, {:?})",
                Self::token_name(&token),
                token.1,
                token.2
            );
        }
        token
    }

    pub fn nextc(&mut self) -> LexChar {
        self.buffer.nextc()
    }
    pub fn char_at(&self, idx: usize) -> LexChar {
        self.buffer.char_at(idx)
    }
    pub fn token_flush(&mut self) {
        self.buffer.token_flush()
    }

    pub fn token_name(token: &Token) -> String {
        let (id, _, _) = token;
        let first_token: usize = Self::YYerror.try_into().unwrap();
        let id_usize: usize = (*id).try_into().unwrap(); // minus first token ID
        if id_usize > first_token + 1 {
            Self::TOKEN_NAMES[id_usize - first_token + 1].to_owned()
        } else if *id == Self::END_OF_INPUT {
            "EOF".to_owned()
        } else {
            panic!(
                "token_name fails, {:?} (first token = {})",
                token, first_token
            )
        }
    }

    pub fn parser_yylex(&mut self) -> i32 {
        let mut c: LexChar;
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
                Some('\r') => {
                    if !self.buffer.cr_seen {
                        self.buffer.cr_seen = true;
                        self.warn("encountered \\r in middle of line, treated as a mere space");
                    }
                }

                Some(' ') | Some('\t') | Some(Self::LF_CHAR) | Some(Self::VTAB_CHAR) => {
                    space_seen = true;
                    continue 'retrying;
                }

                Some('#') | Some('\n') => {
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
                            Some(' ')
                            | Some('\t')
                            | Some(Self::LF_CHAR)
                            | Some('\r')
                            | Some(Self::VTAB_CHAR) => {
                                space_seen = true;
                            }
                            Some('#') => {
                                self.buffer.pushback(&c);
                                continue 'retrying;
                            }
                            Some('&') | Some('.') => {
                                if self.buffer.peek('.') == (c == '&') {
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

                Some('*') => {
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

                Some('!') => {
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

                Some('=') => {
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

                Some('<') => {
                    c = self.nextc();
                    if c == '<'
                        && !self.is_lex_state_some(EXPR_DOT | EXPR_CLASS)
                        && !self.is_end()
                        && (!self.is_arg() || self.is_lex_state_some(EXPR_LABELED) || space_seen)
                    {
                        if let Some(token) = self.heredoc_identifier() {
                            return token;
                        } else {
                            return Self::END_OF_INPUT;
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

                Some('>') => {
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

                Some('"') => {
                    label = if self.is_label_possible(cmd_state) {
                        str_types::str_label
                    } else {
                        0
                    };
                    self.strterm =
                        self.new_strterm(str_types::str_dquote | label, '"', None, None, None);
                    self.buffer.set_ptok(self.buffer.pcur - 1);
                    return Self::tSTRING_BEG;
                }

                Some('`') => {
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
                    self.strterm = self.new_strterm(str_types::str_xquote, '`', None, None, None);
                    return Self::tXSTRING_BEG;
                }

                Some('\'') => {
                    label = if self.is_label_possible(cmd_state) {
                        str_types::str_label
                    } else {
                        0
                    };
                    self.strterm =
                        self.new_strterm(str_types::str_squote | label, '\'', None, None, None);
                    self.buffer.set_ptok(self.buffer.pcur - 1);
                    return Self::tSTRING_BEG;
                }

                Some('?') => {
                    return self.parse_qmark(space_seen).unwrap_or(-1);
                }

                Some('&') => {
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

                Some('|') => {
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
                            self.buffer.pushback(&Some('|'));
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

                Some('+') => {
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
                        || (self.is_spacearg(&c, space_seen) && self.arg_ambiguous('+'))
                    {
                        self.set_lex_state(EXPR_BEG);
                        self.buffer.pushback(&c);
                        if !c.is_eof() && c.is_digit() {
                            return self.parse_numeric('+');
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

                Some('-') => {
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
                        || (self.is_spacearg(&c, space_seen) && self.arg_ambiguous('-'))
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

                Some('.') => {
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
                            self.buffer.char_at(self.buffer.pcur - 2)
                        } else {
                            LexChar::EOF
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

                Some('0') | Some('1') | Some('2') | Some('3') | Some('4') | Some('5')
                | Some('6') | Some('7') | Some('8') | Some('9') => {
                    return self.parse_numeric(c.unwrap().clone());
                }

                Some(')') => {
                    self.cond_pop();
                    self.cmdarg_pop();
                    self.set_lex_state(EXPR_ENDFN);
                    self.paren_nest -= 1;

                    return Self::tRPAREN;
                }

                Some(']') => {
                    self.cond_pop();
                    self.cmdarg_pop();
                    self.set_lex_state(EXPR_END);
                    self.paren_nest -= 1;

                    return Self::tRBRACK;
                }

                Some('}') => {
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

                Some(':') => {
                    c = self.nextc();
                    if c == ':' {
                        if self.is_beg()
                            || self.is_lex_state_some(EXPR_CLASS)
                            || self.is_spacearg(&LexChar::EOF, space_seen)
                        {
                            self.set_lex_state(EXPR_BEG);
                            return Self::tCOLON3;
                        }
                        self.set_yylval_id("::");
                        self.set_lex_state(EXPR_DOT);
                        return Self::tCOLON2;
                    }
                    if self.is_end() || c.is_space() || c == Some('#') {
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
                        Some('\'') => {
                            self.strterm =
                                self.new_strterm(str_types::str_ssym, c.unwrap(), None, None, None)
                        }
                        Some('"') => {
                            self.strterm =
                                self.new_strterm(str_types::str_dsym, c.unwrap(), None, None, None)
                        }
                        _ => self.buffer.pushback(&c),
                    }
                    self.set_lex_state(EXPR_FNAME);
                    return Self::tSYMBEG;
                }

                Some('/') => {
                    if self.is_beg() {
                        self.strterm =
                            self.new_strterm(str_types::str_regexp, '/', None, None, None);
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
                        self.arg_ambiguous('/');
                        self.strterm =
                            self.new_strterm(str_types::str_regexp, '/', None, None, None);
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

                Some('^') => {
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

                Some(';') => {
                    self.set_lex_state(EXPR_BEG);
                    self.command_start = true;
                    return Self::tSEMI;
                }

                Some(',') => {
                    self.set_lex_state(EXPR_BEG | EXPR_LABEL);
                    return Self::tCOMMA;
                }

                Some('~') => {
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

                Some('(') => {
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

                Some('[') => {
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

                Some('{') => {
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

                Some('\\') => {
                    c = self.nextc();
                    if c == '\n' {
                        space_seen = true;
                        continue 'retrying; /* skip \\n */
                    }
                    if c == ' ' {
                        return Self::tSP;
                    }
                    if c.is_space() {
                        panic!("unclear what to return for \\");
                    }
                    self.buffer.pushback(&c);
                    panic!("unclear what to return for \\ (2)");
                }

                Some('%') => {
                    return self.parse_percent(space_seen, last_state);
                }

                Some('$') => {
                    return self.parse_gvar(last_state);
                }

                Some('@') => {
                    return self.parse_atmark(last_state);
                }

                Some('_') => {
                    if self.buffer.was_bol() && self.buffer.is_whole_match("__END__", 0) {
                        self.buffer.eofp = true;
                        return Self::END_OF_INPUT;
                    }
                    self.newtok();
                }

                _ => {
                    if !self.parser_is_identchar() {
                        self.compile_error(&format!("Invalid char `{}' in expression", c.unwrap()));
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

    pub fn is_lex_state_some(&self, states: i32) -> bool {
        self.state.is_some(states)
    }

    pub fn is_lex_state_all(&self, states: i32) -> bool {
        self.state.is_all(states)
    }

    pub fn warn(&self, message: &str) {
        if self.debug {
            println!("WARNING: {}", message)
        }
    }

    pub fn set_yylval_id(&mut self, id: &str) {
        if self.debug {
            println!("set_yylval_id({})", id);
        }
        self.lval = Some(TokenValue::String(id.to_owned()));
    }

    pub fn is_spacearg(&self, c: &LexChar, space_seen: bool) -> bool {
        self.is_arg() && space_seen && !c.is_space()
    }

    pub fn is_beg(&self) -> bool {
        self.is_lex_state_some(EXPR_BEG_ANY) || self.is_lex_state_all(EXPR_ARG | EXPR_LABELED)
    }

    pub fn warn_balanced(
        &self,
        token_type: i32,
        op: &str,
        syn: &str,
        c: &LexChar,
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

    pub fn is_after_operator(&self) -> bool {
        self.is_lex_state_some(EXPR_FNAME | EXPR_DOT)
    }

    pub fn compile_error(&self, message: &str) {
        if self.debug {
            println!("Compile error: {}", message)
        }
    }

    pub fn is_end(&self) -> bool {
        self.is_lex_state_some(EXPR_END_ANY)
    }

    pub fn is_arg(&self) -> bool {
        self.is_lex_state_some(EXPR_ARG_ANY)
    }

    pub fn is_label_possible(&self, cmd_state: bool) -> bool {
        (self.is_lex_state_some(EXPR_LABEL | EXPR_ENDFN) && !cmd_state) || self.is_arg()
    }

    pub fn new_strterm(
        &self,
        func: usize,
        term: char,
        paren: Option<char>,
        heredoc_end: Option<usize>,
        heredoc_len: Option<usize>,
    ) -> Option<StrTerm> {
        Some(StrTerm::new_literal(StringLiteral::new(
            0,
            func,
            paren,
            term,
            heredoc_end,
            heredoc_len,
        )))
    }

    pub fn escaped_control_code(&self, c: &LexChar) -> Option<char> {
        if *c == ' ' {
            return Some('s');
        }
        if *c == '\n' {
            return Some('n');
        }
        if *c == '\t' {
            return Some('t');
        }
        if *c == 0x0b as char {
            return Some('v');
        }
        if *c == '\r' {
            return Some('r');
        }
        if *c == 0x0c as char {
            return Some('f');
        }
        None
    }

    pub fn parse_qmark_ternary(&mut self, c: &LexChar) -> Result<i32, ()> {
        self.buffer.pushback(c);
        self.set_lex_state(EXPR_VALUE);
        Ok(Self::tEH)
    }

    pub fn warn_space_char(&mut self, c: char, prefix: &str) {
        self.warn(&format!(
            "invalid character syntax; use \"{}\"\\{}",
            prefix, c
        ))
    }

    pub fn parse_qmark(&mut self, space_seen: bool) -> Result<i32, ()> {
        // let enc;
        let mut c;
        let lit;

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
            if self.buffer.peek('u') {
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
        lit = self.tok();
        self.set_yylval_str(lit);
        self.set_lex_state(EXPR_END);
        return Ok(Self::tCHAR);
    }

    pub fn arg_ambiguous(&self, c: char) -> bool {
        self.warn(&format!(
            "ambiguous first argument; put parentheses or a space even after `{}' operator",
            c
        ));
        true
    }

    // pub fn tokadd(&mut self, c: &LexChar) {
    //     let c = c.unwrap();
    //     self.tokenbuf.push(c);
    // }

    // pub fn tokadd_byte(&mut self, byte: u8) {
    //     unimplemented!()
    // }

    pub fn toklen(&self) -> usize {
        match &self.tokenbuf {
            TokenBuf::String(s) => s.len(),
            TokenBuf::Bytes(_) => unreachable!("toklen is supposed to be used with String"),
        }
    }

    pub fn tokfix(&self) {
        // nop
    }

    pub fn tok(&self) -> TokenBuf {
        self.tokenbuf.clone()
    }

    pub fn yyerror0(&self, message: &str) {
        if self.debug {
            println!("yyerror0: {}", message)
        }
        panic!("{}", message)
    }

    pub fn is_lambda_beginning(&self) -> bool {
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

    pub fn percent_unknown(&mut self, term: &LexChar) -> i32 {
        self.buffer.pushback(term);
        let len = self.parser_precise_mbclen(self.buffer.pcur);
        if let Some(len) = len {
            self.buffer.pcur += len;
            self.yyerror0("unknown type of %string");
        }
        return Self::END_OF_INPUT;
    }

    pub fn percent_quatation(&mut self, c: &LexChar, ptok: usize) -> i32 {
        let mut c = c.clone();
        let mut term: LexChar;
        let mut paren: Option<char>;

        if c.is_eof() || !c.is_alnum() {
            term = c.clone();
            if !c.is_ascii() {
                return self.percent_unknown(&term);
            }
            c = LexChar::new('Q');
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
            term = LexChar::new(')')
        } else if term == '[' {
            term = LexChar::new(']')
        } else if term == '{' {
            term = LexChar::new('}')
        } else if term == '<' {
            term = LexChar::new('>')
        } else {
            paren = None
        }

        self.buffer.ptok = ptok - 1;
        match c.to_option() {
            Some('Q') => {
                self.strterm =
                    self.new_strterm(str_types::str_dquote, term.unwrap(), paren, None, None);
                return Self::tSTRING_BEG;
            }
            Some('q') => {
                self.strterm =
                    self.new_strterm(str_types::str_squote, term.unwrap(), paren, None, None);
                return Self::tSTRING_BEG;
            }
            Some('W') => {
                self.strterm =
                    self.new_strterm(str_types::str_dword, term.unwrap(), paren, None, None);
                return Self::tWORDS_BEG;
            }
            Some('w') => {
                self.strterm =
                    self.new_strterm(str_types::str_sword, term.unwrap(), paren, None, None);
                return Self::tQWORDS_BEG;
            }
            Some('I') => {
                self.strterm =
                    self.new_strterm(str_types::str_dword, term.unwrap(), paren, None, None);
                return Self::tSYMBOLS_BEG;
            }
            Some('i') => {
                self.strterm =
                    self.new_strterm(str_types::str_sword, term.unwrap(), paren, None, None);
                return Self::tQSYMBOLS_BEG;
            }
            Some('x') => {
                self.strterm =
                    self.new_strterm(str_types::str_xquote, term.unwrap(), paren, None, None);
                return Self::tXSTRING_BEG;
            }
            Some('r') => {
                self.strterm =
                    self.new_strterm(str_types::str_regexp, term.unwrap(), paren, None, None);
                return Self::tREGEXP_BEG;
            }
            Some('s') => {
                self.strterm =
                    self.new_strterm(str_types::str_ssym, term.unwrap(), paren, None, None);
                self.set_lex_state(EXPR_FNAME | EXPR_FITEM);
                return Self::tSYMBEG;
            }
            _ => {
                self.yyerror0("unknown type of %string");
                return Self::END_OF_INPUT;
            }
        }
    }

    pub fn parse_percent(&mut self, space_seen: bool, last_state: LexState) -> i32 {
        let c: LexChar;
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

    pub fn parse_gvar(&mut self, last_state: LexState) -> i32 {
        let ptr = self.buffer.pcur;
        let mut c;

        self.set_lex_state(EXPR_END);
        self.buffer.ptok = ptr - 1; // from '$'
        self.newtok();
        c = self.nextc();
        match c.to_option() {
            Some('_') => { /* $_: last read line string */
                c = self.nextc();
                if self.parser_is_identchar() {
                    self.tokadd('$');
                    self.tokadd('_');
                } else {
                    self.buffer.pushback(&c);
                    c = LexChar::new('_');
                    self.tokadd('$');
                    self.tokadd(&c);
                    return Self::tGVAR;
                }
            },
            Some('~')          /* $~: match-data */
            | Some('*')        /* $*: argv */
            | Some('$')        /* $$: pid */
            | Some('?')        /* $?: last status */
            | Some('!')        /* $!: error string */
            | Some('@')        /* $@: error position */
            | Some('/')        /* $/: input record separator */
            | Some('\\')       /* $\: output record separator */
            | Some(';')        /* $;: field separator */
            | Some(',')        /* $,: output field separator */
            | Some('.')        /* $.: last read line number */
            | Some('=')        /* $=: ignorecase */
            | Some(':')        /* $:: load path */
            | Some('<')        /* $<: reading filename */
            | Some('>')        /* $>: default output handle */
            | Some('\"') => {  /* $": already loaded files */
                self.tokadd('$');
                self.tokadd(&c);
                return Self::tGVAR;
            },
            Some('-') => {
                self.tokadd('$');
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
            Some('&')         /* $&: last match */
            | Some('`')       /* $`: string before last match */
            | Some('\'')      /* $': string after last match */
            | Some('+') => {  /* $+: string matches last paren. */
                if last_state.is_some(EXPR_FNAME) {
                    self.tokadd('$');
                    self.tokadd(&c);
                    return Self::tGVAR
                }
                return Self::tBACK_REF;
            },
            Some('1')
            | Some('2')
            | Some('3')
            | Some('4')
            | Some('5')
            | Some('6')
            | Some('7')
            | Some('8')
            | Some('9') => {
                self.tokadd('$');
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

                self.tokadd('$');
            }
        }

        if self.tokadd_ident(&c) {
            return Self::END_OF_INPUT;
        }
        self.set_lex_state(EXPR_END);
        self.tokenize_ident(&last_state);
        return Self::tGVAR;
    }

    pub fn parse_atmark(&mut self, last_state: LexState) -> i32 {
        let ptr = self.buffer.pcur;
        let mut result: i32 = Self::tIVAR;
        let mut c = self.nextc();

        self.buffer.ptok = ptr - 1; // from '@'
        self.newtok();
        self.tokadd('@');
        if c == '@' {
            result = Self::tCVAR;
            self.tokadd('@');
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

    pub fn tokadd_ident(&mut self, c: &LexChar) -> bool {
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

    pub fn newtok(&mut self) {
        self.buffer.tokidx = 0;
        self.buffer.tokline = self.buffer.ruby_sourceline;
        self.tokenbuf = TokenBuf::default();
    }

    pub fn is_identchar(&self, begin: usize, _end: usize) -> bool {
        self.buffer.input[begin].is_ascii_alphanumeric()
            || self.buffer.input[begin] == '_'
            || !self.buffer.input[begin].is_ascii()
    }

    pub fn literal_flush(&mut self, ptok: usize) {
        self.buffer.set_ptok(ptok);
    }

    pub fn set_yylval_literal(&mut self, value: TokenBuf) {
        if self.debug {
            println!(
                "set_yylval_literal({:#?}) ptok = {}, pcur = {}",
                value, self.buffer.ptok, self.buffer.pcur
            );
        }
        self.lval = Some(value.to_token_value());
    }

    pub fn tokadd_mbchar(&mut self, c: &LexChar) -> Result<(), ()> {
        match c {
            LexChar::EOF => Err(()),
            _ => {
                self.tokadd(c);
                Ok(())
            }
        }
    }

    pub fn parser_precise_mbclen(&mut self, _ptr: usize) -> Option<usize> {
        // FIXME: mbc = multibyte char, so we need to do some byte work once we take String instead of String
        Some(1)
    }

    pub fn is_label_suffix(&mut self, n: usize) -> bool {
        self.buffer.peek_n(':', n) && !self.buffer.peek_n(':', n + 1)
    }

    pub fn set_yyval_name(&mut self, name: TokenBuf) {
        if self.debug {
            println!("set_yyval_name({:#?})", name);
        }
        self.lval = Some(name.to_token_value());
    }

    pub fn is_lvar_defined(&self, name: &str) -> bool {
        self.static_env.is_declared(name)
    }
}
