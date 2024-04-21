use crate::lex_states::*;
use crate::lexer::*;
use crate::maybe_byte::*;
use crate::reserved_word;
use crate::source::buffer::*;
use crate::DiagnosticMessage;

fn is_var_name(ident: &str) -> bool {
    if let Some(first_char) = ident.chars().next() {
        return !first_char.is_uppercase();
    }
    false
}

impl<'b, 'i> Lexer<'b, 'i> {
    pub(crate) fn is_identchar(&self) -> bool {
        !self.buffer.eofp
            && self
                .buffer
                .is_identchar(self.buffer.pcur - 1, self.buffer.pend)
    }

    pub(crate) fn tokenize_ident(&mut self) -> Option<String> {
        self.set_yylval_name();
        self.tokenbuf.borrow_string().map(|s| s.to_string()).ok()
    }

    pub(crate) fn parse_ident(&mut self, mut c: MaybeByte, cmd_state: bool) -> i32 {
        let mut result: i32;
        let last_state = self.lex_state;

        loop {
            if !c.is_ascii() { /* mb = ENC_CODERANGE_UNKNOWN */ }
            if self.tokadd_mbchar(c).is_err() {
                return Self::END_OF_INPUT;
            }
            c = self.nextc();

            if !self.is_identchar() {
                break;
            }
        }

        if (c == b'!' || c == b'?') && !self.buffer.peek(b'=') {
            result = Self::tFID;
            self.tokadd(c);
        } else if c == b'='
            && self.lex_state.is_some(EXPR_FNAME)
            && (!self.buffer.peek(b'~')
                && !self.buffer.peek(b'>')
                && (!self.buffer.peek(b'=') || (self.buffer.peek_n(b'>', 1))))
        {
            result = Self::tIDENTIFIER;
            self.tokadd(c)
        } else {
            result = Self::tCONSTANT; /* assume provisionally */
            self.buffer.pushback(c)
        }
        self.tokfix();

        if self.lex_state.is_label_possible(cmd_state) && self.is_label_suffix(0) {
            self.lex_state.set(EXPR_ARG | EXPR_LABELED);
            self.nextc();
            self.set_yylval_name();
            return Self::tLABEL;
        }
        if !self.lex_state.is_some(EXPR_DOT) {
            if let Some(kw) = reserved_word(self.tokenbuf.bytes.as_raw()) {
                let state = self.lex_state;
                if state.is_some(EXPR_FNAME) {
                    self.lex_state.set(EXPR_ENDFN);
                    self.set_yylval_name();
                    return kw.id;
                }
                self.lex_state.set(kw.state);
                if self.lex_state.is_some(EXPR_BEG) {
                    self.command_start = true
                }
                if kw.id == Self::kDO {
                    if self.is_lambda_beginning() {
                        self.lpar_beg = -1; /* make lambda_beginning_p() == FALSE in the body of "-> do ... end" */
                        return Self::kDO_LAMBDA;
                    }
                    if self.cond.is_active() {
                        return Self::kDO_COND;
                    }
                    if self.cmdarg.is_active() && !state.is_some(EXPR_CMDARG) {
                        return Self::kDO_BLOCK;
                    }
                    return Self::kDO;
                }
                if state.is_some(EXPR_BEG | EXPR_LABELED) {
                    return kw.id;
                } else {
                    if kw.id != kw.modifier_id {
                        self.lex_state.set(EXPR_BEG | EXPR_LABEL)
                    }
                    return kw.modifier_id;
                }
            }
        }

        if self
            .lex_state
            .is_some(EXPR_BEG_ANY | EXPR_ARG_ANY | EXPR_DOT)
        {
            if cmd_state {
                self.lex_state.set(EXPR_CMDARG);
            } else {
                self.lex_state.set(EXPR_ARG);
            }
        } else if self.lex_state.is(EXPR_FNAME) {
            self.lex_state.set(EXPR_ENDFN)
        } else {
            self.lex_state.set(EXPR_END)
        }

        let ident = match self.tokenize_ident() {
            Some(ident) => ident,
            None => {
                self.yyerror0(DiagnosticMessage::InvalidMultibyteChar {});
                return Self::END_OF_INPUT;
            }
        };
        if result == Self::tCONSTANT && is_var_name(&ident) {
            result = Self::tIDENTIFIER
        }
        if !last_state.is_some(EXPR_DOT|EXPR_FNAME) &&
            result == Self::tIDENTIFIER && /* not EXPR_FNAME, not attrasgn */
            self.is_lvar_defined(&ident)
        {
            self.lex_state.set(EXPR_END | EXPR_LABEL);
        }

        result
    }
}
