use crate::lexer::*;
use crate::maybe_byte::*;
use crate::reserved_word;
use crate::source::buffer::*;
use crate::{lex_states::*, LexState};

impl Lexer {
    pub(crate) fn parser_is_identchar(&self) -> bool {
        !self.buffer.eofp && self.is_identchar(self.buffer.pcur - 1, self.buffer.pend)
    }

    pub(crate) fn tokenize_ident(&mut self, _last_state: &LexState) -> String {
        self.set_yyval_name();
        match self.tokenbuf.borrow_string() {
            Ok(s) => s.to_owned(),
            Err(bytes) => unreachable!("locals can't have non-utf chars: {:?}", bytes),
        }
    }

    // This method is called is_local_id in MRI, not sure why
    pub(crate) fn is_var_name(&self, ident: &str) -> bool {
        // FIXME: unclear what it can be
        // MRI has some weird logic of comparing given ID with tLAST_OP_ID
        // and then checking & ID_SCOPE_MASK
        if let Some(first_char) = ident.chars().nth(0) {
            return !first_char.is_ascii_uppercase();
        }
        false
    }

    pub(crate) fn parse_ident(&mut self, c: &MaybeByte, cmd_state: bool) -> i32 {
        let mut c = c.clone();
        let mut result: i32;
        let last_state: LexState = self.state.clone();
        let ident: String;

        loop {
            if !c.is_ascii() { /* mb = ENC_CODERANGE_UNKNOWN */ }
            if self.tokadd_mbchar(&c).is_err() {
                return Self::END_OF_INPUT;
            }
            c = self.nextc();

            if !self.parser_is_identchar() {
                break;
            }
        }

        if (c == '!' || c == '?') && !self.buffer.peek(b'=') {
            result = Self::tFID;
            self.tokadd(&c);
        } else if c == '='
            && self.is_lex_state_some(EXPR_FNAME)
            && (!self.buffer.peek(b'~')
                && !self.buffer.peek(b'>')
                && (!self.buffer.peek(b'=') || (self.buffer.peek_n(b'>', 1))))
        {
            result = Self::tIDENTIFIER;
            self.tokadd(&c)
        } else {
            result = Self::tCONSTANT; /* assume provisionally */
            self.buffer.pushback(&c)
        }
        self.tokfix();

        if self.is_label_possible(cmd_state) {
            if self.is_label_suffix(0) {
                self.set_lex_state(EXPR_ARG | EXPR_LABELED);
                self.nextc();
                self.set_yyval_name();
                return Self::tLABEL;
            }
        }
        if
        /* mb == ENC_CODERANGE_7BIT && */
        !self.is_lex_state_some(EXPR_DOT) {
            if let Some(kw) = reserved_word(&self.tokenbuf) {
                let state: LexState = self.state.clone();
                if state.is_some(EXPR_FNAME) {
                    self.set_lex_state(EXPR_ENDFN);
                    self.set_yyval_name();
                    return kw.id.clone();
                }
                self.set_lex_state(kw.state);
                if self.is_lex_state_some(EXPR_BEG) {
                    self.command_start = true
                }
                if kw.id == Self::kDO {
                    if self.is_lambda_beginning() {
                        self.lpar_beg = -1; /* make lambda_beginning_p() == FALSE in the body of "-> do ... end" */
                        return Self::kDO_LAMBDA;
                    }
                    if self.is_cond_active() {
                        return Self::kDO_COND;
                    }
                    if self.is_cmdarg_active() && !state.is_some(EXPR_CMDARG) {
                        return Self::kDO_BLOCK;
                    }
                    return Self::kDO;
                }
                if state.is_some(EXPR_BEG | EXPR_LABELED) {
                    return kw.id.clone();
                } else {
                    if kw.id != kw.modifier_id {
                        self.set_lex_state(EXPR_BEG | EXPR_LABEL)
                    }
                    return kw.modifier_id.clone();
                }
            }
        }

        if self.is_lex_state_some(EXPR_BEG_ANY | EXPR_ARG_ANY | EXPR_DOT) {
            if cmd_state {
                self.set_lex_state(EXPR_CMDARG);
            } else {
                self.set_lex_state(EXPR_ARG);
            }
        } else if self.state.is(EXPR_FNAME) {
            self.set_lex_state(EXPR_ENDFN)
        } else {
            self.set_lex_state(EXPR_END)
        }

        ident = self.tokenize_ident(&last_state);
        if result == Self::tCONSTANT && self.is_var_name(&ident) {
            result = Self::tIDENTIFIER
        }
        if !last_state.is_some(EXPR_DOT|EXPR_FNAME) &&
            result == Self::tIDENTIFIER && /* not EXPR_FNAME, not attrasgn */
            self.is_lvar_defined(&ident)
        {
            self.set_lex_state(EXPR_END | EXPR_LABEL);
        }

        result
    }
}
