use crate::Lexer;
use crate::lexer::lex_states::*;
use crate::lexer::lex_char::LexChar;

impl Lexer {
    const NUM_SUFFIX_R: i8 = 1 << 0;
    const NUM_SUFFIX_I: i8 = 1 << 1;
    const NUM_SUFFIX_ALL: i8 = 3;

    pub fn parse_numeric(&mut self, prefix: u8) -> i32 {
        let mut c = LexChar::Some(prefix);

        let mut is_float: bool = false;
        let mut seen_point: Option<usize> = None;
        let mut seen_e: bool = false;
        let mut nondigit: Option<LexChar> = None;
        let suffix: i8;

        self.set_lex_state(EXPR_END);
        self.newtok();
        if c == b'-' || c == b'+' {
            self.tokadd(&c);
            c = self.nextc();
        }
        if c == b'0' {
            let start = self.toklen();
            c = self.nextc();
            if c == b'x' || c == b'X' {
                // hexadecimal
                c = self.nextc();
                if !c.is_eof() && c.is_hexdigit() {
                    loop {
                        if c == b'_' {
                            if nondigit.is_some() { break }
                            nondigit = Some(c.clone());
                            continue;
                        }
                        if !c.is_hexdigit() { break }
                        nondigit = None;
                        self.tokadd(&c);

                        c = self.nextc();
                        if c.is_eof() { break }
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
                return self.set_integer_literal([ "0x".to_owned().into_bytes(), self.tok() ].concat(), suffix);
            }
            if c == b'b' || c == b'B' {
                // binary
                c = self.nextc();
                if c == b'0' || c == b'1' {
                    loop {
                        if c == b'_' {
                            if nondigit.is_some() { break }
                            nondigit = Some(c.clone());
                            continue;
                        }
                        if c != b'0' && c != b'1' { break }
                        nondigit = None;
                        self.tokadd(&c);

                        c = self.nextc();
                        if c.is_eof() { break }
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
                return self.set_integer_literal([ "0b".to_owned().into_bytes(), self.tok() ].concat(), suffix);
            }
            if c == b'd' || c == b'D' {
                // decimal
                c = self.nextc();
                if !c.is_eof() && c.is_digit() {
                    loop {
                        if c == b'_' {
                            if nondigit.is_some() { break }
                            nondigit = Some(c.clone());
                            continue;
                        }
                        if !c.is_digit() { break }
                        nondigit = None;
                        self.tokadd(&c);

                        c = self.nextc();
                        if c.is_eof() { break }
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
                return self.set_integer_literal([ "0d".to_owned().into_bytes(), self.tok() ].concat(), suffix);
            }
            if c == b'_' {
                // 0_0
                if let Some(result) = self.parse_octal(&mut c, &mut nondigit, start) {
                    return result;
                }
            }
            if c == b'o' || c == b'O' {
                // prefixed octal
                c = self.nextc();
                if c.is_eof() || c == b'_' || !c.is_digit() {
                    return self.no_digits();
                }
            }
            if c >= b'0' && c <= b'7' {
                // octal
                if let Some(result) = self.parse_octal(&mut c, &mut nondigit, start) {
                    return result;
                }
            }
            if c > b'7' && c <= b'9' {
                self.invalid_octal();
            } else if c == b'.' || c == b'e' || c == b'E' {
                self.tokadd(&LexChar::Some(b'0'));
            } else {
                self.buffer.pushback(&c);
                suffix = self.number_literal_suffix(Self::NUM_SUFFIX_ALL);
                return self.set_integer_literal("0".to_owned().into_bytes(), suffix);
            }
        }

        loop {
            match c {
                LexChar::Some(b'0') |
                LexChar::Some(b'1') |
                LexChar::Some(b'2') |
                LexChar::Some(b'3') |
                LexChar::Some(b'4') |
                LexChar::Some(b'5') |
                LexChar::Some(b'6') |
                LexChar::Some(b'7') |
                LexChar::Some(b'8') |
                LexChar::Some(b'9') => {
                    nondigit = None;
                    self.tokadd(&c);
                },

                LexChar::Some(b'.') => {
                    if nondigit.is_some() { return self.trailing_uc(&nondigit) }
                    if seen_point.is_some() || seen_e {
                        return self.decode_num(c, nondigit, is_float, seen_e, seen_point);
                    } else {
                        let c0 = self.nextc();
                        if c.is_eof() || !c0.is_digit() {
                            self.buffer.pushback(&c0);
                            return self.decode_num(c, nondigit, is_float, seen_e, seen_point);
                        }
                        c = c0;
                    }
                    seen_point = Some(self.toklen());
                    self.tokadd(&LexChar::Some(b'.'));
                    self.tokadd(&c);
                    is_float = true;
                    nondigit = None;
                },

                LexChar::Some(b'e') |
                LexChar::Some(b'E') => {
                    if let Some(nondigit_value) = &nondigit {
                        self.buffer.pushback(&c);
                        c = nondigit_value.clone();
                        return self.decode_num(c, nondigit, is_float, seen_e, seen_point);
                    }
                    if seen_e {
                        return self.decode_num(c, nondigit, is_float, seen_e, seen_point);
                    }
                    nondigit = Some(c.clone());
                    c = self.nextc();
                    if c != b'-' && c != b'+' && !c.is_digit() {
                        self.buffer.pushback(&c);
                        nondigit = None;
                        return self.decode_num(c, nondigit, is_float, seen_e, seen_point);
                    }
                    self.tokadd(&nondigit.clone().unwrap());
                    seen_e = true;
                    is_float = true;
                    self.tokadd(&c);
                    nondigit = if c == b'-' || c == b'+' { Some(c) } else { None };
                },

                LexChar::Some(b'_') => {
                    if nondigit.is_some() { return self.decode_num(c, nondigit, is_float, seen_e, seen_point); }
                    nondigit = Some(c);
                },

                _ => { return self.decode_num(c, nondigit, is_float, seen_e, seen_point) }
            }

            c = self.nextc();
        }
    }

    pub fn parse_octal(&mut self, c: &mut LexChar, nondigit: &mut Option<LexChar>, start: usize) -> Option<i32> {
        loop {
            if *c == b'_' {
                if nondigit.is_some() { break }
                *nondigit = Some(c.clone());
                continue;
            }
            if *c < b'0' || *c > b'9' { break }
            if *c > b'7' { self.invalid_octal(); return None }
            *nondigit = None;
            self.tokadd(&c);

            *c = self.nextc();
            if c.is_eof() { break }
        }

        if self.toklen() > start {
            self.buffer.pushback(&c);
            self.tokfix();
            if nondigit.is_some() { return Some(self.trailing_uc(&nondigit)) }
            let suffix = self.number_literal_suffix(Self::NUM_SUFFIX_ALL);
            return Some(self.set_integer_literal([ "0".to_owned().into_bytes(), self.tok() ].concat(), suffix));
        }
        if nondigit.is_some() {
            self.buffer.pushback(&c);
            return Some(self.trailing_uc(&nondigit));
        }

        None
    }

    fn invalid_octal(&self) -> i32 {
        // FIXME: yyerror0(...)
        Self::END_OF_INPUT // ("Invalid octal digit".into())
    }

    fn trailing_uc(&mut self, _nondigit: &Option<LexChar>) -> i32 {
        self.literal_flush(self.buffer.pcur - 1);
        // FIXME: compile_error(p, "trailing `%c' in number", nondigit);
        Self::END_OF_INPUT // (format!("trailing `{}' in number", nondigit.clone().unwrap().unwrap()))
    }

    fn decode_num(&mut self, c: LexChar, nondigit: Option<LexChar>, is_float: bool, seen_e: bool, seen_point: Option<usize>) -> i32 {
        self.buffer.pushback(&c);
        if nondigit.is_some() {
            self.trailing_uc(&nondigit);
        }
        return self.parse_numeric_footer(is_float, seen_e, seen_point);
    }

    fn parse_numeric_footer(&mut self, is_float: bool, seen_e: bool, seen_point: Option<usize>) -> i32 {
        self.tokfix();
        if is_float {
            let mut token_type: i32 = Self::tFLOAT;
            let v: Vec<u8>;

            let suffix = self.number_literal_suffix(if seen_e { Self::NUM_SUFFIX_I } else { Self::NUM_SUFFIX_ALL });
            if (suffix & Self::NUM_SUFFIX_R) != 0 {
                let value = [ self.tok(), vec![b'r'] ].concat();
                token_type = Self::tRATIONAL;
                v = self.parse_rational(value, self.toklen(), seen_point);
            } else {
                // we don't parse the number
                v = self.tok();
            }
            return self.set_number_literal(v, token_type, suffix);
        }
        let suffix = self.number_literal_suffix(Self::NUM_SUFFIX_ALL);
        return self.set_integer_literal(self.tok(), suffix);
    }

    fn parse_rational(&mut self, tok: Vec<u8>, _len: usize, _seen_point: Option<usize>) -> Vec<u8> {
        tok
    }

    fn set_number_literal(&mut self, value: Vec<u8>, token_type: i32, suffix: i8) -> i32 {
        let mut token_type = token_type;
        let mut value = value.to_owned();
        if suffix & Self::NUM_SUFFIX_I != 0 {
            value = [ value, vec![b'i'] ].concat();
            token_type = Self::tIMAGINARY;
        }
        self.set_yylval_literal(value);
        self.set_lex_state(EXPR_END);
        token_type
    }

    pub fn no_digits(&mut self) -> i32 {
        self.yyerror0("numeric literal without digits");
        if self.buffer.peek(b'_') { self.nextc(); }
        self.set_integer_literal(vec![b'0'], 0)
    }

    pub fn number_literal_suffix(&mut self, mask: i8) -> i8 {
        let mut c: LexChar;
        let mut mask = mask;
        let mut result: i8 = 0;
        let lastp = self.buffer.pcur;

        loop {
            c = self.nextc();
            if c.is_eof() { break }

            if (mask & Self::NUM_SUFFIX_I != 0) && c == b'i' {
                result |= mask & Self::NUM_SUFFIX_I;
                mask &= !Self::NUM_SUFFIX_I;
                // r after i, rational of complex is disallowed
                mask &= !Self::NUM_SUFFIX_R;
                continue;
            }
            if (mask & Self::NUM_SUFFIX_R != 0) && c == b'r' {
                result |= mask & Self::NUM_SUFFIX_R;
                mask &= !Self::NUM_SUFFIX_R;
                continue;
            }
            if !c.is_ascii() || c.is_alpha() || c == b'_' {
                self.buffer.pcur = lastp;
                // self.literal_flush(self.buffer.pcur);
                return 0;
            }
            self.buffer.pushback(&c);
            break;
        }

        result
    }

    pub fn set_integer_literal(&mut self, value: Vec<u8>, suffix: i8) -> i32 {
        let mut token_type: i32 = Self::tINTEGER;
        let mut value = value.to_owned();
        if suffix & Self::NUM_SUFFIX_R != 0 {
            value = [ vec![b'r'], value ].concat();
            token_type = Self::tRATIONAL;
        }
        self.set_number_literal(value, token_type, suffix)
    }
}
