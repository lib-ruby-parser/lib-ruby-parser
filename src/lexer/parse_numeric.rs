use crate::lex_states::*;
use crate::lexer::TokAdd;
use crate::maybe_byte::*;
use crate::source::buffer::*;
use crate::Lexer;
use crate::TokenBuf;

impl Lexer {
    const NUM_SUFFIX_R: i8 = 1 << 0;
    const NUM_SUFFIX_I: i8 = 1 << 1;
    const NUM_SUFFIX_ALL: i8 = 3;

    pub(crate) fn parse_numeric(&mut self, prefix: u8) -> i32 {
        let mut c = MaybeByte::new(prefix);

        let mut is_float: bool = false;
        let mut seen_point: Option<usize> = None;
        let mut seen_e: bool = false;
        let mut nondigit: Option<MaybeByte> = None;
        let suffix: i8;

        self.set_lex_state(EXPR_END);
        self.newtok();
        if c == '-' || c == '+' {
            self.tokadd(&c);
            c = self.nextc();
        }
        if c == '0' {
            let start = self.toklen();
            c = self.nextc();
            if c == 'x' || c == 'X' {
                // hexadecimal
                c = self.nextc();
                if !c.is_eof() && c.is_hexdigit() {
                    loop {
                        if c == '_' {
                            if nondigit.is_some() {
                                break;
                            }
                            nondigit = Some(c.clone());
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
                        self.tokadd(&c);

                        c = self.nextc();
                        if c.is_eof() {
                            break;
                        }
                    }
                }
                self.buffer.pushback(&c);
                self.tokfix();
                if self.toklen() == start {
                    return self.no_digits();
                } else if nondigit.is_some() {
                    return self.trailing_uc(&nondigit);
                }
                suffix = self.number_literal_suffix(Self::NUM_SUFFIX_ALL);
                let mut tok = self.tokenbuf.clone();
                tok.prepend("0x");
                return self.set_integer_literal(&mut tok, suffix);
            }
            if c == 'b' || c == 'B' {
                // binary
                c = self.nextc();
                if c == '0' || c == '1' {
                    loop {
                        if c == '_' {
                            if nondigit.is_some() {
                                break;
                            }
                            nondigit = Some(c.clone());
                            continue;
                        }
                        if c != '0' && c != '1' {
                            break;
                        }
                        nondigit = None;
                        self.tokadd(&c);

                        c = self.nextc();
                        if c.is_eof() {
                            break;
                        }
                    }
                }
                self.buffer.pushback(&c);
                self.tokfix();
                if self.toklen() == start {
                    return self.no_digits();
                } else if nondigit.is_some() {
                    return self.trailing_uc(&nondigit);
                }
                suffix = self.number_literal_suffix(Self::NUM_SUFFIX_ALL);
                let mut tok = self.tokenbuf.clone();
                tok.prepend("0b");
                return self.set_integer_literal(&mut tok, suffix);
            }
            if c == 'd' || c == 'D' {
                // decimal
                c = self.nextc();
                if !c.is_eof() && c.is_digit() {
                    loop {
                        if c == '_' {
                            if nondigit.is_some() {
                                break;
                            }
                            nondigit = Some(c.clone());
                            continue;
                        }
                        if !c.is_digit() {
                            break;
                        }
                        nondigit = None;
                        self.tokadd(&c);

                        c = self.nextc();
                        if c.is_eof() {
                            break;
                        }
                    }
                }
                self.buffer.pushback(&c);
                self.tokfix();
                if self.toklen() == start {
                    return self.no_digits();
                } else if nondigit.is_some() {
                    return self.trailing_uc(&nondigit);
                }
                suffix = self.number_literal_suffix(Self::NUM_SUFFIX_ALL);
                let mut tok = self.tokenbuf.clone();
                tok.prepend("0d");
                return self.set_integer_literal(&mut tok, suffix);
            }
            if c == '_' {
                // 0_0
                if let Some(result) = self.parse_octal(&mut c, &mut nondigit, start) {
                    return result;
                }
            }
            if c == 'o' || c == 'O' {
                // prefixed octal
                c = self.nextc();
                if c.is_eof() || c == '_' || !c.is_digit() {
                    return self.no_digits();
                }
            }
            if c >= '0' && c <= '7' {
                // octal
                if let Some(result) = self.parse_octal(&mut c, &mut nondigit, start) {
                    return result;
                }
            }
            if c > '7' && c <= '9' {
                self.invalid_octal();
            } else if c == '.' || c == 'e' || c == 'E' {
                self.tokadd(b'0');
            } else {
                self.buffer.pushback(&c);
                suffix = self.number_literal_suffix(Self::NUM_SUFFIX_ALL);
                return self.set_integer_literal(&mut TokenBuf::new(b"0"), suffix);
            }
        }

        loop {
            match c.to_option() {
                Some(b'0') | Some(b'1') | Some(b'2') | Some(b'3') | Some(b'4') | Some(b'5')
                | Some(b'6') | Some(b'7') | Some(b'8') | Some(b'9') => {
                    nondigit = None;
                    self.tokadd(&c);
                }

                Some(b'.') => {
                    if nondigit.is_some() {
                        return self.trailing_uc(&nondigit);
                    }
                    if seen_point.is_some() || seen_e {
                        return self.decode_num(c, nondigit, is_float, seen_e);
                    } else {
                        let c0 = self.nextc();
                        if c.is_eof() || !c0.is_digit() {
                            self.buffer.pushback(&c0);
                            return self.decode_num(c, nondigit, is_float, seen_e);
                        }
                        c = c0;
                    }
                    seen_point = Some(self.toklen());
                    self.tokadd(b'.');
                    self.tokadd(&c);
                    is_float = true;
                    nondigit = None;
                }

                Some(b'e') | Some(b'E') => {
                    if let Some(nondigit_value) = &nondigit {
                        self.buffer.pushback(&c);
                        c = nondigit_value.clone();
                        return self.decode_num(c, nondigit, is_float, seen_e);
                    }
                    if seen_e {
                        return self.decode_num(c, nondigit, is_float, seen_e);
                    }
                    nondigit = Some(c.clone());
                    c = self.nextc();
                    if c != '-' && c != '+' && !c.is_digit() {
                        self.buffer.pushback(&c);
                        nondigit = None;
                        return self.decode_num(c, nondigit, is_float, seen_e);
                    }
                    self.tokadd(&nondigit.clone().unwrap());
                    seen_e = true;
                    is_float = true;
                    self.tokadd(&c);
                    nondigit = if c == '-' || c == '+' { Some(c) } else { None };
                }

                Some(b'_') => {
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

    pub(crate) fn parse_octal(
        &mut self,
        c: &mut MaybeByte,
        nondigit: &mut Option<MaybeByte>,
        start: usize,
    ) -> Option<i32> {
        loop {
            if *c == '_' {
                if nondigit.is_some() {
                    break;
                }
                *nondigit = Some(c.clone());
                continue;
            }
            if *c < '0' || *c > '9' {
                break;
            }
            if *c > '7' {
                self.invalid_octal();
                return None;
            }
            *nondigit = None;
            self.tokadd(&*c);

            *c = self.nextc();
            if c.is_eof() {
                break;
            }
        }

        if self.toklen() > start {
            self.buffer.pushback(c);
            self.tokfix();
            if nondigit.is_some() {
                return Some(self.trailing_uc(&nondigit));
            }
            let suffix = self.number_literal_suffix(Self::NUM_SUFFIX_ALL);
            let mut tok = self.tokenbuf.clone();
            tok.prepend("0");
            return Some(self.set_integer_literal(&mut tok, suffix));
        }
        if nondigit.is_some() {
            self.buffer.pushback(c);
            return Some(self.trailing_uc(&nondigit));
        }

        None
    }

    fn invalid_octal(&self) -> i32 {
        // FIXME: yyerror0(...)
        Self::END_OF_INPUT // ("Invalid octal digit".into())
    }

    fn trailing_uc(&mut self, _nondigit: &Option<MaybeByte>) -> i32 {
        self.literal_flush(self.buffer.pcur - 1);
        // FIXME: compile_error(p, "trailing `%c' in number", nondigit);
        Self::END_OF_INPUT // (format!("trailing `{}' in number", nondigit.clone().unwrap().unwrap()))
    }

    fn decode_num(
        &mut self,
        c: MaybeByte,
        nondigit: Option<MaybeByte>,
        is_float: bool,
        seen_e: bool,
    ) -> i32 {
        self.buffer.pushback(&c);
        if nondigit.is_some() {
            self.trailing_uc(&nondigit);
        }
        return self.parse_numeric_footer(is_float, seen_e);
    }

    fn parse_numeric_footer(&mut self, is_float: bool, seen_e: bool) -> i32 {
        self.tokfix();
        if is_float {
            let mut token_type: i32 = Self::tFLOAT;
            let mut tokenbuf;

            let suffix = self.number_literal_suffix(if seen_e {
                Self::NUM_SUFFIX_I
            } else {
                Self::NUM_SUFFIX_ALL
            });
            if (suffix & Self::NUM_SUFFIX_R) != 0 {
                let mut value = self.tokenbuf.clone();
                value.push(b'r');
                token_type = Self::tRATIONAL;
                tokenbuf = value
            } else {
                tokenbuf = self.tokenbuf.clone();
            }
            // we don't parse the number
            return self.set_number_literal(&mut tokenbuf, token_type, suffix);
        }
        let suffix = self.number_literal_suffix(Self::NUM_SUFFIX_ALL);
        return self.set_integer_literal(&mut self.tokenbuf.clone(), suffix);
    }

    fn set_number_literal(&mut self, value: &mut TokenBuf, token_type: i32, suffix: i8) -> i32 {
        let mut token_type = token_type;
        if suffix & Self::NUM_SUFFIX_I != 0 {
            value.push(b'i');
            token_type = Self::tIMAGINARY;
        }
        self.set_yylval_literal(value);
        self.set_lex_state(EXPR_END);
        token_type
    }

    pub(crate) fn no_digits(&mut self) -> i32 {
        self.yyerror0("numeric literal without digits");
        if self.buffer.peek(b'_') {
            self.nextc();
        }
        self.set_integer_literal(&mut TokenBuf::new(b"0"), 0)
    }

    pub(crate) fn number_literal_suffix(&mut self, mask: i8) -> i8 {
        let mut c: MaybeByte;
        let mut mask = mask;
        let mut result: i8 = 0;
        let lastp = self.buffer.pcur;

        loop {
            c = self.nextc();
            if c.is_eof() {
                break;
            }

            if (mask & Self::NUM_SUFFIX_I != 0) && c == 'i' {
                result |= mask & Self::NUM_SUFFIX_I;
                mask &= !Self::NUM_SUFFIX_I;
                // r after i, rational of complex is disallowed
                mask &= !Self::NUM_SUFFIX_R;
                continue;
            }
            if (mask & Self::NUM_SUFFIX_R != 0) && c == 'r' {
                result |= mask & Self::NUM_SUFFIX_R;
                mask &= !Self::NUM_SUFFIX_R;
                continue;
            }
            if !c.is_ascii() || c.is_alpha() || c == '_' {
                self.buffer.pcur = lastp;
                // self.literal_flush(self.buffer.pcur);
                return 0;
            }
            self.buffer.pushback(&c);
            break;
        }

        result
    }

    pub(crate) fn set_integer_literal(&mut self, value: &mut TokenBuf, suffix: i8) -> i32 {
        let mut token_type = Self::tINTEGER;
        if suffix & Self::NUM_SUFFIX_R != 0 {
            value.push(b'r');
            token_type = Self::tRATIONAL;
        }
        self.set_number_literal(value, token_type, suffix)
    }
}
