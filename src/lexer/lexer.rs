use crate::lexer::{StrTerm, StringLiteral, HeredocLiteral};
use crate::lexer::lex_state::{lex_states, LexState};
use lex_states::*;
use crate::lexer::LocalsTable;
use crate::lexer::LexContext;
use crate::lexer::TokenType;
use crate::lexer::strings;
use crate::lexer::lex_char::LexChar;
use crate::lexer::Token;

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
pub struct ParserParamsLex {
    strterm: Option<StrTerm>,
    input: Vec<char>,
    lines: Vec<SourceLine>,
    prevline_idx: Option<usize>,
    lastline_idx: Option<usize>,
    lastline_start: usize,
    nextline_idx: Option<usize>,
    pbeg: usize,
    pub pcur: usize,
    pend: usize,
    ptok: usize,
    state: LexState,
    paren_nest: i32,
    lpar_beg: i32,
    brace_nest: i32,
}

#[derive(Debug, Clone, Default)]
pub struct ParserParams {
    lval: Option<String>,
    pub lex: ParserParamsLex,
    cond_stack: usize,
    cmdarg_stack: usize,
    tokidx: usize,
    toksize: usize,
    tokline: usize,
    heredoc_end: usize,
    heredoc_indent: usize,
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

    ctxt: LexContext,

    command_start: bool,
    eofp: bool,
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

    // NODE *eval_tree_begin;
    // NODE *eval_tree;
    // VALUE error_buffer;
    // VALUE debug_lines;
    // const struct rb_iseq_struct *parent_iseq;
}

#[derive(Debug, Clone, Default)]
pub struct Lexer {
    pub p: ParserParams,
}

const NULL_CHAR  : char = 0x00 as char;
const CTRL_D_CHAR: char = 0x04 as char;
const CTRL_Z_CHAR: char = 0x1a as char;
const LF_CHAR    : char = 0x0c as char;
const VTAB_CHAR  : char = 0x0b as char;

impl Lexer {
    pub fn new(source: &str) -> Self {
        let mut result = Lexer::default();
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
        println!("lines = {:#?}", lines);

        self.p.lex.input = chars;
        self.p.lex.lines = lines;
    }

    pub fn yylex(&mut self) -> Token {
        self.p.lval = None;
        let token_type = self.parser_yylex();

        let begin = self.p.lex.ptok;
        let end = self.p.lex.pcur;
        let token_value = self.p.lval.clone().unwrap_or_else(||
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

        println!("yylex {:#?}", token_type);

        Token { token_type, token_value: Some(token_value), begin: self.p.lex.ptok, end: self.p.lex.pcur }
    }

    pub fn parser_yylex(&mut self) -> TokenType {
        let mut c: LexChar;
        let mut space_seen: bool = false;
        let cmd_state: bool;
        let label: i32;
        let mut last_state: LexState;
        let token_seen = self.p.token_seen;

        if let Some(strterm) = &self.p.lex.strterm {
            match strterm {
                StrTerm::Heredoc(heredoc) => return self.here_document(heredoc),
                StrTerm::Literal(string) => return self.parse_string(string)
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
                return TokenType::END_OF_INPUT
            }

            match c {
                LexChar::EOF |
                LexChar::Some(NULL_CHAR) |
                LexChar::Some(CTRL_D_CHAR) |
                LexChar::Some(CTRL_Z_CHAR) => { return TokenType::END_OF_INPUT },

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
                        if !cc && self.p.ctxt.is_in_kwargs() {
                            self.p.command_start = true;
                            self.set_lex_state(EXPR_BEG);
                            return TokenType::tNL;
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
                                self.p.lex.ptok = self.p.lex.pcur;

                                self.p.command_start = true;
                                self.set_lex_state(EXPR_BEG);
                                return TokenType::tNL;
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
                                self.p.lex.ptok = self.p.lex.pcur;

                                self.p.command_start = true;
                                self.set_lex_state(EXPR_BEG);
                                return TokenType::tNL;
                            },
                        }
                    }
                },

                LexChar::Some('*') => {
                    let result: TokenType;

                    c = self.nextc();

                    if c == '*' {
                        c = self.nextc();
                        if c == '=' {
                            self.set_yylval_id("idPow");
                            self.set_lex_state(EXPR_BEG);
                            return TokenType::tOP_ASGN;
                        }
                        self.pushback(&c);
                        if self.is_spacearg(&c, space_seen) {
                            self.warn("`**' interpreted as argument prefix");
                            result = TokenType::tDSTAR;
                        } else if self.is_beg() {
                            result = TokenType::tDSTAR;
                        } else {
                            result = self.warn_balanced(TokenType::tPOW, "**", "argument prefix", &c, space_seen, &last_state);
                        }
                    } else {
                        if c == '=' {
                            self.set_yylval_id("*");
                            self.set_lex_state(EXPR_BEG);
                            return TokenType::tOP_ASGN;
                        }
                        self.pushback(&c);
                        if self.is_spacearg(&c, space_seen) {
                            self.warn("`*' interpreted as argument prefix");
                            result = TokenType::tSTAR;
                        } else if self.is_beg() {
                            result = TokenType::tSTAR;
                        } else {
                            result = self.warn_balanced(TokenType::tSTAR, "*", "argument prefix", &c, space_seen, &last_state);
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
                            return TokenType::tBANG;
                        }
                    } else {
                        self.set_lex_state(EXPR_BEG);
                    }
                    if c == '=' {
                        return TokenType::tNEQ;
                    }
                    if c == '~' {
                        return TokenType::tNMATCH;
                    }
                    self.pushback(&c);
                    return TokenType::tBANG;
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
                                    return TokenType::END_OF_INPUT;
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
                            return TokenType::tEQQ;
                        }
                        self.pushback(&c);
                        return TokenType::tEQ;
                    }
                    if c == '~' {
                        return TokenType::tMATCH;
                    } else if c == '>' {
                        return TokenType::tASSOC;
                    }
                    self.pushback(&c);
                    return TokenType::tEQ;
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
                            return TokenType::tCMP;
                        }
                        self.pushback(&c);
                        return TokenType::tLEQ;
                    }
                    if c == '<' {
                        c = self.nextc();
                        if c == '=' {
                            self.set_yylval_id("idLTLT");
                            self.set_lex_state(EXPR_BEG);
                            return TokenType::tOP_ASGN;
                        }
                        self.pushback(&c);
                        return TokenType::tLT;
                    }
                },

                LexChar::Some('>') => {
                    self.set_lex_state(if self.is_after_operator() { EXPR_ARG } else { EXPR_BEG });

                    c = self.nextc();
                    if c == '=' {
                        return TokenType::tGEQ;
                    }

                    if c == '>' {
                        c = self.nextc();
                        if c == '=' {
                            self.set_yylval_id("idGTGT");
                            self.set_lex_state(EXPR_BEG);
                            return TokenType::tOP_ASGN;
                        }
                        self.pushback(&c);
                        return TokenType::tRSHFT;
                    }
                    self.pushback(&c);
                    return TokenType::tGT;
                },

                LexChar::Some('"') => {
                    label = if self.is_label_possible() { strings::str_label } else { 0 };
                    self.p.lex.strterm = self.new_strterm(strings::str_dquote | label, '"', 0);
                    self.p.lex.ptok = self.p.lex.pcur - 1;
                    return TokenType::tSTRING_BEG;
                },

                LexChar::Some('`') => {
                    if self.is_lex_state_some(EXPR_FNAME) {
                        self.set_lex_state(EXPR_ENDFN);
                        return TokenType::tBACK_REF2;
                    }
                    if self.is_lex_state_some(EXPR_DOT) {
                        if cmd_state {
                            self.set_lex_state(EXPR_CMDARG);
                        } else {
                            self.set_lex_state(EXPR_ARG);
                        }
                        return TokenType::tBACK_REF2;
                    }
                    self.p.lex.strterm = self.new_strterm(strings::str_xquote, '`', 0);
                    return TokenType::tXSTRING_BEG;
                },

                LexChar::Some('\'') => {
                    label = if self.is_label_possible() { strings::str_label } else { 0 };
                    self.p.lex.strterm = self.new_strterm(strings::str_squote | label, '\'', 0);
                    return TokenType::tSTRING_BEG;
                },

                LexChar::Some('?') => {
                    return self.parse_qmark(space_seen);
                },

                LexChar::Some('&') => {
                    let result: TokenType;

                    c = self.nextc();
                    if c == '&' {
                        self.set_lex_state(EXPR_BEG);
                        c = self.nextc();
                        if c == '=' {
                            self.set_yylval_id("&&");
                            self.set_lex_state(EXPR_BEG);
                            return TokenType::tOP_ASGN;
                        }
                        self.pushback(&c);
                        return TokenType::tANDOP;
                    } else if c == '=' {
                        self.set_yylval_id("&");
                        self.set_lex_state(EXPR_BEG);
                        return TokenType::tOP_ASGN;
                    } else if c == '.' {
                        self.set_yylval_id("&.");
                        self.set_lex_state(EXPR_DOT);
                        return TokenType::tANDDOT;
                    }
                    self.pushback(&c);
                    if self.is_spacearg(&c, space_seen) {
                        // TODO: check for some warnings here
                        result = TokenType::tAMPER;
                    } else if self.is_beg() {
                        result = TokenType::tAMPER;
                    } else {
                        result = self.warn_balanced(TokenType::tAMPER2, "&", "argument prefix", &c, space_seen, &last_state);
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
                            self.set_yylval_id("idOROP");
                            self.set_lex_state(EXPR_BEG);
                            return TokenType::tOP_ASGN;
                        }
                        self.pushback(&c);
                        if last_state.is_some(EXPR_BEG) {
                            self.pushback(&LexChar::Some('|'));
                            return TokenType::tPIPE;
                        }
                        return TokenType::tOROP;
                    }
                    if c == '=' {
                        self.set_yylval_id("|");
                        self.set_lex_state(EXPR_BEG);
                        return TokenType::tOP_ASGN;
                    }
                    self.set_lex_state(if self.is_after_operator() { EXPR_ARG } else { EXPR_BEG|EXPR_LABEL });
                    return TokenType::tPIPE;
                },

                LexChar::Some('+') => {
                    c = self.nextc();
                    if self.is_after_operator() {
                        self.set_lex_state(EXPR_ARG);
                        if c == '@' {
                            return TokenType::tUPLUS;
                        }
                        self.pushback(&c);
                        return TokenType::tPLUS;
                    }
                    if c == '=' {
                        self.set_yylval_id("+");
                        self.set_lex_state(EXPR_BEG);
                        return TokenType::tOP_ASGN;
                    }
                    if self.is_beg() || (self.is_spacearg(&c, space_seen) && self.arg_ambiguous('+')) {
                        self.set_lex_state(EXPR_BEG);
                        self.pushback(&c);
                        if !c.is_eof() && c.is_digit() {
                            return self.parse_numeric('+');
                        }
                        return TokenType::tUPLUS;
                    }
                    self.set_lex_state(EXPR_BEG);
                    self.pushback(&c);
                    return self.warn_balanced(TokenType::tPLUS, "+", "unary operator", &c, space_seen, &last_state);
                },

                LexChar::Some('-') => {
                    c = self.nextc();
                    if self.is_after_operator() {
                        self.set_lex_state(EXPR_ARG);
                        if c == '@' {
                            return TokenType::tUMINUS;
                        }
                        self.pushback(&c);
                        return TokenType::tMINUS;
                    }
                    if c == '=' {
                        self.set_yylval_id("-");
                        self.set_lex_state(EXPR_BEG);
                        return TokenType::tOP_ASGN;
                    }
                    if c == '>' {
                        self.set_lex_state(EXPR_ENDFN);
                        return TokenType::tLAMBDA;
                    }
                    if self.is_beg() || (self.is_spacearg(&c, space_seen) && self.arg_ambiguous('-')) {
                        self.set_lex_state(EXPR_BEG);
                        self.pushback(&c);
                        if !c.is_eof() && c.is_digit() {
                            // panic!("here");
                            return TokenType::tUNARY_NUM;
                        }
                        return TokenType::tUMINUS;
                    }
                    self.set_lex_state(EXPR_BEG);
                    self.pushback(&c);
                    return self.warn_balanced(TokenType::tMINUS, "-", "unary operator", &c, space_seen, &last_state);
                },

                LexChar::Some('.') => {
                    let is_beg = self.is_beg();
                    self.set_lex_state(EXPR_BEG);
                    c = self.nextc();
                    if c == '.' {
                        c = self.nextc();
                        if c == '.' {
                            if self.p.lex.paren_nest == 0 && self.looking_at_eol_p() {
                                self.warn("... at EOL, should be parenthesized?");
                            } else if self.p.lex.lpar_beg >= 0 && self.p.lex.lpar_beg + 1 == self.p.lex.paren_nest {
                                if last_state.is_some(EXPR_LABEL) {
                                    return TokenType::tDOT3
                                }
                            }
                            return if is_beg { TokenType::tBDOT3 } else { TokenType::tDOT3 };
                        }
                        self.pushback(&c);
                        return if is_beg { TokenType::tBDOT2 } else { TokenType::tDOT2 };
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
                        self.p.lex.ptok = self.p.lex.pcur;
                        continue 'retrying;
                    }
                    self.set_yylval_id(".");
                    self.set_lex_state(EXPR_DOT);
                    return TokenType::tDOT;
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

                    return TokenType::tRPAREN;
                },

                LexChar::Some(']') => {
                    self.cond_pop();
                    self.cmdarg_pop();
                    self.set_lex_state(EXPR_END);
                    self.p.lex.paren_nest -= 1;

                    return TokenType::tRBRACK;
                },

                LexChar::Some('}') => {
                    // tSTRING_DEND does COND_POP and CMDARG_POP in the yacc's rule (lalrpop here)
                    self.p.lex.brace_nest -= 1;
                    if self.p.lex.brace_nest == 0 {
                        return TokenType::tSTRING_DEND;
                    }
                    self.cond_pop();
                    self.cmdarg_pop();
                    self.set_lex_state(EXPR_END);
                    self.p.lex.paren_nest -= 1;

                    return TokenType::tRCURLY;
                },

                LexChar::Some(':') => {
                    c = self.nextc();
                    if c == ':' {
                        if self.is_beg() || self.is_lex_state_some(EXPR_CLASS) || self.is_spacearg(&LexChar::EOF, space_seen) {
                            self.set_lex_state(EXPR_BEG);
                            return TokenType::tCOLON3;
                        }
                        self.set_yylval_id("idCOLON2");
                        self.set_lex_state(EXPR_DOT);
                        return TokenType::tCOLON2;
                    }
                    if self.is_end() || self.is_space(&c) || c == LexChar::Some('#') {
                        self.pushback(&c);
                        let result = self.warn_balanced(TokenType::tCOLON, ":", "symbol literal", &c, space_seen, &last_state);
                        self.set_lex_state(EXPR_BEG);
                        return result;
                    }
                    match c {
                        LexChar::Some('\'') => self.p.lex.strterm = self.new_strterm(strings::str_ssym, c.unwrap(), 0),
                        LexChar::Some('"')  => self.p.lex.strterm = self.new_strterm(strings::str_dsym, c.unwrap(), 0),
                        _ => self.pushback(&c)
                    }
                    self.set_lex_state(EXPR_FNAME);
                    return TokenType::tSYMBEG;
                },

                LexChar::Some('/') => {
                    if self.is_beg() {
                        self.p.lex.strterm = self.new_strterm(strings::str_regexp, '/', 0);
                        return TokenType::tREGEXP_BEG;
                    }
                    c = self.nextc();
                    if c == '=' {
                        self.set_yylval_id("/");
                        self.set_lex_state(EXPR_BEG);
                        return TokenType::tOP_ASGN;
                    }
                    self.pushback(&c);
                    if self.is_spacearg(&c, space_seen) {
                        self.arg_ambiguous('/');
                        self.p.lex.strterm = self.new_strterm(strings::str_regexp, '/', 0);
                        return TokenType::tREGEXP_END;
                    }
                    self.set_lex_state(if self.is_after_operator() { EXPR_ARG } else { EXPR_END });
                    return self.warn_balanced(TokenType::tDIVIDE, "/", "regexp literal", &c, space_seen, &last_state);
                },

                LexChar::Some('^') => {
                    c = self.nextc();
                    if c == '=' {
                        self.set_yylval_id("^");
                        self.set_lex_state(EXPR_BEG);
                        return TokenType::tOP_ASGN;
                    }
                    self.set_lex_state(if self.is_after_operator() { EXPR_ARG } else { EXPR_BEG });
                    self.pushback(&c);
                    return TokenType::tCARET;
                }

                LexChar::Some(';') => {
                    self.set_lex_state(EXPR_BEG);
                    self.p.command_start = true;
                    return TokenType::tSEMI;
                },

                LexChar::Some(',') => {
                    self.set_lex_state(EXPR_BEG|EXPR_LABEL);
                    return TokenType::tCOMMA;
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

                    return TokenType::tTILDE;
                },

                LexChar::Some('(') => {
                    let mut result: TokenType = TokenType::tLPAREN2;

                    if self.is_beg() {
                        result = TokenType::tLPAREN;
                    } else if !space_seen {
                        // foo( ... ) => method call, no ambiguity
                    } else if self.is_arg() || self.is_lex_state_all(EXPR_END|EXPR_LABEL) {
                        result = TokenType::tLPAREN_ARG;
                    } else if self.is_lex_state_some(EXPR_ENDFN) && !self.is_lambda_beginning() {
                        self.warn("parentheses after method name is interpreted as an argument list, not a decomposed argument");
                    }

                    self.p.lex.paren_nest += 1;
                    self.cond_push(0);
                    self.cmdarg_push(0);
                    self.set_lex_state(EXPR_BEG|EXPR_LABEL);

                    return result;
                },

                LexChar::Some('[') => {
                    let mut result: TokenType = TokenType::tLBRACK2;

                    self.p.lex.paren_nest += 1;
                    if self.is_after_operator() {
                        c = self.nextc();
                        if c == ']' {
                            self.set_lex_state(EXPR_ARG);
                            c = self.nextc();
                            if c == '=' {
                                return TokenType::tASET;
                            }
                            self.pushback(&c);
                            return TokenType::tAREF;
                        }
                        self.pushback(&c);
                        self.set_lex_state(EXPR_ARG|EXPR_LABEL);
                        return TokenType::tLBRACK2;
                    } else if self.is_beg() {
                        result = TokenType::tLBRACK;
                    } else if self.is_arg() && (space_seen || self.is_lex_state_some(EXPR_LABELED)) {
                        result = TokenType::tLBRACK;
                    }
                    self.set_lex_state(EXPR_BEG|EXPR_LABEL);
                    self.cond_push(0);
                    self.cmdarg_push(0);
                    return result;
                },

                LexChar::Some('{') => {
                    let result: TokenType;

                    if self.is_lambda_beginning() {
                        result = TokenType::tLAMBEG;
                    } else if self.is_lex_state_some(EXPR_LABELED) {
                        result = TokenType::tLBRACE;
                    } else if self.is_lex_state_some(EXPR_ARG_ANY | EXPR_END | EXPR_ENDFN) {
                        result = TokenType::tLCURLY;
                    } else if self.is_lex_state_some(EXPR_ENDARG) {
                        result = TokenType::tLBRACE_ARG;
                    } else {
                        result = TokenType::tLBRACE;
                    }

                    if result != TokenType::tLBRACE {
                        self.p.command_start = true;
                        self.set_lex_state(EXPR_BEG);
                    } else {
                        self.set_lex_state(EXPR_BEG|EXPR_LABEL);
                    }

                    self.p.lex.paren_nest += 1;
                    self.cond_push(0);
                    self.cmdarg_push(0);
                    return result;
                },

                LexChar::Some('\\') => {
                    c = self.nextc();
                    if c == '\n' {
                        space_seen = true;
                        continue 'retrying; /* skip \\n */
                    }
                    if c == ' ' {
                        return TokenType::tSP;
                    }
                    if self.is_space(&c) {
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
                        return TokenType::END_OF_INPUT;
                    }
                    self.newtok();
                },

                _ => {
                    if !self.is_identchar() {
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

    pub fn set_lex_state(&mut self, states: usize) {
        self.p.lex.state.set(states)
    }

    pub fn is_lex_state_some(&self, states: usize) -> bool {
        self.p.lex.state.is_some(states)
    }

    pub fn is_lex_state_all(&self, states: usize) -> bool {
        self.p.lex.state.is_all(states)
    }

    pub fn here_document(&self, _heredoc: &HeredocLiteral) -> TokenType { unimplemented!("here_document") }
    pub fn parse_string(&self, _string: &StringLiteral) -> TokenType { unimplemented!("parse_string") }

    pub fn token_flush(&mut self) {
        self.p.lex.ptok = self.p.lex.pcur;
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
            if n.is_err() {
                return LexChar::EOF;
            }
        }
        let mut c: char = self.p.lex.input[self.p.lex.pcur];
        self.p.lex.pcur += 1;
        if c == '\r' {
            c = self.parser_cr(c);
        }
        return LexChar::Some(c);
    }

    pub fn warn(&self, _message: &str) { unimplemented!("warn") }

    pub fn pushback(&mut self, c: &LexChar) {
        if c.is_eof() { return };
        self.p.lex.pcur -= 1;
        if self.p.lex.pcur > self.p.lex.pbeg && self.p.lex.input[self.p.lex.pcur] == '\n' && self.p.lex.input[self.p.lex.pcur - 1] == '\r' {
            self.p.lex.pcur -= 1;
        }
    }

    pub fn parser_magic_comment(&self) -> bool { unimplemented!("parser_magic_comment") }
    pub fn comment_at_top(&self) -> bool { unimplemented!("comment_at_top") }
    pub fn set_file_encoding(&self) { unimplemented!("set_file_encoding") }

    pub fn lex_goto_eol(&mut self) {
        self.p.lex.pcur = self.p.lex.pend;
    }

    pub fn emit_normal_line(&self) { unimplemented!("emit_normal_line") }
    pub fn peek(&self, _c: char) -> bool { unimplemented!("peek") }

    pub fn set_yylval_id(&mut self, id: &str) {
        println!("set_yylval_id({})", id);
        self.p.lval = Some(id.into());
    }

    pub fn is_spacearg(&self, c: &LexChar, space_seen: bool) -> bool {
        self.is_arg() && space_seen && !c.is_space()
    }

    pub fn is_beg(&self) -> bool {
        self.is_lex_state_some(EXPR_BEG_ANY) || self.is_lex_state_all(EXPR_ARG|EXPR_LABELED)
    }

    pub fn warn_balanced(&self, token_type: TokenType, op: &str, syn: &str, c: &LexChar, space_seen: bool, last_state: &LexState) -> TokenType {
        if !last_state.is_some(EXPR_CLASS|EXPR_DOT|EXPR_FNAME|EXPR_ENDFN) && space_seen & !c.is_space() {
            self.warn(&format!("`{}' after local variable or literal is interpreted as binary operator even though it seems like {}", op, syn));
        }
        token_type
    }

    pub fn is_after_operator(&self) -> bool {
        self.is_lex_state_some(EXPR_FNAME|EXPR_DOT)
    }

    pub fn was_bol(&self) -> bool { unimplemented!("was_bol") }
    pub fn is_word_match(&self, _word: &str) -> bool { unimplemented!("is_word_match") }
    pub fn compile_error(&self, _message: &str) { unimplemented!("compile_error") }

    pub fn is_end(&self) -> bool {
        self.is_lex_state_some(EXPR_END_ANY)
    }

    pub fn is_arg(&self) -> bool {
        self.is_lex_state_some(EXPR_ARG_ANY)
    }

    pub fn heredoc_identifier(&self) -> TokenType { unimplemented!("heredoc_identifier") }
    pub fn is_label_possible(&self) -> bool { unimplemented!("is_label_possible") }
    pub fn new_strterm(&self, _flag: i32, _term: char, _indent: i32) -> Option<StrTerm> { unimplemented!("new_strterm") }
    pub fn parse_qmark(&self, _space_seen: bool) -> TokenType { unimplemented!("parse_qmark") }
    pub fn arg_ambiguous(&self, _arg: char) -> bool { unimplemented!("arg_ambiguous") }

    pub fn tokadd(&mut self, c: &LexChar) {
        let c = c.unwrap();
        self.p.tokenbuf.push(c);
    }

    pub fn toklen(&self) -> usize {
        self.p.tokidx
     }

    pub fn tokfix(&self) {
        // nop
    }

    pub fn tok(&self) -> String {
        self.p.tokenbuf.clone()
    }

    pub fn looking_at_eol_p(&self) -> bool { unimplemented!("looking_at_eol_p") }
    pub fn yyerror0(&self, _message: &str) { unimplemented!("yyerror0") }
    pub fn cond_pop(&mut self) { unimplemented!("cond_pop") }
    pub fn cmdarg_pop(&mut self) { unimplemented!("cmdarg_pop") }
    pub fn is_space(&self, _c: &LexChar) -> bool { unimplemented!("is_space") }
    pub fn is_lambda_beginning(&self) -> bool { unimplemented!("is_lambda_beginning") }
    pub fn cond_push(&self, _value: usize) { unimplemented!("cond_push") }
    pub fn cmdarg_push(&self, _value: usize) { unimplemented!("cond_pop") }
    pub fn parse_percent(&self, _space_seen: bool, _last_state: LexState) -> TokenType { unimplemented!("parse_percent") }
    pub fn parse_gvar(&self, _last_state: LexState) -> TokenType { unimplemented!("parse_gvar") }
    pub fn parse_atmark(&self, _last_state: LexState) -> TokenType { unimplemented!("parse_atmark") }
    pub fn is_whole_match(&self, _pattern: &str, _indent: usize) -> bool { unimplemented!("is_whole_match") }

    pub fn newtok(&mut self) {
        self.p.tokidx = 0;
        self.p.tokline = self.p.ruby_sourceline;
        self.p.tokenbuf = "".into();
    }

    pub fn is_identchar(&self) -> bool { unimplemented!("is_identchar") }
    pub fn parse_ident(&self, _c: &LexChar, _cmd_state: bool) -> TokenType { unimplemented!("parse_ident") }
    pub fn literal_flush(&mut self, _some_value: usize) { unimplemented!("literal_flush") }

    pub fn set_yylval_literal(&mut self, value: &str) {
        println!("set_yylval_literal({})", value);
        self.p.lval = Some(value.into());
    }
}
