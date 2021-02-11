use crate::lexer::*;
use crate::maybe_byte::*;
use crate::source::buffer::*;
use crate::DiagnosticMessage;
use crate::{lex_states::*, LexState};

pub(crate) trait ParseGvar {
    fn parse_gvar(&mut self, last_state: LexState) -> i32;
}

impl ParseGvar for Lexer {
    fn parse_gvar(&mut self, last_state: LexState) -> i32 {
        let ptr = self.buffer.pcur;
        let mut c;

        self.lex_state.set(EXPR_END);
        self.buffer.ptok = ptr - 1; // from '$'
        self.newtok();
        c = self.nextc();
        match c.to_option() {
            Some(b'_') => { /* $_: last read line string */
                c = self.nextc();
                if self.is_identchar() {
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
                if self.is_identchar() {
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
                if !self.is_identchar() {
                    match c.to_option() {
                        None | Some(b' ') => self.compile_error(DiagnosticMessage::GvarWithoutId, self.current_loc()),
                        Some(name) => {
                            // The following line comes from MRI, but it seems to be a bug
                            // self.buffer.pushback(&c);
                            self.compile_error(DiagnosticMessage::InvalidGvarName(name), self.current_loc());
                        }
                    }
                    return Self::tGVAR
                }

                self.tokadd(b'$');
            }
        }

        if self.tokadd_ident(&c) {
            return Self::END_OF_INPUT;
        }
        self.lex_state.set(EXPR_END);
        self.tokenize_ident();
        Self::tGVAR
    }
}
