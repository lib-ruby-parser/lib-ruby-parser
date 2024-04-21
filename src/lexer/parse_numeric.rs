use crate::lexer::TokAdd;
use crate::maybe_byte::*;
use crate::source::buffer::*;
use crate::Lexer;
use crate::TokenBuf;
use crate::{lex_states::*, DiagnosticMessage};

const NUM_SUFFIX_R: i8 = 1 << 0;
const NUM_SUFFIX_I: i8 = 1 << 1;
const NUM_SUFFIX_ALL: i8 = 3;

impl<'b, 'i> Lexer<'b, 'i> {
    pub(crate) fn parse_numeric(&mut self, prefix: u8) -> i32 {
        let mut c = MaybeByte::new(prefix);

        let mut is_float: bool = false;
        let mut seen_point: Option<usize> = None;
        let mut seen_e: bool = false;
        let mut nondigit: Option<MaybeByte> = None;
        let suffix: i8;

        self.lex_state.set(EXPR_END);
        self.newtok();
        if c == b'-' || c == b'+' {
            self.tokadd(c);
            c = self.nextc();
        }
        if c == b'0' {
            let start = self.toklen();
            c = self.nextc();
            if c == b'x' || c == b'X' {
                // hexadecimal
                self.tokadd(c);
                c = self.nextc();
                if !c.is_eof() && c.is_hexdigit() {
                    loop {
                        if c == b'_' {
                            if nondigit.is_some() {
                                break;
                            }
                            nondigit = Some(c);
                            self.tokadd(c);
                            c = self.nextc();
                            if c.is_eof() {
                                break;
                            }
                            continue;
                        }
                        if !c.is_hexdigit() {
                            break;
                        }
                        nondigit = None;
                        self.tokadd(c);

                        c = self.nextc();
                        if c.is_eof() {
                            break;
                        }
                    }
                }
                self.buffer.pushback(c);
                self.tokfix();
                if self.toklen() == start + 1 {
                    return self.no_digits();
                } else if let Some(MaybeByte::Some(byte)) = nondigit {
                    return self.trailing_uc(byte);
                }
                suffix = self.number_literal_suffix(NUM_SUFFIX_ALL);
                let mut tok = self.tokenbuf.take();
                tok.prepend_valid_escaped('0');
                return self.set_integer_literal(&mut tok, suffix);
            }
            if c == b'b' || c == b'B' {
                // binary
                self.tokadd(c);
                c = self.nextc();
                if c == b'0' || c == b'1' {
                    loop {
                        if c == b'_' {
                            if nondigit.is_some() {
                                break;
                            }
                            nondigit = Some(c);
                            self.tokadd(c);
                            c = self.nextc();
                            if c.is_eof() {
                                break;
                            }
                            continue;
                        }
                        if c != b'0' && c != b'1' {
                            break;
                        }
                        nondigit = None;
                        self.tokadd(c);

                        c = self.nextc();
                        if c.is_eof() {
                            break;
                        }
                    }
                }
                self.buffer.pushback(c);
                self.tokfix();
                if self.toklen() == start + 1 {
                    return self.no_digits();
                } else if let Some(MaybeByte::Some(byte)) = nondigit {
                    return self.trailing_uc(byte);
                }
                suffix = self.number_literal_suffix(NUM_SUFFIX_ALL);
                let mut tok = self.tokenbuf.take();
                tok.prepend_valid_escaped('0');
                return self.set_integer_literal(&mut tok, suffix);
            }
            if c == b'd' || c == b'D' {
                // decimal
                self.tokadd(c);
                c = self.nextc();
                if !c.is_eof() && c.is_digit() {
                    loop {
                        if c == b'_' {
                            if nondigit.is_some() {
                                break;
                            }
                            nondigit = Some(c);
                            self.tokadd(c);
                            c = self.nextc();
                            if c.is_eof() {
                                break;
                            }
                            continue;
                        }
                        if !c.is_digit() {
                            break;
                        }
                        nondigit = None;
                        self.tokadd(c);

                        c = self.nextc();
                        if c.is_eof() {
                            break;
                        }
                    }
                }
                self.buffer.pushback(c);
                self.tokfix();
                if self.toklen() == start + 1 {
                    return self.no_digits();
                } else if let Some(MaybeByte::Some(byte)) = nondigit {
                    return self.trailing_uc(byte);
                }
                suffix = self.number_literal_suffix(NUM_SUFFIX_ALL);
                let mut tok = self.tokenbuf.take();
                tok.prepend_valid_escaped('0');
                return self.set_integer_literal(&mut tok, suffix);
            }
            if c == b'_' {
                // 0_0
                if let Some(result) = self.parse_octal(&mut c, &mut nondigit, start) {
                    return result;
                }
            }
            if c == b'o' || c == b'O' {
                self.tokadd(c);
                // prefixed octal
                c = self.nextc();
                if c.is_eof() || c == b'_' || !c.is_digit() {
                    return self.no_digits();
                }
            }
            // `c` here is a MaybeByte that implements PartialOrd<u8>
            #[allow(clippy::manual_range_contains)]
            if c >= b'0' && c <= b'7' {
                // octal
                if let Some(result) = self.parse_octal(&mut c, &mut nondigit, start) {
                    return result;
                }
            }
            if c > b'7' && c <= b'9' {
                self.invalid_octal();
            } else if c == b'.' || c == b'e' || c == b'E' {
                self.tokadd(b'0');
            } else {
                self.buffer.pushback(c);
                suffix = self.number_literal_suffix(NUM_SUFFIX_ALL);

                let mut tok = self.tokenbuf.take();
                tok.append_valid_escaped('0');
                return self.set_integer_literal(&mut tok, suffix);
            }
        }

        loop {
            match c.as_option() {
                Some(b'0') | Some(b'1') | Some(b'2') | Some(b'3') | Some(b'4') | Some(b'5')
                | Some(b'6') | Some(b'7') | Some(b'8') | Some(b'9') => {
                    nondigit = None;
                    self.tokadd(c);
                }

                Some(b'.') => {
                    if let Some(MaybeByte::Some(byte)) = nondigit {
                        return self.trailing_uc(byte);
                    }
                    if seen_point.is_some() || seen_e {
                        return self.decode_num(c, nondigit, is_float, seen_e);
                    } else {
                        let c0 = self.nextc();
                        if c.is_eof() || !c0.is_digit() {
                            self.buffer.pushback(c0);
                            return self.decode_num(c, nondigit, is_float, seen_e);
                        }
                        c = c0;
                    }
                    seen_point = Some(self.toklen());
                    self.tokadd(b'.');
                    self.tokadd(c);
                    is_float = true;
                    nondigit = None;
                }

                Some(b'e') | Some(b'E') => {
                    if let Some(nondigit_value) = &nondigit {
                        self.buffer.pushback(c);
                        c = *nondigit_value;
                        return self.decode_num(c, nondigit, is_float, seen_e);
                    }
                    if seen_e {
                        return self.decode_num(c, nondigit, is_float, seen_e);
                    }
                    nondigit = Some(c);
                    c = self.nextc();
                    if c != b'-' && c != b'+' && !c.is_digit() {
                        self.buffer.pushback(c);
                        nondigit = None;
                        return self.decode_num(c, nondigit, is_float, seen_e);
                    }
                    self.tokadd(nondigit.expect("nondigit must be set"));
                    seen_e = true;
                    is_float = true;
                    self.tokadd(c);
                    nondigit = if c == b'-' || c == b'+' {
                        Some(c)
                    } else {
                        None
                    };
                }

                Some(b'_') => {
                    self.tokadd(c);
                    if nondigit.is_some() {
                        return self.decode_num(c, nondigit, is_float, seen_e);
                    }
                    nondigit = Some(c);
                }

                _ => return self.decode_num(c, nondigit, is_float, seen_e),
            }

            c = self.nextc();
        }
    }

    fn parse_octal(
        &mut self,
        c: &mut MaybeByte,
        nondigit: &mut Option<MaybeByte>,
        start: usize,
    ) -> Option<i32> {
        loop {
            if *c == b'_' {
                if nondigit.is_some() {
                    break;
                }
                *nondigit = Some(*c);
                self.tokadd(*c);
                *c = self.nextc();
                if c.is_eof() {
                    break;
                }
                continue;
            }
            if *c < b'0' || *c > b'9' {
                break;
            }
            if *c > b'7' {
                self.invalid_octal();
                return None;
            }
            *nondigit = None;
            self.tokadd(*c);

            *c = self.nextc();
            if c.is_eof() {
                break;
            }
        }

        if self.toklen() > start {
            self.buffer.pushback(c);
            self.tokfix();
            if let Some(MaybeByte::Some(byte)) = nondigit {
                return Some(self.trailing_uc(*byte));
            }
            let suffix = self.number_literal_suffix(NUM_SUFFIX_ALL);
            let mut tok = self.tokenbuf.take();
            tok.prepend_valid_escaped('0');
            return Some(self.set_integer_literal(&mut tok, suffix));
        }
        if let Some(MaybeByte::Some(byte)) = nondigit {
            self.buffer.pushback(c);
            return Some(self.trailing_uc(*byte));
        }

        None
    }

    fn invalid_octal(&mut self) -> i32 {
        self.yyerror0(DiagnosticMessage::InvalidOctalDigit {});
        Self::END_OF_INPUT
    }

    fn trailing_uc(&mut self, nondigit: u8) -> i32 {
        self.literal_flush(self.buffer.pcur - 1);
        self.yyerror0(DiagnosticMessage::TrailingCharInNumber { c: nondigit });
        Self::END_OF_INPUT
    }

    fn decode_num(
        &mut self,
        c: MaybeByte,
        nondigit: Option<MaybeByte>,
        is_float: bool,
        seen_e: bool,
    ) -> i32 {
        self.buffer.pushback(c);
        if let Some(MaybeByte::Some(byte)) = nondigit {
            self.trailing_uc(byte);
        }
        self.parse_numeric_footer(is_float, seen_e)
    }

    fn parse_numeric_footer(&mut self, is_float: bool, seen_e: bool) -> i32 {
        self.tokfix();
        if is_float {
            let mut token_type: i32 = Self::tFLOAT;

            let suffix =
                self.number_literal_suffix(if seen_e { NUM_SUFFIX_I } else { NUM_SUFFIX_ALL });
            let mut tokenbuf = if (suffix & NUM_SUFFIX_R) != 0 {
                let mut value = self.tokenbuf.take();
                value.append_valid_escaped('r');
                token_type = Self::tRATIONAL;
                value
            } else {
                self.tokenbuf.take()
            };
            // we don't parse the number
            return self.set_number_literal(&mut tokenbuf, token_type, suffix);
        }
        let suffix = self.number_literal_suffix(NUM_SUFFIX_ALL);
        let mut tokenbuf = self.tokenbuf.take();
        self.set_integer_literal(&mut tokenbuf, suffix)
    }

    fn set_number_literal(&mut self, value: &mut TokenBuf, token_type: i32, suffix: i8) -> i32 {
        let mut token_type = token_type;
        if suffix & NUM_SUFFIX_I != 0 {
            value.append_valid_escaped('i');
            token_type = Self::tIMAGINARY;
        }
        self.set_yylval_literal(value);
        self.lex_state.set(EXPR_END);
        token_type
    }

    fn no_digits(&mut self) -> i32 {
        self.yyerror0(DiagnosticMessage::NumericLiteralWithoutDigits {});
        if self.buffer.peek(b'_') {
            self.nextc();
        }
        let mut token_buf = TokenBuf::empty(self.blob);
        token_buf.append_valid_escaped('0');
        self.set_integer_literal(&mut token_buf, 0)
    }

    fn number_literal_suffix(&mut self, mask: i8) -> i8 {
        let mut c: MaybeByte;
        let mut mask = mask;
        let mut result: i8 = 0;
        let lastp = self.buffer.pcur;

        loop {
            c = self.nextc();
            if c.is_eof() {
                break;
            }

            if (mask & NUM_SUFFIX_I != 0) && c == b'i' {
                result |= mask & NUM_SUFFIX_I;
                mask &= !NUM_SUFFIX_I;
                // r after i, rational of complex is disallowed
                mask &= !NUM_SUFFIX_R;
                continue;
            }
            if (mask & NUM_SUFFIX_R != 0) && c == b'r' {
                result |= mask & NUM_SUFFIX_R;
                mask &= !NUM_SUFFIX_R;
                continue;
            }
            if !c.is_ascii() || c.is_alpha() || c == b'_' {
                self.buffer.pcur = lastp;
                // self.literal_flush(self.buffer.pcur);
                return 0;
            }
            self.buffer.pushback(c);
            break;
        }

        result
    }

    fn set_integer_literal(&mut self, value: &mut TokenBuf, suffix: i8) -> i32 {
        let mut token_type = Self::tINTEGER;
        if suffix & NUM_SUFFIX_R != 0 {
            value.append_valid_escaped('r');
            token_type = Self::tRATIONAL;
        }
        self.set_number_literal(value, token_type, suffix)
    }
}
