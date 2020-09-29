use std::convert::TryFrom;

use crate::lexer::{StrTerm, StringLiteral, HeredocLiteral, str_types};
use crate::lexer::lex_state::{lex_states, LexState};
use lex_states::*;
use crate::lexer::LocalsTable;
use crate::lexer::{Context};
use crate::parser::{Token, Loc};
// use crate::lexer::{Token, TokenType};
use crate::lexer::lex_char::LexChar;
use crate::StaticEnvironment;
use crate::lexer::StackState;

#[derive(Debug, Clone, Default)]
struct SourceLine {
    start: usize,
    end: usize,
}

impl SourceLine {
    #[allow(dead_code)]
    fn source(&self, source: &Vec<char>) -> String {
        let chars = &source[self.start..self.end].to_owned();
        return chars.iter().collect()
    }

    fn len(&self) -> usize {
        self.end - self.start
    }

    fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

#[derive(Debug, Clone, Default)]
pub struct LexerState {
    pub strterm: Option<StrTerm>,
    input: Vec<char>,
    lines: Vec<SourceLine>,
    prevline_idx: Option<usize>,
    lastline_idx: Option<usize>,
    lastline_start: usize,
    nextline_idx: Option<usize>,
    pbeg: usize,
    pub pcur: usize,
    pub pend: usize,
    pub ptok: usize,
    pub state: LexState,
    paren_nest: i32,
    pub lpar_beg: i32,
    brace_nest: i32,
}

#[derive(Debug, Clone, Default)]
pub struct ParserState {
    pub lval: Option<String>,
    pub lex: LexerState,
    cond_stack: StackState,
    cmdarg_stack: StackState,
    tokidx: usize,
    toksize: usize,
    tokline: usize,
    heredoc_end: usize,
    pub heredoc_indent: i32,
    heredoc_line_indent: usize,
    tokenbuf: String,
    lvtbl: LocalsTable,
    pvtbl: std::collections::HashMap<String, String>,
    pktbl: std::collections::HashMap<String, String>,
    line_count: usize,
    ruby_sourceline: usize,	/* current line no. */
    ruby_sourcefile: String, /* current source file */
    ruby_sourcefile_string: String,
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
    pub eofp: bool,
    ruby__end__seen: bool,
    debug: usize,
    has_shebang: bool,
    token_seen: bool,
    token_info_enabled: bool,

    error_p: usize,
    cr_seen: bool,

    do_print: usize,
    do_loop: usize,
    do_chomp: usize,
    do_split: usize,

    pub static_env: StaticEnvironment,

    // NODE *eval_tree_begin;
    // NODE *eval_tree;
    // VALUE error_buffer;
    // VALUE debug_lines;
    // const struct rb_iseq_struct *parent_iseq;
}

#[derive(Debug, Clone, Default)]
pub struct Lexer {
    pub debug: bool,
    pub p: ParserState,
}

const NULL_CHAR  : char = 0x00 as char;
const CTRL_D_CHAR: char = 0x04 as char;
const CTRL_Z_CHAR: char = 0x1a as char;
const LF_CHAR    : char = 0x0c as char;
const VTAB_CHAR  : char = 0x0b as char;

impl Lexer {
    pub fn new(source: &str) -> Self {
        let mut result = Lexer::default();
        result.p.cond_stack = StackState::new("cond");
        result.p.cmdarg_stack = StackState::new("cmdarg");
        result.p.lex.lpar_beg = -1; /* make lambda_beginning_p() == FALSE at first */
        result.set_source(source);
        result
    }

    pub fn set_source(&mut self, source: &str) {
        let chars: Vec<char> = source.chars().collect();
        let mut line = SourceLine { start: 0, end: 0 };
        let mut lines: Vec<SourceLine> = vec![];

        for (idx, c) in chars.iter().enumerate() {
            line.end = idx + 1;
            if *c == '\n' {
                lines.push(line);
                line = SourceLine { start: idx + 1, end: 0 }
            }
        };
        line.end = chars.len();
        if !line.is_empty() {
            lines.push(line);
        }
        if self.debug { println!("lines = {:#?}", lines); }

        self.p.lex.input = chars;
        self.p.lex.lines = lines;
    }

    pub fn tokenize_until_eof(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        loop {
            let token = self.yylex();
            match token {
                (Self::END_OF_INPUT, _, _) => break,
                _ => tokens.push(token)
            }
        }

        tokens
    }

    pub fn yylex(&mut self) -> Token {
        self.p.lval = None;
        // println!("before yylex: {:#?}", self);

        let token_type = self.parser_yylex();

        let begin = self.p.lex.ptok;
        let mut end = self.p.lex.pcur;
        let mut token_value = self.p.lval.clone().unwrap_or_else(||
            // take raw value if nothing was manually captured
            self.p.lex.input[begin..end].iter().collect()
        );

        match self.p.lex.strterm {
            Some(StrTerm::Heredoc(_)) => {
                // RUBY_SET_YYLLOC_FROM_STRTERM_HEREDOC
            },
            _ => {
                // RUBY_SET_YYLLOC
            }
        };

        if token_type == Self::tNL {
            token_value = "".to_owned();
            end = begin + 1;
        }

        let token = (token_type, token_value, Loc { begin, end });
        if self.debug { println!("yylex {:?}", token); }
        token
    }

    pub fn parser_yylex(&mut self) -> i32 {
        let mut c: LexChar;
        let mut space_seen: bool = false;
        let cmd_state: bool;
        let label: usize;
        let mut last_state: LexState;
        let token_seen = self.p.token_seen;

        if let Some(strterm) = self.p.lex.strterm.clone() {
            match strterm {
                StrTerm::Heredoc(mut heredoc) => {
                    return self.here_document(&mut heredoc)
                },

                StrTerm::Literal(mut string) => {
                    self.token_flush();
                    return self.parse_string(&mut string)
                }
            }
        }

        cmd_state = self.p.command_start;
        self.p.command_start = false;
        self.p.token_seen = true;

        'retrying: loop {
            last_state = self.p.lex.state.clone();
            self.token_flush();

            // handle EOF
            c = self.nextc();

            if c.is_eof() {
                return Self::END_OF_INPUT
            }

            match c {
                LexChar::EOF |
                LexChar::Some(NULL_CHAR) |
                LexChar::Some(CTRL_D_CHAR) |
                LexChar::Some(CTRL_Z_CHAR) => { return Self::END_OF_INPUT },

                // whitespaces
                LexChar::Some('\r') => {
                    if !self.p.cr_seen {
                        self.p.cr_seen = true;
                        self.warn("encountered \\r in middle of line, treated as a mere space");
                    }
                },

                LexChar::Some(' ') |
                LexChar::Some('\t') |
                LexChar::Some(LF_CHAR) |
                LexChar::Some(VTAB_CHAR) => {
                    space_seen = true;
                    continue 'retrying;
                },

                LexChar::Some('#') => { // it's a comment
                    self.p.token_seen = token_seen;
                    // no magic_comment in shebang line
                    if !self.parser_magic_comment() {
                        if self.comment_at_top() {
                            self.set_file_encoding()
                        }
                    }
                    self.lex_goto_eol();
                },

                LexChar::Some('\n') => {
                    self.p.token_seen = token_seen;
                    let cc = self.is_lex_state_some( EXPR_BEG|EXPR_CLASS|EXPR_FNAME|EXPR_DOT) && !self.is_lex_state_some(EXPR_LABELED);
                    if cc || self.is_lex_state_all(EXPR_ARG|EXPR_LABELED) {
                        if !cc && self.p.in_kwarg {
                            self.p.command_start = true;
                            self.set_lex_state(EXPR_BEG);
                            return Self::tNL;
                        }
                        continue 'retrying;
                    }

                    loop {
                        c = self.nextc();

                        match c {
                            LexChar::Some(' ') |
                            LexChar::Some('\t') |
                            LexChar::Some(LF_CHAR) |
                            LexChar::Some('\r') |
                            LexChar::Some(VTAB_CHAR) => {
                                space_seen = true;
                            },
                            LexChar::Some('#') => {
                                self.pushback(&c);
                                continue 'retrying;
                            },
                            LexChar::Some('&') | LexChar::Some('.') => {
                                if self.peek('.') == (c == '&') {
                                    self.pushback(&c);
                                    continue 'retrying;
                                }
                                self.p.ruby_sourceline -= 1;
                                self.p.lex.nextline_idx = self.p.lex.lastline_idx;
                            },
                            LexChar::EOF => {
                                // EOF no decrement
                                if self.p.lex.prevline_idx.is_some() && !self.p.eofp {
                                    self.p.lex.lastline_idx = self.p.lex.prevline_idx.clone();
                                }

                                self.p.lex.pbeg = self.p.lex.lastline_start;
                                self.p.lex.pend = self.p.lex.pbeg + self.p.lex.lines[self.p.lex.lastline_idx.unwrap()].len();
                                self.p.lex.pcur = self.p.lex.pend;
                                self.pushback(&LexChar::Some(1 as char));
                                self.set_ptok(self.p.lex.pcur);

                                self.p.command_start = true;
                                self.set_lex_state(EXPR_BEG);
                                return Self::tNL;
                            },
                            _ => {
                                self.p.ruby_sourceline -= 1;
                                self.p.lex.nextline_idx = self.p.lex.lastline_idx;
                                if self.p.lex.prevline_idx.is_some() && !self.p.eofp {
                                    self.p.lex.lastline_idx = self.p.lex.prevline_idx.clone();
                                }

                                self.p.lex.pbeg = self.p.lex.lastline_start;
                                self.p.lex.pend = self.p.lex.pbeg + self.p.lex.lines[self.p.lex.lastline_idx.unwrap()].len();
                                self.p.lex.pcur = self.p.lex.pend;
                                self.pushback(&LexChar::Some(1 as char));
                                self.set_ptok(self.p.lex.pcur);

                                self.p.command_start = true;
                                self.set_lex_state(EXPR_BEG);
                                return Self::tNL;
                            },
                        }
                    }
                },

                LexChar::Some('*') => {
                    let result: i32;

                    c = self.nextc();

                    if c == '*' {
                        c = self.nextc();
                        if c == '=' {
                            self.set_yylval_id("**");
                            self.set_lex_state(EXPR_BEG);
                            return Self::tOP_ASGN;
                        }
                        self.pushback(&c);
                        if self.is_spacearg(&c, space_seen) {
                            self.warn("`**' interpreted as argument prefix");
                            result = Self::tDSTAR;
                        } else if self.is_beg() {
                            result = Self::tDSTAR;
                        } else {
                            result = self.warn_balanced(Self::tPOW, "**", "argument prefix", &c, space_seen, &last_state);
                        }
                    } else {
                        if c == '=' {
                            self.set_yylval_id("*");
                            self.set_lex_state(EXPR_BEG);
                            return Self::tOP_ASGN;
                        }
                        self.pushback(&c);
                        if self.is_spacearg(&c, space_seen) {
                            self.warn("`*' interpreted as argument prefix");
                            result = Self::tSTAR;
                        } else if self.is_beg() {
                            result = Self::tSTAR;
                        } else {
                            result = self.warn_balanced(Self::tSTAR2, "*", "argument prefix", &c, space_seen, &last_state);
                        }
                    }

                    self.set_lex_state(if self.is_after_operator() { EXPR_ARG } else { EXPR_BEG });
                    return result;
                },

                LexChar::Some('!') => {
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
                    self.pushback(&c);
                    return Self::tBANG;
                },

                LexChar::Some('=') => {
                    if self.was_bol() {
                        // skip embedded rd document
                        if self.is_word_match("begin") {
                            self.lex_goto_eol();
                            loop {
                                self.lex_goto_eol();
                                c = self.nextc();
                                if c.is_eof() {
                                    self.compile_error("embedded document meets end of file");
                                    return Self::END_OF_INPUT;
                                }
                                if c == '=' && self.is_word_match("end") {
                                    break;
                                }
                                self.pushback(&c);
                            }
                            self.lex_goto_eol();
                            continue 'retrying;
                        }
                    }

                    self.set_lex_state(if self.is_after_operator() { EXPR_ARG } else { EXPR_BEG });
                    c = self.nextc();
                    if c == '=' {
                        c = self.nextc();
                        if c == '=' {
                            return Self::tEQQ;
                        }
                        self.pushback(&c);
                        return Self::tEQ;
                    }
                    if c == '~' {
                        return Self::tMATCH;
                    } else if c == '>' {
                        return Self::tASSOC;
                    }
                    self.pushback(&c);
                    return Self::tEQL;
                },

                LexChar::Some('<') => {
                    c = self.nextc();
                    if c == '<' &&
                        !self.is_lex_state_some(EXPR_DOT|EXPR_CLASS) &&
                        !self.is_end() &&
                        (!self.is_arg() || self.is_lex_state_some(EXPR_LABELED) || space_seen) {
                            return self.heredoc_identifier();
                    }
                    if self.is_after_operator() {
                        self.set_lex_state(EXPR_ARG);
                    } else {
                        if self.is_lex_state_some(EXPR_CLASS) {
                            self.p.command_start = true;
                        }
                        self.set_lex_state(EXPR_BEG);
                    }
                    if c == '=' {
                        c = self.nextc();
                        if c == '>' {
                            return Self::tCMP;
                        }
                        self.pushback(&c);
                        return Self::tLEQ;
                    }
                    if c == '<' {
                        c = self.nextc();
                        if c == '=' {
                            self.set_yylval_id("<<");
                            self.set_lex_state(EXPR_BEG);
                            return Self::tOP_ASGN;
                        }
                        self.pushback(&c);
                        return self.warn_balanced(Self::tLSHFT, "<<", "here document", &c, space_seen, &last_state);
                    }
                    self.pushback(&c);
                    return Self::tLT
                },

                LexChar::Some('>') => {
                    self.set_lex_state(if self.is_after_operator() { EXPR_ARG } else { EXPR_BEG });

                    c = self.nextc();
                    if c == '=' {
                        return Self::tGEQ;
                    }

                    if c == '>' {
                        c = self.nextc();
                        if c == '=' {
                            self.set_yylval_id(">>");
                            self.set_lex_state(EXPR_BEG);
                            return Self::tOP_ASGN;
                        }
                        self.pushback(&c);
                        return Self::tRSHFT;
                    }
                    self.pushback(&c);
                    return Self::tGT;
                },

                LexChar::Some('"') => {
                    label = if self.is_label_possible(cmd_state) { str_types::str_label } else { 0 };
                    self.p.lex.strterm = self.new_strterm(str_types::str_dquote | label, '"', None);
                    self.set_ptok(self.p.lex.pcur - 1);
                    return Self::tSTRING_BEG;
                },

                LexChar::Some('`') => {
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
                    self.p.lex.strterm = self.new_strterm(str_types::str_xquote, '`', None);
                    return Self::tXSTRING_BEG;
                },

                LexChar::Some('\'') => {
                    label = if self.is_label_possible(cmd_state) { str_types::str_label } else { 0 };
                    self.p.lex.strterm = self.new_strterm(str_types::str_squote | label, '\'', None);
                    self.set_ptok(self.p.lex.pcur - 1);
                    return Self::tSTRING_BEG;
                },

                LexChar::Some('?') => {
                    return self.parse_qmark(space_seen);
                },

                LexChar::Some('&') => {
                    let result: i32;

                    c = self.nextc();
                    if c == '&' {
                        self.set_lex_state(EXPR_BEG);
                        c = self.nextc();
                        if c == '=' {
                            self.set_yylval_id("&&");
                            self.set_lex_state(EXPR_BEG);
                            return Self::tOP_ASGN;
                        }
                        self.pushback(&c);
                        return Self::tANDOP;
                    } else if c == '=' {
                        self.set_yylval_id("&");
                        self.set_lex_state(EXPR_BEG);
                        return Self::tOP_ASGN;
                    } else if c == '.' {
                        self.set_yylval_id("&.");
                        self.set_lex_state(EXPR_DOT);
                        return Self::tANDDOT;
                    }
                    self.pushback(&c);
                    if self.is_spacearg(&c, space_seen) {
                        // TODO: check for some warnings here
                        result = Self::tAMPER;
                    } else if self.is_beg() {
                        result = Self::tAMPER;
                    } else {
                        result = self.warn_balanced(Self::tAMPER2, "&", "argument prefix", &c, space_seen, &last_state);
                    }
                    self.set_lex_state(if self.is_after_operator() { EXPR_ARG } else { EXPR_BEG });
                    return result;
                },

                LexChar::Some('|') => {
                    c = self.nextc();
                    if c == '|' {
                        self.set_lex_state(EXPR_BEG);
                        c = self.nextc();
                        if c == '=' {
                            self.set_yylval_id("||");
                            self.set_lex_state(EXPR_BEG);
                            return Self::tOP_ASGN;
                        }
                        self.pushback(&c);
                        if last_state.is_some(EXPR_BEG) {
                            self.pushback(&LexChar::Some('|'));
                            return Self::tPIPE;
                        }
                        return Self::tOROP;
                    }
                    if c == '=' {
                        self.set_yylval_id("|");
                        self.set_lex_state(EXPR_BEG);
                        return Self::tOP_ASGN;
                    }
                    self.set_lex_state(if self.is_after_operator() { EXPR_ARG } else { EXPR_BEG|EXPR_LABEL });
                    return Self::tPIPE;
                },

                LexChar::Some('+') => {
                    c = self.nextc();
                    if self.is_after_operator() {
                        self.set_lex_state(EXPR_ARG);
                        if c == '@' {
                            return Self::tUPLUS;
                        }
                        self.pushback(&c);
                        return Self::tPLUS;
                    }
                    if c == '=' {
                        self.set_yylval_id("+");
                        self.set_lex_state(EXPR_BEG);
                        return Self::tOP_ASGN;
                    }
                    if self.is_beg() || (self.is_spacearg(&c, space_seen) && self.arg_ambiguous('+')) {
                        self.set_lex_state(EXPR_BEG);
                        self.pushback(&c);
                        if !c.is_eof() && c.is_digit() {
                            return self.parse_numeric('+');
                        }
                        return Self::tUPLUS;
                    }
                    self.set_lex_state(EXPR_BEG);
                    self.pushback(&c);
                    return self.warn_balanced(Self::tPLUS, "+", "unary operator", &c, space_seen, &last_state);
                },

                LexChar::Some('-') => {
                    c = self.nextc();
                    if self.is_after_operator() {
                        self.set_lex_state(EXPR_ARG);
                        if c == '@' {
                            return Self::tUMINUS;
                        }
                        self.pushback(&c);
                        return Self::tMINUS;
                    }
                    if c == '=' {
                        self.set_yylval_id("-");
                        self.set_lex_state(EXPR_BEG);
                        return Self::tOP_ASGN;
                    }
                    if c == '>' {
                        self.set_lex_state(EXPR_ENDFN);
                        return Self::tLAMBDA;
                    }
                    if self.is_beg() || (self.is_spacearg(&c, space_seen) && self.arg_ambiguous('-')) {
                        self.set_lex_state(EXPR_BEG);
                        self.pushback(&c);
                        if !c.is_eof() && c.is_digit() {
                            return Self::tUMINUS_NUM;
                        }
                        return Self::tUMINUS;
                    }
                    self.set_lex_state(EXPR_BEG);
                    self.pushback(&c);
                    return self.warn_balanced(Self::tMINUS, "-", "unary operator", &c, space_seen, &last_state);
                },

                LexChar::Some('.') => {
                    let is_beg = self.is_beg();
                    self.set_lex_state(EXPR_BEG);
                    c = self.nextc();
                    if c == '.' {
                        c = self.nextc();
                        if c == '.' {
                            if self.p.lex.paren_nest == 0 && self.is_looking_at_eol() {
                                self.warn("... at EOL, should be parenthesized?");
                            } else if self.p.lex.lpar_beg >= 0 && self.p.lex.lpar_beg + 1 == self.p.lex.paren_nest {
                                if last_state.is_some(EXPR_LABEL) {
                                    return Self::tDOT3
                                }
                            }
                            return if is_beg { Self::tBDOT3 } else { Self::tDOT3 };
                        }
                        self.pushback(&c);
                        return if is_beg { Self::tBDOT2 } else { Self::tDOT2 };
                    }
                    self.pushback(&c);
                    if !c.is_eof() && c.is_digit() {
                        let prev =
                            if self.p.lex.pcur - 1 > self.p.lex.pbeg {
                                LexChar::Some(self.p.lex.input[self.p.lex.pcur - 2])
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
                        self.set_ptok(self.p.lex.pcur);
                        continue 'retrying;
                    }
                    self.set_yylval_id(".");
                    self.set_lex_state(EXPR_DOT);
                    return Self::tDOT;
                },

                LexChar::Some('0') |
                LexChar::Some('1') |
                LexChar::Some('2') |
                LexChar::Some('3') |
                LexChar::Some('4') |
                LexChar::Some('5') |
                LexChar::Some('6') |
                LexChar::Some('7') |
                LexChar::Some('8') |
                LexChar::Some('9')  => {
                    return self.parse_numeric(c.unwrap().clone());
                },

                LexChar::Some(')') => {
                    self.cond_pop();
                    self.cmdarg_pop();
                    self.set_lex_state(EXPR_ENDFN);
                    self.p.lex.paren_nest -= 1;

                    return Self::tRPAREN;
                },

                LexChar::Some(']') => {
                    self.cond_pop();
                    self.cmdarg_pop();
                    self.set_lex_state(EXPR_END);
                    self.p.lex.paren_nest -= 1;

                    return Self::tRBRACK;
                },

                LexChar::Some('}') => {
                    // tSTRING_DEND does COND_POP and CMDARG_POP in the yacc's rule (lalrpop here)
                    if self.p.lex.brace_nest == 0 {
                        self.p.lex.brace_nest -= 1;
                        return Self::tSTRING_DEND;
                    }
                    self.p.lex.brace_nest -= 1;
                    self.cond_pop();
                    self.cmdarg_pop();
                    self.set_lex_state(EXPR_END);
                    self.p.lex.paren_nest -= 1;

                    return Self::tRCURLY;
                },

                LexChar::Some(':') => {
                    c = self.nextc();
                    if c == ':' {
                        if self.is_beg() || self.is_lex_state_some(EXPR_CLASS) || self.is_spacearg(&LexChar::EOF, space_seen) {
                            self.set_lex_state(EXPR_BEG);
                            return Self::tCOLON3;
                        }
                        self.set_yylval_id("::");
                        self.set_lex_state(EXPR_DOT);
                        return Self::tCOLON2;
                    }
                    if self.is_end() || c.is_space() || c == LexChar::Some('#') {
                        self.pushback(&c);
                        let result = self.warn_balanced(Self::tCOLON, ":", "symbol literal", &c, space_seen, &last_state);
                        self.set_lex_state(EXPR_BEG);
                        return result;
                    }
                    match c {
                        LexChar::Some('\'') => self.p.lex.strterm = self.new_strterm(str_types::str_ssym, c.unwrap(), None),
                        LexChar::Some('"')  => self.p.lex.strterm = self.new_strterm(str_types::str_dsym, c.unwrap(), None),
                        _ => self.pushback(&c)
                    }
                    self.set_lex_state(EXPR_FNAME);
                    return Self::tSYMBEG;
                },

                LexChar::Some('/') => {
                    if self.is_beg() {
                        self.p.lex.strterm = self.new_strterm(str_types::str_regexp, '/', None);
                        return Self::tREGEXP_BEG;
                    }
                    c = self.nextc();
                    if c == '=' {
                        self.set_yylval_id("/");
                        self.set_lex_state(EXPR_BEG);
                        return Self::tOP_ASGN;
                    }
                    self.pushback(&c);
                    if self.is_spacearg(&c, space_seen) {
                        self.arg_ambiguous('/');
                        self.p.lex.strterm = self.new_strterm(str_types::str_regexp, '/', None);
                        return Self::tREGEXP_END;
                    }
                    self.set_lex_state(if self.is_after_operator() { EXPR_ARG } else { EXPR_END });
                    return self.warn_balanced(Self::tDIVIDE, "/", "regexp literal", &c, space_seen, &last_state);
                },

                LexChar::Some('^') => {
                    c = self.nextc();
                    if c == '=' {
                        self.set_yylval_id("^");
                        self.set_lex_state(EXPR_BEG);
                        return Self::tOP_ASGN;
                    }
                    self.set_lex_state(if self.is_after_operator() { EXPR_ARG } else { EXPR_BEG });
                    self.pushback(&c);
                    return Self::tCARET;
                }

                LexChar::Some(';') => {
                    self.set_lex_state(EXPR_BEG);
                    self.p.command_start = true;
                    return Self::tSEMI;
                },

                LexChar::Some(',') => {
                    self.set_lex_state(EXPR_BEG|EXPR_LABEL);
                    return Self::tCOMMA;
                },

                LexChar::Some('~') => {
                    if self.is_after_operator() {
                        c = self.nextc();
                        if c != '@' {
                            self.pushback(&c);
                        }
                        self.set_lex_state(EXPR_ARG);
                    } else {
                        self.set_lex_state(EXPR_BEG);
                    }

                    return Self::tTILDE;
                },

                LexChar::Some('(') => {
                    let mut result: i32 = Self::tLPAREN2;

                    if self.is_beg() {
                        result = Self::tLPAREN;
                    } else if !space_seen {
                        // foo( ... ) => method call, no ambiguity
                    } else if self.is_arg() || self.is_lex_state_all(EXPR_END|EXPR_LABEL) {
                        result = Self::tLPAREN_ARG;
                    } else if self.is_lex_state_some(EXPR_ENDFN) && !self.is_lambda_beginning() {
                        self.warn("parentheses after method name is interpreted as an argument list, not a decomposed argument");
                    }

                    self.p.lex.paren_nest += 1;
                    self.cond_push(false);
                    self.cmdarg_push(false);
                    self.set_lex_state(EXPR_BEG|EXPR_LABEL);

                    return result;
                },

                LexChar::Some('[') => {
                    let mut result: i32 = Self::tLBRACK2;

                    self.p.lex.paren_nest += 1;
                    if self.is_after_operator() {
                        c = self.nextc();
                        if c == ']' {
                            self.set_lex_state(EXPR_ARG);
                            c = self.nextc();
                            if c == '=' {
                                return Self::tASET;
                            }
                            self.pushback(&c);
                            return Self::tAREF;
                        }
                        self.pushback(&c);
                        self.set_lex_state(EXPR_ARG|EXPR_LABEL);
                        return Self::tLBRACK2;
                    } else if self.is_beg() {
                        result = Self::tLBRACK;
                    } else if self.is_arg() && (space_seen || self.is_lex_state_some(EXPR_LABELED)) {
                        result = Self::tLBRACK;
                    }
                    self.set_lex_state(EXPR_BEG|EXPR_LABEL);
                    self.cond_push(false);
                    self.cmdarg_push(false);
                    return result;
                },

                LexChar::Some('{') => {
                    self.p.lex.brace_nest += 1;

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
                        self.p.command_start = true;
                        self.set_lex_state(EXPR_BEG);
                    } else {
                        self.set_lex_state(EXPR_BEG|EXPR_LABEL);
                    }

                    self.p.lex.paren_nest += 1;
                    self.cond_push(false);
                    self.cmdarg_push(false);
                    return result;
                },

                LexChar::Some('\\') => {
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
                    self.pushback(&c);
                    panic!("unclear what to return for \\ (2)");
                },

                LexChar::Some('%') => {
                    return self.parse_percent(space_seen, last_state);
                },

                LexChar::Some('$') => {
                    return self.parse_gvar(last_state);
                },

                LexChar::Some('@') => {
                    return self.parse_atmark(last_state);
                },

                LexChar::Some('_') => {
                    if self.was_bol() && self.is_whole_match("__END__", 0) {
                        self.p.ruby__end__seen = true;
                        self.p.eofp = true;
                        return Self::END_OF_INPUT;
                    }
                    self.newtok();
                },

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

    pub fn set_ptok(&mut self, ptok: usize) {
        if self.debug { println!("set_ptok({})", ptok); }
        self.p.lex.ptok = ptok;
    }

    pub fn set_lex_state(&mut self, states: usize) {
        self.p.lex.state.set(states)
    }

    pub fn is_lex_state_some(&self, states: usize) -> bool {
        self.p.lex.state.is_some(states)
    }

    pub fn is_lex_state_all(&self, states: usize) -> bool {
        self.p.lex.state.is_all(states)
    }

    pub fn here_document(&self, _heredoc: &HeredocLiteral) -> i32 { unimplemented!("here_document") }

    pub fn token_flush(&mut self) {
        self.set_ptok(self.p.lex.pcur);
    }

    pub fn lex_getline(&mut self) -> Option<usize> {
        if self.p.line_count < self.p.lex.lines.len() {
            self.p.line_count += 1;
            Some(self.p.line_count - 1)
        } else {
            None
        }
    }

    pub fn nextline(&mut self) -> Result<(), ()> {
        let mut v = self.p.lex.nextline_idx;
        self.p.lex.nextline_idx = None;

        if v.is_none() {
            if self.p.eofp {
                return Err(());
            }

            if self.p.lex.pend > self.p.lex.pbeg && self.p.lex.input[self.p.lex.pend - 1] != '\n' {
                self.p.eofp = true;
                self.lex_goto_eol();
                return Err(());
            }

            v = self.lex_getline();
            if v.is_none() {
                self.p.eofp = true;
                self.lex_goto_eol();
                return Err(());
            }

            self.p.cr_seen = false;
        }
        // TODO: after here-document without terminator

        let v = v.unwrap();
        let line = &self.p.lex.lines[v];

        if self.p.heredoc_end > 0 {
            self.p.ruby_sourceline = self.p.heredoc_end;
            self.p.heredoc_end = 0;
        }
        self.p.ruby_sourceline += 1;
        self.p.lex.pbeg = line.start;
        self.p.lex.pcur = line.start;
        self.p.lex.pend = line.end;
        self.token_flush();
        self.p.lex.prevline_idx = self.p.lex.lastline_idx;
        self.p.lex.lastline_idx = Some(v);


        Ok(())
    }

    pub fn parser_cr(&mut self, _c: char) -> char {
        unimplemented!("parser_cr")
    }

    pub fn nextc(&mut self) -> LexChar {
        if self.p.lex.pcur == self.p.lex.pend || self.p.eofp || self.p.lex.nextline_idx.is_some() {
            let n = self.nextline();
            if self.debug { println!("nextline = {:?}", n); }
            if n.is_err() {
                return LexChar::EOF;
            }
        }
        let mut c: char = self.p.lex.input[self.p.lex.pcur];
        self.p.lex.pcur += 1;
        if c == '\r' {
            c = self.parser_cr(c);
        }
        if self.debug { println!("nextc = {:?}", c); }
        return LexChar::Some(c);
    }

    pub fn char_at(&self, idx: usize) -> LexChar {
        if let Some(c) = self.p.lex.input.get(idx) {
            LexChar::Some(c.clone())
        } else {
            LexChar::EOF
        }
    }

    pub fn substr_at(&self, start: usize, end: usize) -> Option<String> {
        if start < end && end < self.p.lex.input.len() {
            Some(self.p.lex.input[start..end].iter().collect())
        } else {
            None
        }
    }

    pub fn warn(&self, message: &str) {
        if self.debug { println!("WARNING: {}", message) }
    }

    pub fn pushback(&mut self, c: &LexChar) {
        if c.is_eof() { return };
        self.p.lex.pcur -= 1;
        if self.p.lex.pcur > self.p.lex.pbeg && self.p.lex.input[self.p.lex.pcur] == '\n' && self.p.lex.input[self.p.lex.pcur - 1] == '\r' {
            self.p.lex.pcur -= 1;
        }
        if self.debug { println!("pushback({:?}) pcur = {}", c, self.p.lex.pcur); }
    }

    pub fn parser_magic_comment(&self) -> bool { unimplemented!("parser_magic_comment") }
    pub fn comment_at_top(&self) -> bool { unimplemented!("comment_at_top") }
    pub fn set_file_encoding(&self) { unimplemented!("set_file_encoding") }

    pub fn lex_goto_eol(&mut self) {
        self.p.lex.pcur = self.p.lex.pend;
    }

    pub fn is_lex_eol(&self) -> bool {
        self.p.lex.pcur >= self.p.lex.pend
    }

    pub fn is_lex_eol_n(&self, n: usize) -> bool {
        self.p.lex.pcur + n >= self.p.lex.pend
    }

    pub fn peek(&self, c: char) -> bool {
        self.peek_n(c, 0)
    }
    pub fn peek_n(&self, c: char, n: usize) -> bool {
        !self.is_lex_eol_n(n) && c == self.p.lex.input[self.p.lex.pcur + n]
    }

    pub fn set_yylval_id(&mut self, id: &str) {
        if self.debug { println!("set_yylval_id({})", id); }
        self.p.lval = Some(id.into());
    }

    pub fn is_spacearg(&self, c: &LexChar, space_seen: bool) -> bool {
        self.is_arg() && space_seen && !c.is_space()
    }

    pub fn is_beg(&self) -> bool {
        self.is_lex_state_some(EXPR_BEG_ANY) || self.is_lex_state_all(EXPR_ARG|EXPR_LABELED)
    }

    pub fn warn_balanced(&self, token_type: i32, op: &str, syn: &str, c: &LexChar, space_seen: bool, last_state: &LexState) -> i32 {
        if !last_state.is_some(EXPR_CLASS|EXPR_DOT|EXPR_FNAME|EXPR_ENDFN) && space_seen & !c.is_space() {
            self.warn(&format!("`{}' after local variable or literal is interpreted as binary operator even though it seems like {}", op, syn));
        }
        token_type
    }

    pub fn is_after_operator(&self) -> bool {
        self.is_lex_state_some(EXPR_FNAME|EXPR_DOT)
    }

    pub fn was_bol(&self) -> bool {
        self.p.lex.pcur == self.p.lex.pbeg + 1
    }

    pub fn is_word_match(&self, word: &str) -> bool {
        let len = word.len();

        if self.substr_at(self.p.lex.pcur, self.p.lex.pcur + len) != Some(word.to_owned()) { return false }
        if self.p.lex.pcur + len == self.p.lex.pend { return true }
        let c = self.char_at(self.p.lex.pcur + len);
        if c.is_space() { return true }
        if c == '\0' || c == '\u{0004}' || c == '\u{001A}' {
            return true;
        }
        false
    }

    pub fn compile_error(&self, message: &str) {
        if self.debug { println!("Compile error: {}", message) }
    }

    pub fn is_end(&self) -> bool {
        self.is_lex_state_some(EXPR_END_ANY)
    }

    pub fn is_arg(&self) -> bool {
        self.is_lex_state_some(EXPR_ARG_ANY)
    }

    pub fn heredoc_identifier(&self) -> i32 { unimplemented!("heredoc_identifier") }

    pub fn is_label_possible(&self, cmd_state: bool) -> bool {
        (self.is_lex_state_some(EXPR_LABEL|EXPR_ENDFN) && !cmd_state) ||
            self.is_arg()
    }

    pub fn new_strterm(&self, func: usize, term: char, paren: Option<char>) -> Option<StrTerm> {
        Some(StrTerm::Literal(StringLiteral { nest: 0, func, paren, term }))
    }

    pub fn parse_qmark(&self, _space_seen: bool) -> i32 { unimplemented!("parse_qmark") }

    pub fn arg_ambiguous(&self, c: char) -> bool {
        self.warn(&format!("ambiguous first argument; put parentheses or a space even after `{}' operator", c));
        true
    }

    pub fn tokadd(&mut self, c: &LexChar) {
        let c = c.unwrap();
        self.p.tokenbuf.push(c);
    }

    pub fn toklen(&self) -> usize {
        self.p.tokenbuf.len()
     }

    pub fn tokfix(&self) {
        // nop
    }

    pub fn tok(&self) -> String {
        self.p.tokenbuf.clone()
    }

    pub fn is_looking_at_eol(&self) -> bool {
        let mut ptr = self.p.lex.pcur;
        while ptr < self.p.lex.pend {
            let c = self.p.lex.input.get(ptr);
            ptr += 1;
            if let Some(c) = c {
                let eol = *c == '\n' || *c == '#';
                if eol || !c.is_ascii_whitespace() {
                    return eol
                }
            };
        };
        true
    }

    pub fn yyerror0(&self, message: &str) {
        if self.debug { println!("yyerror0: {}", message) }
    }

    // pub fn is_space(&self, _c: &LexChar) -> bool { unimplemented!("is_space") }

    pub fn is_lambda_beginning(&self) -> bool {
        self.p.lex.lpar_beg == self.p.lex.paren_nest
    }

    pub fn cond_push(&mut self, value: bool) {
        self.p.cond_stack.push(value)
    }

    pub fn cond_pop(&mut self) {
        self.p.cond_stack.pop()
    }

    pub fn is_cond_active(&self) -> bool {
        self.p.cond_stack.is_active()
    }

    pub fn cmdarg_push(&mut self, value: bool) {
        self.p.cmdarg_stack.push(value)
    }

    pub fn cmdarg_pop(&mut self) {
        self.p.cmdarg_stack.pop()
    }

    pub fn is_cmdarg_active(&self) -> bool {
        self.p.cmdarg_stack.is_active()
    }

    pub fn parse_percent(&mut self, _space_seen: bool, _last_state: LexState) -> i32 { unimplemented!("parse_percent") }
    pub fn parse_gvar(&mut self, _last_state: LexState) -> i32 { unimplemented!("parse_gvar") }

    pub fn parse_atmark(&mut self, last_state: LexState) -> i32 {
        let ptr = self.p.lex.pcur;
        let mut result: i32 = Self::tIVAR;
        let mut c = self.nextc();

        self.p.lex.ptok = ptr - 1; // from '@'
        self.newtok();
        self.tokadd(&LexChar::Some('@'));
        if c == '@' {
            result = Self::tCVAR;
            self.tokadd(&LexChar::Some('@'));
            c = self.nextc()
        }
        self.set_lex_state(if last_state.is_some(EXPR_FNAME) { EXPR_ENDFN } else { EXPR_END });
        if c.is_eof() || !self.parser_is_identchar() {
            self.pushback(&c);
            if result == Self::tIVAR {
                self.compile_error("`@' without identifiers is not allowed as an instance variable name");
            } else {
                self.compile_error("`@@' without identifiers is not allowed as a class variable name");
            }
            self.set_lex_state(EXPR_END);
            return result;
        } else if c.is_digit() {
            self.pushback(&c);
            if result == Self::tIVAR {
                self.compile_error(&format!("`@{}' is not allowed as an instance variable name", c.unwrap()));
            } else {
                self.compile_error(&format!("`@@{}' is not allowed as a class variable name", c.unwrap()));
            }
            self.set_lex_state(EXPR_END);
            return result;
        }

        if self.tokadd_ident(&c) { return Self::END_OF_INPUT }
        self.tokenize_ident(&last_state);
        return result;
    }

    pub fn tokadd_ident(&mut self, c: &LexChar) -> bool {
        let mut c = c.clone();
        loop {
            if self.tokadd_mbchar(&c).is_err() { return true }
            c = self.nextc();

            if !self.parser_is_identchar() { break; }
        };

        self.pushback(&c);
        return false;
    }

    pub fn is_whole_match(&self, eos: &str, indent: usize) -> bool {
        let mut ptr = self.p.lex.pbeg;
        let len = eos.len();

        if indent > 0 {
            while let Some(c) = self.p.lex.input.get(ptr) {
                if !c.is_ascii_whitespace() { break }
                ptr += 1;
            }
        }

        if let Ok(n) = isize::try_from(self.p.lex.pend - (ptr + len)) {
            if n < 0 { return false }
            let last_char = self.p.lex.input.get(ptr + len);
            let char_after_last_char = self.p.lex.input.get(ptr + len + 1);

            if n > 0 && last_char != Some(&'\n') {
                if last_char != Some(&'\r') { return false }
                if n <= 1 || char_after_last_char != Some(&'\n') { return false }
            }

            let next_len_chars: String = self.p.lex.input[ptr..ptr+len].iter().collect();
            return eos == next_len_chars
        } else {
            return false
        }
    }

    pub fn newtok(&mut self) {
        self.p.tokidx = 0;
        self.p.tokline = self.p.ruby_sourceline;
        self.p.tokenbuf = "".into();
    }

    pub fn is_identchar(&self, begin: usize, _end: usize) -> bool {
        self.p.lex.input[begin].is_alphanumeric() ||
            self.p.lex.input[begin] == '_' ||
            !self.p.lex.input[begin].is_ascii()
    }

    pub fn literal_flush(&mut self, ptok: usize) {
        self.set_ptok(ptok);
    }

    pub fn set_yylval_literal(&mut self, value: &str) {
        if self.debug { println!("set_yylval_literal({}) ptok = {}, pcur = {}", value, self.p.lex.ptok, self.p.lex.pcur); }
        self.p.lval = Some(value.into());
    }

    pub fn tokadd_mbchar(&mut self, c: &LexChar) -> Result<(), ()> {
        match c {
            LexChar::EOF => Err(()),
            _ => {
                self.tokadd(&c);
                Ok(())
            }
        }
    }

    pub fn is_label_suffix(&mut self, n: usize) -> bool {
        self.peek_n(':', n) && !self.peek_n(':', n+1)
    }

    pub fn set_yyval_name(&mut self, name: &str) {
        if self.debug { println!("set_yyval_name({})", name); }
        self.p.lval = Some(name.into());
    }
}
