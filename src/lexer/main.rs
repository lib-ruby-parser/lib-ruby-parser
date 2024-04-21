use alloc_from_pool::{Factory as PoolFactory, PoolValue};
use lib_ruby_parser_ast_arena::Blob;
use lib_ruby_parser_ast_arena::IntrusiveList;

use crate::lexer::*;
use crate::maybe_byte::*;
use crate::source::buffer::*;
use crate::source::Comment;
use crate::source::Decoder;
use crate::source::MagicComment;
use crate::str_term::{str_types::*, HeredocEnd, StrTerm, StringLiteral};
use crate::Loc;
use crate::SharedContext;
use crate::StackState;
use crate::StaticEnvironment;
use crate::Token;
use crate::TokenBuf;
use crate::{error::Diagnostics, Bytes};
use crate::{lex_states::*, LexState};
use crate::{Diagnostic, DiagnosticMessage, ErrorLevel};

/// A struct responsible for converting a given input
/// into a sequence of tokens
#[derive(Debug)]
pub struct Lexer<'b> {
    pub(crate) buffer: Buffer<'b>,

    pub(crate) lval: Option<Bytes>,
    pub(crate) lval_start: Option<usize>,
    pub(crate) lval_end: Option<usize>,

    pub(crate) strterm: Option<Box<StrTerm<'b>>>,
    /// Current state of the lexer, used internally for testing
    pub lex_state: LexState,
    pub(crate) paren_nest: i32,
    pub(crate) lpar_beg: i32,
    pub(crate) brace_nest: i32,

    /// Internal field, used to differentiate kDO_COND vs kDO,
    /// exposed for internal testing
    pub cond: StackState,
    /// Internal field, used to differentiate kDO_BLOCK vs kDO,
    /// exposed for internal testing
    pub cmdarg: StackState,

    pub(crate) tokenbuf: TokenBuf<'b>,

    // pub(crate) max_numparam: usize,
    pub(crate) context: SharedContext,

    pub(crate) command_start: bool,
    pub(crate) token_seen: bool,

    /// Stack of sets of variables in current scopes.
    /// Each stack item represents locals in the scope.
    ///
    /// You can use it to pre-define some locals and parse
    /// your input as if these locals exist.
    ///
    /// For example, you can parse the following code
    ///
    /// ```text
    /// a = b + c
    /// ```
    ///
    /// as
    ///
    /// ```text
    /// Send(LocalVar(a), "+", LocalVar(b))
    /// ```
    ///
    /// by declaring `a` and `b` as locals using
    ///
    /// ```text
    /// parser.lexer.static_env.declare("a")
    /// parser.lexer.static_env.declare("b")
    /// parser.parse()
    /// ```
    pub static_env: StaticEnvironment,

    pub(crate) diagnostics: Diagnostics,
    pub(crate) comments: &'b mut IntrusiveList<'b, Comment>,
    pub(crate) magic_comments: &'b mut IntrusiveList<'b, MagicComment>,

    #[doc(hidden)]
    pub tokens_factory: PoolFactory<Token>,
    pub(crate) blob: &'b Blob<'b>,
}

impl<'b> Lexer<'b> {
    pub(crate) const NULL_CHAR: u8 = 0x00;
    pub(crate) const CTRL_D_CHAR: u8 = 0x04;
    pub(crate) const CTRL_Z_CHAR: u8 = 0x1a;
    pub(crate) const LF_CHAR: u8 = 0x0c;
    pub(crate) const VTAB_CHAR: u8 = 0x0b;

    /// Constructs an instance of Lexer
    pub fn new(
        bytes: &'b [u8],
        name: &'b str,
        decoder: Option<Decoder<'b>>,
        blob: &'b Blob<'b>,
    ) -> Self {
        Self {
            cond: StackState::new("cond"),
            cmdarg: StackState::new("cmdarg"),
            lpar_beg: -1, /* make lambda_beginning_p() == FALSE at first */
            buffer: Buffer::new(name, bytes, decoder, blob),
            lval: None,
            lval_start: None,
            lval_end: None,
            strterm: None,
            lex_state: LexState::default(),
            paren_nest: 0,
            brace_nest: 0,
            tokenbuf: TokenBuf::empty(blob),
            context: SharedContext::default(),
            command_start: false,
            token_seen: false,
            static_env: StaticEnvironment::default(),
            diagnostics: Diagnostics::default(),
            comments: blob.alloc_ref(),
            magic_comments: blob.alloc_ref(),
            tokens_factory: PoolFactory::default(),
            blob,
        }
    }

    /// Tokenizes given input until EOF
    ///
    /// Keep in mind that Lexer in Ruby is driven by Parser,
    /// and so this method on its own can return a wrong sequence
    /// of tokens. It's used internally to test simple inputs.
    ///
    /// If you need to get tokens better use `ParserResult::tokens` field
    pub fn tokenize_until_eof(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        loop {
            let token = self.yylex().take_value();
            match token.token_type {
                Self::END_OF_INPUT => break,
                _ => tokens.push(token),
            }
        }

        tokens
    }

    pub(crate) fn yylex(&mut self) -> PoolValue<Token> {
        self.lval = None;

        let token_type = self.parser_yylex();

        let begin = core::mem::take(&mut self.lval_start).unwrap_or(self.buffer.ptok);
        let mut end = core::mem::take(&mut self.lval_end).unwrap_or(self.buffer.pcur);

        let mut token_value = self
            .lval
            .take()
            .or_else(|| {
                // take raw value if nothing was manually captured
                self.buffer
                    .substr_at(begin, end)
                    .map(|s| Bytes::new(Vec::from(s)))
            })
            .unwrap_or_else(|| Bytes::new(vec![]));

        if token_type == Self::tNL {
            token_value = Bytes::new(vec![b'\n']);
            end = begin + 1;
        }

        let token = self.tokens_factory.alloc(Token {
            token_type,
            token_value,
            loc: Loc { begin, end },
        });
        println_if_debug_lexer!(
            "yylex ({:?}, {:?}, {:?})",
            token.token_name(),
            token.token_value,
            token.loc
        );
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

    pub(crate) fn parser_yylex(&mut self) -> i32 {
        let mut c: MaybeByte;
        let mut space_seen: bool = false;
        let label: usize;
        let mut last_state: LexState;
        let token_seen = self.token_seen;

        if let Some(strterm) = self.strterm.as_ref().map(|i| i.as_ref()) {
            match strterm {
                StrTerm::HeredocLiteral(_) => {
                    return self.here_document();
                }

                StrTerm::StringLiteral(_) => {
                    self.token_flush();
                    return self.parse_string();
                }
            }
        }

        let cmd_state = self.command_start;
        self.command_start = false;
        self.token_seen = true;

        'retrying: loop {
            last_state = self.lex_state;
            self.token_flush();

            // handle EOF
            c = self.nextc();

            if c.is_eof() {
                return Self::END_OF_INPUT;
            }

            match c.as_option() {
                None
                | Some(Self::NULL_CHAR)
                | Some(Self::CTRL_D_CHAR)
                | Some(Self::CTRL_Z_CHAR) => return Self::END_OF_INPUT,

                // whitespaces
                Some(b'\r') => {
                    if !self.buffer.cr_seen {
                        self.buffer.cr_seen = true;
                        self.warn(
                            DiagnosticMessage::SlashRAtMiddleOfLine {},
                            self.current_loc(),
                        );
                    }
                }

                Some(b' ') | Some(b'\t') | Some(Self::LF_CHAR) | Some(Self::VTAB_CHAR) => {
                    space_seen = true;
                    continue 'retrying;
                }

                Some(b'#') | Some(b'\n') => {
                    if c == b'#' {
                        // it's a comment
                        self.token_seen = token_seen;
                        // no magic_comment in shebang line
                        let magic_comment = self
                            .magic_comment(self.buffer.pcur, self.buffer.pend - self.buffer.pcur);
                        match magic_comment {
                            Ok(magic_comment) => {
                                if !magic_comment && self.comment_at_top() {
                                    self.set_file_encoding(self.buffer.pcur, self.buffer.pend)
                                }
                            }
                            Err(_) => return Self::END_OF_INPUT,
                        }
                        self.buffer.goto_eol();
                        self.comments.push(Comment::from_loc_and_input(
                            self.current_loc(),
                            &self.buffer.input.decoded,
                            self.blob,
                        ))
                    }
                    self.token_seen = token_seen;
                    let cc = self
                        .lex_state
                        .is_some(EXPR_BEG | EXPR_CLASS | EXPR_FNAME | EXPR_DOT)
                        && !self.lex_state.is_some(EXPR_LABELED);
                    if cc || self.lex_state.is_all(EXPR_ARG | EXPR_LABELED) {
                        if !cc && self.context.in_kwarg() {
                            return self.normal_newline_leaf_label();
                        }
                        continue 'retrying;
                    }

                    loop {
                        // while(1)
                        c = self.nextc();

                        #[allow(clippy::never_loop)]
                        // emulate ugly C switch with fall-through logic
                        loop {
                            if c == b' '
                                || c == b'\t'
                                || c == Self::LF_CHAR
                                || c == b'\r'
                                || c == Self::VTAB_CHAR
                            {
                                space_seen = true;
                                break;
                            }

                            if c == b'#' {
                                self.buffer.pushback(c);
                                continue 'retrying;
                            }

                            if c == b'&' || c == b'.' {
                                if self.buffer.peek(b'.') == (c == b'&') {
                                    self.buffer.pushback(c);
                                    continue 'retrying;
                                }
                            }

                            if c.is_eof() {
                                // EOF no decrement
                                self.buffer.eof_no_decrement();
                                return self.normal_newline_leaf_label();
                            }

                            // default:
                            self.buffer.ruby_sourceline -= 1;
                            self.buffer.nextline = self.buffer.lastline;
                            // -1 branch fallthrough
                            self.buffer.eof_no_decrement();
                            return self.normal_newline_leaf_label();
                        }
                    }
                }

                Some(b'*') => {
                    let result: i32;

                    c = self.nextc();

                    if c == b'*' {
                        c = self.nextc();
                        if c == b'=' {
                            self.set_yylval_id("**=");
                            self.lex_state.set(EXPR_BEG);
                            return Self::tOP_ASGN;
                        }
                        self.buffer.pushback(c);
                        if self.lex_state.is_spacearg(c, space_seen) {
                            self.warn(
                                DiagnosticMessage::DStarInterpretedAsArgPrefix {},
                                self.current_loc(),
                            );
                            result = Self::tDSTAR;
                        } else if self.lex_state.is_beg() {
                            result = Self::tDSTAR;
                        } else {
                            result = self.warn_balanced(
                                Self::tPOW,
                                "**",
                                "argument prefix",
                                c,
                                space_seen,
                                last_state,
                            );
                        }
                    } else {
                        if c == b'=' {
                            self.set_yylval_id("*=");
                            self.lex_state.set(EXPR_BEG);
                            return Self::tOP_ASGN;
                        }
                        self.buffer.pushback(c);
                        if self.lex_state.is_spacearg(c, space_seen) {
                            self.warn(
                                DiagnosticMessage::StarInterpretedAsArgPrefix {},
                                self.current_loc(),
                            );
                            result = Self::tSTAR;
                        } else if self.lex_state.is_beg() {
                            result = Self::tSTAR;
                        } else {
                            result = self.warn_balanced(
                                Self::tSTAR2,
                                "*",
                                "argument prefix",
                                c,
                                space_seen,
                                last_state,
                            );
                        }
                    }

                    self.lex_state.set(if self.lex_state.is_after_operator() {
                        EXPR_ARG
                    } else {
                        EXPR_BEG
                    });
                    return result;
                }

                Some(b'!') => {
                    c = self.nextc();
                    if self.lex_state.is_after_operator() {
                        self.lex_state.set(EXPR_ARG);
                        if c == b'@' {
                            return Self::tBANG;
                        }
                    } else {
                        self.lex_state.set(EXPR_BEG);
                    }
                    if c == b'=' {
                        return Self::tNEQ;
                    }
                    if c == b'~' {
                        return Self::tNMATCH;
                    }
                    self.buffer.pushback(c);
                    return Self::tBANG;
                }

                Some(b'=') => {
                    if self.buffer.was_bol() {
                        // skip embedded rd document
                        if self.buffer.is_word_match("begin") {
                            let begin_loc = self.loc(self.buffer.pcur - 1, self.buffer.pcur + 5);
                            self.buffer.goto_eol();
                            loop {
                                self.buffer.goto_eol();
                                c = self.nextc();
                                if c.is_eof() {
                                    self.compile_error(
                                        DiagnosticMessage::EmbeddedDocumentMeetsEof {},
                                        begin_loc,
                                    );
                                    return Self::END_OF_INPUT;
                                }
                                if c == b'=' && self.buffer.is_word_match("end") {
                                    break;
                                }
                                self.buffer.pushback(c);
                            }
                            self.buffer.goto_eol();
                            self.comments.push(Comment::from_loc_and_input(
                                begin_loc.with_end(self.buffer.pcur),
                                &self.buffer.input.decoded,
                                self.blob,
                            ));
                            continue 'retrying;
                        }
                    }

                    self.lex_state.set(if self.lex_state.is_after_operator() {
                        EXPR_ARG
                    } else {
                        EXPR_BEG
                    });
                    c = self.nextc();
                    if c == b'=' {
                        c = self.nextc();
                        if c == b'=' {
                            return Self::tEQQ;
                        }
                        self.buffer.pushback(c);
                        return Self::tEQ;
                    }
                    if c == b'~' {
                        return Self::tMATCH;
                    } else if c == b'>' {
                        return Self::tASSOC;
                    }
                    self.buffer.pushback(c);
                    return Self::tEQL;
                }

                Some(b'<') => {
                    c = self.nextc();
                    if c == b'<'
                        && !self.lex_state.is_some(EXPR_DOT | EXPR_CLASS)
                        && !self.lex_state.is_end()
                        && (!self.lex_state.is_arg()
                            || self.lex_state.is_some(EXPR_LABELED)
                            || space_seen)
                    {
                        if let Some(token) = self.heredoc_identifier() {
                            return token;
                        }
                    }
                    if self.lex_state.is_after_operator() {
                        self.lex_state.set(EXPR_ARG);
                    } else {
                        if self.lex_state.is_some(EXPR_CLASS) {
                            self.command_start = true;
                        }
                        self.lex_state.set(EXPR_BEG);
                    }
                    if c == b'=' {
                        c = self.nextc();
                        if c == b'>' {
                            return Self::tCMP;
                        }
                        self.buffer.pushback(c);
                        return Self::tLEQ;
                    }
                    if c == b'<' {
                        c = self.nextc();
                        if c == b'=' {
                            self.set_yylval_id("<<=");
                            self.lex_state.set(EXPR_BEG);
                            return Self::tOP_ASGN;
                        }
                        self.buffer.pushback(c);
                        return self.warn_balanced(
                            Self::tLSHFT,
                            "<<",
                            "here document",
                            c,
                            space_seen,
                            last_state,
                        );
                    }
                    self.buffer.pushback(c);
                    return Self::tLT;
                }

                Some(b'>') => {
                    self.lex_state.set(if self.lex_state.is_after_operator() {
                        EXPR_ARG
                    } else {
                        EXPR_BEG
                    });

                    c = self.nextc();
                    if c == b'=' {
                        return Self::tGEQ;
                    }

                    if c == b'>' {
                        c = self.nextc();
                        if c == b'=' {
                            self.set_yylval_id(">>=");
                            self.lex_state.set(EXPR_BEG);
                            return Self::tOP_ASGN;
                        }
                        self.buffer.pushback(c);
                        return Self::tRSHFT;
                    }
                    self.buffer.pushback(c);
                    return Self::tGT;
                }

                Some(b'"') => {
                    label = if self.lex_state.is_label_possible(cmd_state) {
                        str_label
                    } else {
                        0
                    };
                    self.strterm = self.new_strterm(str_dquote | label, b'"', None, None);
                    self.buffer.set_ptok(self.buffer.pcur - 1);
                    return Self::tSTRING_BEG;
                }

                Some(b'`') => {
                    if self.lex_state.is_some(EXPR_FNAME) {
                        self.lex_state.set(EXPR_ENDFN);
                        return Self::tBACK_REF2;
                    }
                    if self.lex_state.is_some(EXPR_DOT) {
                        if cmd_state {
                            self.lex_state.set(EXPR_CMDARG);
                        } else {
                            self.lex_state.set(EXPR_ARG);
                        }
                        return Self::tBACK_REF2;
                    }
                    self.strterm = self.new_strterm(str_xquote, b'`', None, None);
                    return Self::tXSTRING_BEG;
                }

                Some(b'\'') => {
                    label = if self.lex_state.is_label_possible(cmd_state) {
                        str_label
                    } else {
                        0
                    };
                    self.strterm = self.new_strterm(str_squote | label, b'\'', None, None);
                    self.buffer.set_ptok(self.buffer.pcur - 1);
                    return Self::tSTRING_BEG;
                }

                Some(b'?') => {
                    return self.parse_qmark(space_seen).unwrap_or(-1);
                }

                Some(b'&') => {
                    let result: i32;

                    c = self.nextc();
                    if c == b'&' {
                        self.lex_state.set(EXPR_BEG);
                        c = self.nextc();
                        if c == b'=' {
                            self.set_yylval_id("&&=");
                            self.lex_state.set(EXPR_BEG);
                            return Self::tOP_ASGN;
                        }
                        self.buffer.pushback(c);
                        return Self::tANDOP;
                    } else if c == b'=' {
                        self.set_yylval_id("&=");
                        self.lex_state.set(EXPR_BEG);
                        return Self::tOP_ASGN;
                    } else if c == b'.' {
                        self.set_yylval_id("&.");
                        self.lex_state.set(EXPR_DOT);
                        return Self::tANDDOT;
                    }
                    self.buffer.pushback(c);
                    if self.lex_state.is_spacearg(c, space_seen) {
                        if c != b':'
                            || {
                                c = self.buffer.peekc_n(1);
                                !c.is_eof()
                            }
                            || !(c == b'\''
                                || c == b'"'
                                || self
                                    .buffer
                                    .is_identchar(self.buffer.pcur + 1, self.buffer.pend))
                        {
                            self.warn(
                                DiagnosticMessage::AmpersandInterpretedAsArgPrefix {},
                                self.current_loc(),
                            );
                        }
                        result = Self::tAMPER;
                    } else if self.lex_state.is_beg() {
                        result = Self::tAMPER;
                    } else {
                        result = self.warn_balanced(
                            Self::tAMPER2,
                            "&",
                            "argument prefix",
                            c,
                            space_seen,
                            last_state,
                        );
                    }
                    self.lex_state.set(if self.lex_state.is_after_operator() {
                        EXPR_ARG
                    } else {
                        EXPR_BEG
                    });
                    return result;
                }

                Some(b'|') => {
                    c = self.nextc();
                    if c == b'|' {
                        self.lex_state.set(EXPR_BEG);
                        c = self.nextc();
                        if c == b'=' {
                            self.set_yylval_id("||=");
                            self.lex_state.set(EXPR_BEG);
                            return Self::tOP_ASGN;
                        }
                        self.buffer.pushback(c);
                        if last_state.is_some(EXPR_BEG) {
                            self.buffer.pushback(b'|');
                            return Self::tPIPE;
                        }
                        return Self::tOROP;
                    }
                    if c == b'=' {
                        self.set_yylval_id("|=");
                        self.lex_state.set(EXPR_BEG);
                        return Self::tOP_ASGN;
                    }
                    self.lex_state.set(if self.lex_state.is_after_operator() {
                        EXPR_ARG
                    } else {
                        EXPR_BEG | EXPR_LABEL
                    });
                    self.buffer.pushback(c);
                    return Self::tPIPE;
                }

                Some(b'+') => {
                    c = self.nextc();
                    if self.lex_state.is_after_operator() {
                        self.lex_state.set(EXPR_ARG);
                        if c == b'@' {
                            return Self::tUPLUS;
                        }
                        self.buffer.pushback(c);
                        return Self::tPLUS;
                    }
                    if c == b'=' {
                        self.set_yylval_id("+=");
                        self.lex_state.set(EXPR_BEG);
                        return Self::tOP_ASGN;
                    }
                    if self.lex_state.is_beg()
                        || (self.lex_state.is_spacearg(c, space_seen)
                            && self.arg_ambiguous(b'+', self.current_loc().adjust_end(-1)))
                    {
                        self.lex_state.set(EXPR_BEG);
                        self.buffer.pushback(c);
                        if !c.is_eof() && c.is_digit() {
                            return self.parse_numeric(b'+');
                        }
                        return Self::tUPLUS;
                    }
                    self.lex_state.set(EXPR_BEG);
                    self.buffer.pushback(c);
                    return self.warn_balanced(
                        Self::tPLUS,
                        "+",
                        "unary operator",
                        c,
                        space_seen,
                        last_state,
                    );
                }

                Some(b'-') => {
                    c = self.nextc();
                    if self.lex_state.is_after_operator() {
                        self.lex_state.set(EXPR_ARG);
                        if c == b'@' {
                            return Self::tUMINUS;
                        }
                        self.buffer.pushback(c);
                        return Self::tMINUS;
                    }
                    if c == b'=' {
                        self.set_yylval_id("-=");
                        self.lex_state.set(EXPR_BEG);
                        return Self::tOP_ASGN;
                    }
                    if c == b'>' {
                        self.lex_state.set(EXPR_ENDFN);
                        return Self::tLAMBDA;
                    }
                    if self.lex_state.is_beg()
                        || (self.lex_state.is_spacearg(c, space_seen)
                            && self.arg_ambiguous(b'-', self.current_loc().adjust_end(-1)))
                    {
                        self.lex_state.set(EXPR_BEG);
                        self.buffer.pushback(c);
                        if !c.is_eof() && c.is_digit() {
                            return Self::tUMINUS_NUM;
                        }
                        return Self::tUMINUS;
                    }
                    self.lex_state.set(EXPR_BEG);
                    self.buffer.pushback(c);
                    return self.warn_balanced(
                        Self::tMINUS,
                        "-",
                        "unary operator",
                        c,
                        space_seen,
                        last_state,
                    );
                }

                Some(b'.') => {
                    let is_beg = self.lex_state.is_beg();
                    self.lex_state.set(EXPR_BEG);
                    c = self.nextc();
                    if c == b'.' {
                        c = self.nextc();
                        if c == b'.' {
                            if self.context.in_argdef() {
                                self.lex_state.set(EXPR_ENDARG);
                                return Self::tBDOT3;
                            }
                            if self.paren_nest == 0 && self.buffer.is_looking_at_eol() {
                                self.warn(DiagnosticMessage::TripleDotAtEol {}, self.current_loc());
                            } else if self.lpar_beg >= 0
                                && self.lpar_beg + 1 == self.paren_nest
                                && last_state.is_some(EXPR_LABEL)
                            {
                                return Self::tDOT3;
                            }
                            return if is_beg { Self::tBDOT3 } else { Self::tDOT3 };
                        }
                        self.buffer.pushback(c);
                        return if is_beg { Self::tBDOT2 } else { Self::tDOT2 };
                    }
                    self.buffer.pushback(c);
                    if !c.is_eof() && c.is_digit() {
                        let prev = if self.buffer.pcur - 1 > self.buffer.pbeg {
                            self.buffer.byte_at(self.buffer.pcur - 2)
                        } else {
                            MaybeByte::EndOfInput
                        };
                        self.parse_numeric(b'.');
                        if prev.is_digit() {
                            self.yyerror0(DiagnosticMessage::FractionAfterNumeric {});
                        } else {
                            self.yyerror0(DiagnosticMessage::NoDigitsAfterDot {});
                        }
                        self.lex_state.set(EXPR_END);
                        self.buffer.set_ptok(self.buffer.pcur);
                        continue 'retrying;
                    }
                    self.set_yylval_id(".");
                    self.lex_state.set(EXPR_DOT);
                    return Self::tDOT;
                }

                Some(c) if c.is_ascii_digit() => {
                    return self.parse_numeric(c);
                }

                Some(b')') => {
                    self.cond.pop();
                    self.cmdarg.pop();
                    self.lex_state.set(EXPR_ENDFN);
                    self.paren_nest -= 1;

                    return Self::tRPAREN;
                }

                Some(b']') => {
                    self.cond.pop();
                    self.cmdarg.pop();
                    self.lex_state.set(EXPR_END);
                    self.paren_nest -= 1;

                    return Self::tRBRACK;
                }

                Some(b'}') => {
                    // tSTRING_DEND does COND.POP and CMDARG.POP in the yacc's rule (lalrpop here)
                    if self.brace_nest == 0 {
                        self.brace_nest -= 1;
                        return Self::tSTRING_DEND;
                    }
                    self.brace_nest -= 1;
                    self.cond.pop();
                    self.cmdarg.pop();
                    self.lex_state.set(EXPR_END);
                    self.paren_nest -= 1;

                    return Self::tRCURLY;
                }

                Some(b':') => {
                    c = self.nextc();
                    if c == b':' {
                        if self.lex_state.is_beg()
                            || self.lex_state.is_some(EXPR_CLASS)
                            || self
                                .lex_state
                                .is_spacearg(MaybeByte::EndOfInput, space_seen)
                        {
                            self.lex_state.set(EXPR_BEG);
                            return Self::tCOLON3;
                        }
                        self.set_yylval_id("::");
                        self.lex_state.set(EXPR_DOT);
                        return Self::tCOLON2;
                    }
                    if self.lex_state.is_end() || c.is_space() || c == Some(b'#') {
                        self.buffer.pushback(c);
                        let result = self.warn_balanced(
                            Self::tCOLON,
                            ":",
                            "symbol literal",
                            c,
                            space_seen,
                            last_state,
                        );
                        self.lex_state.set(EXPR_BEG);
                        return result;
                    }
                    match c.as_option() {
                        Some(c) if c == b'\'' => {
                            self.strterm = self.new_strterm(str_ssym, c, None, None)
                        }
                        Some(c) if c == b'"' => {
                            self.strterm = self.new_strterm(str_dsym, c, None, None)
                        }
                        _ => self.buffer.pushback(c),
                    }
                    self.lex_state.set(EXPR_FNAME);
                    return Self::tSYMBEG;
                }

                Some(b'/') => {
                    if self.lex_state.is_beg() {
                        self.strterm = self.new_strterm(str_regexp, b'/', None, None);
                        return Self::tREGEXP_BEG;
                    }
                    c = self.nextc();
                    if c == b'=' {
                        self.set_yylval_id("/=");
                        self.lex_state.set(EXPR_BEG);
                        return Self::tOP_ASGN;
                    }
                    self.buffer.pushback(c);
                    if self.lex_state.is_spacearg(c, space_seen) {
                        self.arg_ambiguous(b'/', self.current_loc());
                        self.strterm = self.new_strterm(str_regexp, b'/', None, None);
                        return Self::tREGEXP_BEG;
                    }
                    self.lex_state.set(if self.lex_state.is_after_operator() {
                        EXPR_ARG
                    } else {
                        EXPR_BEG
                    });
                    return self.warn_balanced(
                        Self::tDIVIDE,
                        "/",
                        "regexp literal",
                        c,
                        space_seen,
                        last_state,
                    );
                }

                Some(b'^') => {
                    c = self.nextc();
                    if c == b'=' {
                        self.set_yylval_id("^=");
                        self.lex_state.set(EXPR_BEG);
                        return Self::tOP_ASGN;
                    }
                    self.lex_state.set(if self.lex_state.is_after_operator() {
                        EXPR_ARG
                    } else {
                        EXPR_BEG
                    });
                    self.buffer.pushback(c);
                    return Self::tCARET;
                }

                Some(b';') => {
                    self.lex_state.set(EXPR_BEG);
                    self.command_start = true;
                    return Self::tSEMI;
                }

                Some(b',') => {
                    self.lex_state.set(EXPR_BEG | EXPR_LABEL);
                    return Self::tCOMMA;
                }

                Some(b'~') => {
                    if self.lex_state.is_after_operator() {
                        c = self.nextc();
                        if c != b'@' {
                            self.buffer.pushback(c);
                        }
                        self.lex_state.set(EXPR_ARG);
                    } else {
                        self.lex_state.set(EXPR_BEG);
                    }

                    return Self::tTILDE;
                }

                Some(b'(') => {
                    let mut result: i32 = Self::tLPAREN2;

                    if self.lex_state.is_beg() {
                        result = Self::tLPAREN;
                    } else if !space_seen {
                        // foo( ... ) => method call, no ambiguity
                    } else if self.lex_state.is_arg()
                        || self.lex_state.is_all(EXPR_END | EXPR_LABEL)
                    {
                        result = Self::tLPAREN_ARG;
                    } else if self.lex_state.is_some(EXPR_ENDFN) && !self.is_lambda_beginning() {
                        self.warn(
                            DiagnosticMessage::ParenthesesIterpretedAsArglist {},
                            self.current_loc(),
                        );
                    }

                    self.paren_nest += 1;
                    self.cond.push(false);
                    self.cmdarg.push(false);
                    self.lex_state.set(EXPR_BEG | EXPR_LABEL);

                    return result;
                }

                Some(b'[') => {
                    let mut result: i32 = Self::tLBRACK2;

                    self.paren_nest += 1;
                    if self.lex_state.is_after_operator() {
                        c = self.nextc();
                        if c == b']' {
                            self.paren_nest -= 1;
                            self.lex_state.set(EXPR_ARG);
                            c = self.nextc();
                            if c == b'=' {
                                return Self::tASET;
                            }
                            self.buffer.pushback(c);
                            return Self::tAREF;
                        }
                        self.buffer.pushback(c);
                        self.lex_state.set(EXPR_ARG | EXPR_LABEL);
                        return Self::tLBRACK2;
                    } else if self.lex_state.is_beg()
                        || (self.lex_state.is_arg()
                            && (space_seen || self.lex_state.is_some(EXPR_LABELED)))
                    {
                        result = Self::tLBRACK;
                    }
                    self.lex_state.set(EXPR_BEG | EXPR_LABEL);
                    self.cond.push(false);
                    self.cmdarg.push(false);
                    return result;
                }

                Some(b'{') => {
                    self.brace_nest += 1;

                    let result: i32;

                    if self.is_lambda_beginning() {
                        result = Self::tLAMBEG;
                    } else if self.lex_state.is_some(EXPR_LABELED) {
                        result = Self::tLBRACE;
                    } else if self.lex_state.is_some(EXPR_ARG_ANY | EXPR_END | EXPR_ENDFN) {
                        result = Self::tLCURLY;
                    } else if self.lex_state.is_some(EXPR_ENDARG) {
                        result = Self::tLBRACE_ARG;
                    } else {
                        result = Self::tLBRACE;
                    }

                    if result != Self::tLBRACE {
                        self.command_start = true;
                        self.lex_state.set(EXPR_BEG);
                    } else {
                        self.lex_state.set(EXPR_BEG | EXPR_LABEL);
                    }

                    self.paren_nest += 1;
                    self.cond.push(false);
                    self.cmdarg.push(false);
                    return result;
                }

                Some(b'\\') => {
                    c = self.nextc();
                    if c == b'\n' {
                        space_seen = true;
                        continue 'retrying; /* skip \\n */
                    }
                    if c == b' ' {
                        return Self::tSP;
                    }
                    if c.is_space() {
                        match c.as_option() {
                            Some(b'\t') => return Self::tSLASH_T,
                            Some(Self::LF_CHAR) => return Self::tSLASH_F,
                            Some(b'\r') => return Self::tSLASH_R,
                            Some(Self::VTAB_CHAR) => return Self::tVTAB,
                            Some(other) => unreachable!("unsupported space char {:?}", other),
                            None => {}
                        }
                    }
                    self.buffer.pushback(c);
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

                Some(c) => {
                    if !self.is_identchar() {
                        self.compile_error(
                            DiagnosticMessage::InvalidChar { c },
                            self.current_loc(),
                        );
                        self.token_flush();
                        continue 'retrying;
                    }

                    self.newtok();
                }
            }

            break;
        }

        self.parse_ident(c, cmd_state)
    }

    fn normal_newline_leaf_label(&mut self) -> i32 {
        self.command_start = true;
        self.lex_state.set(EXPR_BEG);
        Self::tNL
    }

    pub(crate) fn warn(&mut self, message: DiagnosticMessage, loc: Loc) {
        println_if_debug_lexer!("WARNING: {}", message.render());
        let diagnostic = Diagnostic {
            level: ErrorLevel::Warning,
            message,
            loc,
        };
        self.diagnostics.emit(diagnostic);
    }

    pub(crate) fn warn_balanced(
        &mut self,
        token_type: i32,
        op: &'static str,
        syn: &'static str,
        c: MaybeByte,
        space_seen: bool,
        last_state: LexState,
    ) -> i32 {
        if !last_state.is_some(EXPR_CLASS | EXPR_DOT | EXPR_FNAME | EXPR_ENDFN)
            && space_seen & !c.is_space()
        {
            self.warn(
                DiagnosticMessage::AmbiguousOperator {
                    operator: op.to_string(),
                    interpreted_as: syn.to_string(),
                },
                self.current_loc(),
            );
        }
        token_type
    }

    pub(crate) fn compile_error(&mut self, message: DiagnosticMessage, loc: Loc) {
        println_if_debug_lexer!("Compile error: {}", message.render());
        let diagnostic = Diagnostic {
            level: ErrorLevel::Error,
            message,
            loc,
        };
        self.diagnostics.emit(diagnostic);
    }

    pub(crate) fn new_strterm(
        &self,
        func: usize,
        term: u8,
        paren: Option<u8>,
        heredoc_end: Option<HeredocEnd<'b>>,
    ) -> Option<Box<StrTerm<'b>>> {
        Some(Box::new(StrTerm::new_literal(StringLiteral::new(
            0,
            func,
            paren,
            term,
            heredoc_end,
        ))))
    }

    pub(crate) fn loc(&self, begin_pos: usize, end_pos: usize) -> Loc {
        Loc {
            begin: begin_pos,
            end: end_pos,
        }
    }

    pub(crate) fn current_loc(&self) -> Loc {
        self.loc(self.buffer.ptok, self.buffer.pcur)
    }

    pub(crate) fn arg_ambiguous(&mut self, c: u8, loc: Loc) -> bool {
        if c == b'/' {
            self.warn(DiagnosticMessage::AmbiguousRegexp {}, loc);
        } else {
            self.warn(
                DiagnosticMessage::AmbiguousFirstArgument { operator: c },
                loc,
            );
        }
        true
    }

    pub(crate) fn toklen(&self) -> usize {
        self.tokenbuf.len()
    }

    pub(crate) fn tokfix(&self) {
        // nop
    }

    pub(crate) fn yyerror0(&mut self, message: DiagnosticMessage) {
        self.yyerror1(message, self.current_loc());
    }

    pub(crate) fn yyerror1(&mut self, message: DiagnosticMessage, loc: Loc) {
        println_if_debug_lexer!("yyerror0: {}", message.render());
        let diagnostic = Diagnostic {
            level: ErrorLevel::Error,
            message,
            loc,
        };
        self.diagnostics.emit(diagnostic);
    }

    pub(crate) fn is_lambda_beginning(&self) -> bool {
        self.lpar_beg == self.paren_nest
    }

    pub(crate) fn tokadd_ident(&mut self, mut c: MaybeByte) -> bool {
        loop {
            if self.tokadd_mbchar(c).is_err() {
                return true;
            }
            c = self.nextc();

            if !self.is_identchar() {
                break;
            }
        }

        self.buffer.pushback(c);
        false
    }

    pub(crate) fn newtok(&mut self) {
        self.buffer.tokidx = 0;
        self.buffer.tokline = self.buffer.ruby_sourceline;
        self.tokenbuf.clear();
    }

    pub(crate) fn literal_flush(&mut self, ptok: usize) {
        self.buffer.set_ptok(ptok);
    }

    pub(crate) fn tokadd_mbchar(&mut self, c: MaybeByte) -> Result<(), ()> {
        let mut len = match self.multibyte_char_len(self.buffer.pcur - 1) {
            Some(len) => len,
            None => return Err(()),
        };

        match c {
            MaybeByte::EndOfInput => return Err(()),
            _ => self.tokadd(c),
        }

        len -= 1;
        self.buffer.pcur += len;
        self.tokcopy(len);
        Ok(())
    }

    fn _multibyte_char_len(&self, ptr: usize) -> Option<usize> {
        let c1 = self.buffer.byte_at(ptr).as_option()?;

        let len = if c1 & 0x80 == 0 {
            1
        } else if c1 & 0xE0 == 0xC0 {
            2
        } else if c1 & 0xF0 == 0xE0 {
            3
        } else if c1 & 0xF8 == 0xF0 {
            4
        } else {
            // malformed
            return None;
        };

        let bytes = self.buffer.substr_at(ptr, ptr + len)?;
        core::str::from_utf8(bytes).ok()?;
        Some(len)
    }

    pub(crate) fn multibyte_char_len(&mut self, ptr: usize) -> Option<usize> {
        let result = self._multibyte_char_len(ptr);
        if result.is_none() {
            self.yyerror0(DiagnosticMessage::InvalidMultibyteChar {});
        }
        result
    }

    pub(crate) fn is_label_suffix(&self, n: usize) -> bool {
        self.buffer.peek_n(b':', n) && !self.buffer.peek_n(b':', n + 1)
    }

    pub(crate) fn is_lvar_defined(&self, name: &str) -> bool {
        self.static_env.is_declared(name)
    }
}
