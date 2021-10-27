use crate::lexer::*;
use crate::maybe_byte::*;
use crate::source::buffer::*;
use crate::str_term::str_types::*;
use crate::DiagnosticMessage;
use crate::{lex_states::*, LexState};

impl Lexer {
    fn percent_unknown(&mut self, term: MaybeByte) -> i32 {
        self.buffer.pushback(term);
        let len = self.multibyte_char_len(self.buffer.pcur);
        match len {
            Some(len) => self.buffer.pcur += len,
            None => return Self::END_OF_INPUT,
        }
        self.yyerror1(
            DiagnosticMessage::UnknownTypeOfPercentString {},
            self.current_loc().adjust_end(-1),
        );
        Self::END_OF_INPUT
    }

    fn percent_quotation(&mut self, c: &mut MaybeByte, ptok: usize) -> i32 {
        let term: MaybeByte;
        let mut paren: Option<u8>;

        if c.is_eof() || !c.is_alnum() {
            term = *c;
            if !c.is_ascii() {
                return self.percent_unknown(term);
            }
            *c = MaybeByte::new(b'Q');
        } else {
            term = self.nextc();
            if term.is_alnum() {
                return self.percent_unknown(term);
            }
        }

        let mut term = match term.as_option() {
            None => {
                self.compile_error(
                    DiagnosticMessage::UnterminatedQuotedString {},
                    self.current_loc(),
                );
                return Self::END_OF_INPUT;
            }
            Some(term) => term,
        };

        paren = Some(term);
        if term == b'(' {
            term = b')';
        } else if term == b'[' {
            term = b']';
        } else if term == b'{' {
            term = b'}';
        } else if term == b'<' {
            term = b'>';
        } else {
            paren = None
        }

        self.buffer.ptok = ptok - 1;
        match c.as_option() {
            Some(b'Q') => {
                self.strterm = self.new_strterm(str_dquote, term, paren, None);
                Self::tSTRING_BEG
            }
            Some(b'q') => {
                self.strterm = self.new_strterm(str_squote, term, paren, None);
                Self::tSTRING_BEG
            }
            Some(b'W') => {
                self.strterm = self.new_strterm(str_dword, term, paren, None);
                Self::tWORDS_BEG
            }
            Some(b'w') => {
                self.strterm = self.new_strterm(str_sword, term, paren, None);
                Self::tQWORDS_BEG
            }
            Some(b'I') => {
                self.strterm = self.new_strterm(str_dword, term, paren, None);
                Self::tSYMBOLS_BEG
            }
            Some(b'i') => {
                self.strterm = self.new_strterm(str_sword, term, paren, None);
                Self::tQSYMBOLS_BEG
            }
            Some(b'x') => {
                self.strterm = self.new_strterm(str_xquote, term, paren, None);
                Self::tXSTRING_BEG
            }
            Some(b'r') => {
                self.strterm = self.new_strterm(str_regexp, term, paren, None);
                Self::tREGEXP_BEG
            }
            Some(b's') => {
                self.strterm = self.new_strterm(str_ssym, term, paren, None);
                self.lex_state.set(EXPR_FNAME | EXPR_FITEM);
                Self::tSYMBEG
            }
            _ => {
                self.yyerror1(
                    DiagnosticMessage::UnknownTypeOfPercentString {},
                    self.current_loc().adjust_end(-1),
                );
                Self::END_OF_INPUT
            }
        }
    }

    pub(crate) fn parse_percent(&mut self, space_seen: bool, last_state: LexState) -> i32 {
        let mut c: MaybeByte;
        let ptok = self.buffer.pcur;

        if self.lex_state.is_beg() {
            c = self.nextc();
            return self.percent_quotation(&mut c, ptok);
        }

        c = self.nextc();
        if c == b'=' {
            self.set_yylval_id("%=");
            self.lex_state.set(EXPR_BEG);
            return Self::tOP_ASGN;
        }
        if self.lex_state.is_spacearg(c, space_seen)
            || (self.lex_state.is_some(EXPR_FITEM) && c == b's')
        {
            return self.percent_quotation(&mut c, ptok);
        }
        self.lex_state.set(if self.lex_state.is_after_operator() {
            EXPR_ARG
        } else {
            EXPR_BEG
        });
        self.buffer.pushback(c);
        self.warn_balanced(
            Self::tPERCENT,
            "%%",
            "string literal",
            c,
            space_seen,
            last_state,
        )
    }
}
