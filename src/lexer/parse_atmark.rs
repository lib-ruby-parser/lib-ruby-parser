use crate::lexer::*;
use crate::source::buffer::*;
use crate::{lex_states::*, LexState};
use lib_ruby_parser_ast::DiagnosticMessage;

impl<'b> Lexer<'b> {
    pub(crate) fn parse_atmark(&mut self, last_state: LexState) -> i32 {
        let ptr = self.buffer.pcur;
        let mut result: i32 = Self::tIVAR;
        let mut c = self.nextc();

        self.buffer.ptok = ptr - 1; // from '@'
        self.newtok();
        self.tokadd(b'@');
        if c == b'@' {
            result = Self::tCVAR;
            self.tokadd(b'@');
            c = self.nextc()
        }
        self.lex_state.set(if last_state.is_some(EXPR_FNAME) {
            EXPR_ENDFN
        } else {
            EXPR_END
        });
        if c.is_eof() || !self.is_identchar() {
            self.buffer.pushback(c);
            if result == Self::tIVAR {
                self.compile_error(DiagnosticMessage::IvarWithoutId {}, self.current_loc());
            } else {
                self.compile_error(DiagnosticMessage::CvarWithoutId {}, self.current_loc());
            }
            self.lex_state.set(EXPR_END);
            return result;
        } else if c.is_digit() {
            // The following line comes from MRI, but it seems to be a bug
            // self.buffer.pushback(&c);
            if result == Self::tIVAR {
                self.compile_error(
                    DiagnosticMessage::InvalidIvarName {
                        c: c.expect("c is a digit"),
                    },
                    self.current_loc(),
                );
            } else {
                self.compile_error(
                    DiagnosticMessage::InvalidCvarName {
                        c: c.expect("c is a digit"),
                    },
                    self.current_loc(),
                );
            }
            self.lex_state.set(EXPR_END);
            return result;
        }

        if self.tokadd_ident(c) {
            return Self::END_OF_INPUT;
        }
        self.tokenize_ident();
        result
    }
}
