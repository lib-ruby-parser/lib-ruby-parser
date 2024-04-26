use lib_ruby_parser_ast::{write_to, ByteArray};

use crate::maybe_byte::*;
use crate::source::buffer::*;
use crate::str_term::{str_types::*, StrTerm};
use crate::TokenBuf;
use crate::{lex_states::*, DiagnosticMessage};
use crate::{lexer::*, str_term::StringLiteral};

const ESCAPE_CONTROL: u32 = 1;
const ESCAPE_META: u32 = 2;

impl<'b> Lexer<'b> {
    fn take_strterm(&mut self) -> StringLiteral<'b> {
        match self.strterm.take() {
            Some(StrTerm::StringLiteral(s)) => s,
            _ => unreachable!("strterm must be string"),
        }
    }
    fn restore_strterm(&mut self, literal: StringLiteral<'b>) {
        self.strterm = Some(StrTerm::StringLiteral(literal));
    }

    pub(crate) fn parse_string(&mut self) -> i32 {
        let mut quote = self.take_strterm();

        let func = quote.func;
        let term = quote.term;
        let paren = quote.paren;
        let mut space = false;
        self.lval_start = Some(self.buffer.pcur);

        println_if_debug_lexer!(
            "parse_string: func = {}, pcur = {}, ptok = {}, term = {}",
            func,
            self.buffer.pcur,
            self.buffer.ptok,
            quote.term
        );

        if (func & STR_FUNC_TERM) != 0 {
            if (func & STR_FUNC_QWORDS) != 0 {
                self.nextc();
            } /* delayed term */
            self.lex_state.set(EXPR_END);
            self.strterm = None;
            if (func & STR_FUNC_REGEXP) != 0 {
                return Self::tREGEXP_END;
            } else {
                if let Some(heredoc_end) = quote.heredoc_end {
                    self.lval_start = Some(heredoc_end.start);
                    self.lval_end = Some(heredoc_end.end);
                    let mut token_buf = TokenBuf::empty(self.blob);
                    token_buf.push_bytes(heredoc_end.value);
                    self.set_yylval_str(&mut token_buf);
                }
                return Self::tSTRING_END;
            }
        }
        let mut c = self.nextc();
        if (func & STR_FUNC_QWORDS) != 0 && c.is_space() {
            loop {
                c = self.nextc();

                if !c.is_space() {
                    break;
                }
            }
            space = true;
        }
        if (func & STR_FUNC_LIST) != 0 {
            quote.func &= !STR_FUNC_LIST;
            space = true;
        }
        if c == term && quote.nest == 0 {
            if (func & STR_FUNC_QWORDS) != 0 {
                quote.func |= STR_FUNC_TERM;
                self.buffer.pushback(c); /* dispatch the term at tSTRING_END */
                self.restore_strterm(quote);
                return Self::tSPACE;
            }
            self.restore_strterm(quote);
            return self.string_term(term, func);
        }
        if space {
            self.buffer.pushback(c);
            self.restore_strterm(quote);
            return Self::tSPACE;
        }
        self.newtok();
        if ((func & STR_FUNC_EXPAND) != 0) && c == b'#' {
            if let Some(t) = self.peek_variable_name() {
                self.restore_strterm(quote);
                return t;
            }
            self.tokadd(b'#');
            c = self.nextc();
        }
        self.buffer.pushback(c);

        let mut nest = quote.nest;
        let added = self.tokadd_string(func, term, paren, &mut nest);
        quote.nest = nest;

        if added.is_some() && self.buffer.eofp {
            self.literal_flush(self.buffer.pcur);
            if (func & STR_FUNC_QWORDS) != 0 {
                /* no content to add, bailing out here */
                self.yyerror0(DiagnosticMessage::UnterminatedList {});
                self.strterm = None;
                return Self::tSTRING_END;
            }
            if (func & STR_FUNC_REGEXP) != 0 {
                self.yyerror0(DiagnosticMessage::UnterminatedRegexp {});
            } else {
                self.yyerror0(DiagnosticMessage::UnterminatedString {});
            }
            quote.func |= STR_FUNC_TERM;
        }

        self.tokfix();
        let mut str_ = self.tokenbuf.take();
        self.set_yylval_str(&mut str_);
        self.flush_string_content();
        self.restore_strterm(quote);

        Self::tSTRING_CONTENT
    }

    fn string_term(&mut self, term: u8, func: u32) -> i32 {
        self.strterm = None;
        if (func & STR_FUNC_REGEXP) != 0 {
            let (flags, flags_count) = self.regx_options();
            let mut mem = [0; 11];
            mem[0] = term;
            mem[1..(flags_count + 1)].copy_from_slice(&flags[..flags_count]);
            let flags = core::str::from_utf8(&mem[..flags_count + 1]).unwrap();
            self.set_yylval_num(flags);
            self.lex_state.set(EXPR_END);
            return Self::tREGEXP_END;
        }
        if (func & STR_FUNC_LABEL) != 0 && self.is_label_suffix(0) {
            self.nextc();
            self.lex_state.set(EXPR_BEG | EXPR_LABEL);
            return Self::tLABEL_END;
        }
        self.lex_state.set(EXPR_END);
        Self::tSTRING_END
    }

    fn regx_options(&mut self) -> ([u8; 10], usize) {
        let mut c: MaybeByte;
        let mut flags = [0; 10];
        let mut flags_count = 0;

        self.newtok();
        loop {
            c = self.nextc();

            let ch = match c.as_option() {
                Some(_) if !c.is_alpha() => break,
                None => break,
                Some(ch) => ch,
            };

            match ch {
                b'o' | b'n' | b'e' | b's' | b'u' | b'i' | b'x' | b'm' => {
                    flags[flags_count] = ch;
                    flags_count += 1;
                }
                _ => {
                    self.tokadd(c);
                }
            }
        }

        self.buffer.pushback(c);
        if self.toklen() > 0 {
            self.tokfix();
            self.compile_error(
                DiagnosticMessage::UnknownRegexOptions {
                    options: self
                        .tokenbuf
                        .as_whole_string()
                        .expect("expected buffer to have only utf-8 chars"),
                },
                self.current_loc(),
            );
        }

        (flags, flags_count)
    }

    pub(crate) fn peek_variable_name(&mut self) -> Option<i32> {
        let mut ptr = self.buffer.pcur;

        if ptr + 1 >= self.buffer.pend {
            return None;
        }
        let mut c = self.char_at(ptr);
        ptr += 1;

        match c.as_option() {
            Some(b'$') => {
                c = self.char_at(ptr);
                if c == b'-' {
                    ptr += 1;
                    if ptr >= self.buffer.pend {
                        return None;
                    }
                    c = self.char_at(ptr);
                } else if c.is_global_name_punct() || c.is_digit() {
                    return Some(Self::tSTRING_DVAR);
                }
            }

            Some(b'@') => {
                c = self.char_at(ptr);
                if c == b'@' {
                    ptr += 1;
                    if ptr >= self.buffer.pend {
                        return None;
                    }
                    c = self.char_at(ptr);
                }
            }

            Some(b'{') => {
                self.buffer.pcur = ptr;
                self.command_start = true;
                return Some(Self::tSTRING_DBEG);
            }

            _ => return None,
        }

        if !c.is_ascii() || c == b'_' || c.is_alpha() {
            return Some(Self::tSTRING_DVAR);
        }

        None
    }

    pub(crate) fn tokadd_string(
        &mut self,
        func: u32,
        term: u8,
        paren: Option<u8>,
        nest: &mut u32,
    ) -> Option<MaybeByte> {
        let mut c: MaybeByte;
        let _erred = false;

        loop {
            c = self.nextc();
            if c.is_eof() {
                break;
            }

            if self.buffer.heredoc_indent > 0 {
                self.update_heredoc_indent(c);
            }

            if c == paren {
                *nest += 1;
            } else if c == term {
                if *nest == 0 {
                    self.buffer.pushback(c);
                    break;
                }
                *nest -= 1;
            } else if ((func & STR_FUNC_EXPAND) != 0)
                && c == b'#'
                && self.buffer.pcur < self.buffer.pend
            {
                let c2 = self.char_at(self.buffer.pcur);
                if c2 == b'$' || c2 == b'@' || c2 == b'{' {
                    self.buffer.pushback(c);
                    break;
                }
            } else if c == b'\\' {
                self.literal_flush(self.buffer.pcur - 1);
                c = self.nextc();
                match c.as_option() {
                    Some(b'\n') => {
                        if (func & STR_FUNC_QWORDS) != 0 {
                            // break;
                        } else {
                            if (func & STR_FUNC_EXPAND) != 0 {
                                if (func & STR_FUNC_INDENT) == 0 || self.buffer.heredoc_indent < 0 {
                                    continue;
                                }
                                if c == term {
                                    return Some(MaybeByte::new(b'\\'));
                                }
                            }
                            self.tokadd(b'\\');
                        }
                    }
                    Some(b'\\') => {
                        if (func & STR_FUNC_ESCAPE) != 0 {
                            self.tokadd(c)
                        }
                    }
                    Some(b'u') => {
                        if (func & STR_FUNC_EXPAND) == 0 {
                            self.tokadd(b'\\');
                        } else {
                            self.tokadd_utf8(
                                Some(term),
                                func & STR_FUNC_SYMBOL,
                                func & STR_FUNC_REGEXP,
                            );
                            continue;
                        }
                    }
                    None => {
                        return None;
                    }
                    _ => {
                        if !c.is_ascii() && (func & STR_FUNC_EXPAND) == 0 {
                            self.tokadd(b'\\');
                            self.tokadd(c);
                        }
                        if (func & STR_FUNC_REGEXP) != 0 {
                            match c {
                                MaybeByte::Some(b'c')
                                | MaybeByte::Some(b'C')
                                | MaybeByte::Some(b'M') => {
                                    self.buffer.pushback(c);
                                    c = self.read_escape(0);

                                    let mut escbuf = [0_u8; 5];
                                    write_to(&mut escbuf, format_args!("\\x{:X}", c.expect("bug")))
                                        .unwrap();
                                    for byte in escbuf.iter().take(4) {
                                        self.tokadd(MaybeByte::Some(*byte));
                                    }
                                    continue;
                                }
                                _ => {}
                            }
                            if c == term && !self.simple_re_meta(c) {
                                self.tokadd(c);
                                continue;
                            }
                            self.buffer.pushback(c);
                            if self.tokadd_escape().is_err() {
                                return None;
                            }
                            continue;
                        } else if (func & STR_FUNC_EXPAND) != 0 {
                            self.buffer.pushback(c);
                            if (func & STR_FUNC_ESCAPE) != 0 {
                                self.tokadd(b'\\')
                            }
                            c = self.read_escape(0);
                            if c.is_eof() {
                                return None;
                            }
                        } else if (func & STR_FUNC_QWORDS) != 0 && c.is_space() {
                            // ignore backslashed spaces in %w
                        } else if c != term && c != paren {
                            self.tokadd(b'\\');
                            self.buffer.pushback(c);
                            continue;
                        }
                    }
                }
            } else if !self.is_ascii() {
                self.tokadd(c);
                continue;
            } else if (func & STR_FUNC_QWORDS) != 0 && c.is_space() {
                self.buffer.pushback(c);
                break;
            }
            self.tokadd(c);
        }

        Some(c)
    }

    pub(crate) fn flush_string_content(&mut self) {
        // noop
    }

    fn tokadd_utf8_unterminated(&mut self) {
        self.token_flush();
        self.yyerror1(
            DiagnosticMessage::UnterminatedUnicodeEscape {},
            self.loc(self.buffer.ptok, self.buffer.pcur + 1),
        );
    }

    fn hex_byte_to_decimal(c: u8) -> Option<u32> {
        match c {
            b'0' => Some(0),
            b'1' => Some(1),
            b'2' => Some(2),
            b'3' => Some(3),
            b'4' => Some(4),
            b'5' => Some(5),
            b'6' => Some(6),
            b'7' => Some(7),
            b'8' => Some(8),
            b'9' => Some(9),
            b'A' | b'a' => Some(10),
            b'B' | b'b' => Some(11),
            b'C' | b'c' => Some(12),
            b'D' | b'd' => Some(13),
            b'E' | b'e' => Some(14),
            b'F' | b'f' => Some(15),
            _ => None,
        }
    }

    fn scan_hex(&mut self, start: u32, len: u32, numlen: &mut u32) -> u32 {
        let mut s = start;
        let mut result = 0;

        for _ in 0..len {
            match self.buffer.byte_at(s).as_option() {
                None => break,
                Some(c) => match Self::hex_byte_to_decimal(c) {
                    Some(hex) => {
                        result <<= 4;
                        result |= hex;
                    }
                    None => break,
                },
            }
            s += 1;
        }

        *numlen = s - start;
        result
    }

    fn scan_oct(&mut self, start: u32, len: u32, numlen: &mut u32) -> u32 {
        let mut s = start;
        let mut result = 0;

        for _ in 0..len {
            match self.buffer.byte_at(s).as_option() {
                Some(c) if (b'0'..=b'7').contains(&c) => {
                    result <<= 3;
                    result |= (c - b'0') as u32;
                }
                _ => break,
            }
            s += 1;
        }

        *numlen = s - start;
        result
    }

    pub(crate) fn tokcopy(&mut self, n: u32) {
        let substr = self
            .buffer
            .substr_at(self.buffer.pcur - n, self.buffer.pcur)
            .unwrap_or_else(|| panic!("no substr {}..{}", self.buffer.pcur - n, self.buffer.pcur));
        self.tokenbuf.push_bytes(substr);
    }

    fn tokaddmbc(&mut self, codepoint: u32) {
        let utf8_char = core::char::from_u32(codepoint).expect("expected codepoint to have digits");
        let mut buf = [0; 4];
        let utf8_bytes = utf8_char.encode_utf8(&mut buf).bytes();
        for byte in utf8_bytes {
            self.tokadd(byte)
        }
    }

    fn tokadd_codepoint(&mut self, regexp_literal: u32, wide: bool) -> bool {
        let mut numlen = 0;
        let codepoint = self.scan_hex(
            self.buffer.pcur,
            if wide {
                self.buffer.pend - self.buffer.pcur
            } else {
                4
            },
            &mut numlen,
        );
        self.literal_flush(self.buffer.pcur);
        self.buffer.pcur += numlen;
        if if wide {
            numlen == 0 || numlen > 6
        } else {
            numlen < 4
        } {
            self.yyerror1(
                DiagnosticMessage::InvalidUnicodeEscape {},
                self.loc(self.buffer.pcur, self.buffer.pcur + 1),
            );
            return wide && numlen > 0;
        }
        if codepoint > 0x10ffff {
            self.yyerror0(DiagnosticMessage::TooLargeUnicodeCodepoint {});
            return wide;
        }
        if (codepoint & 0xfffff800) == 0xd800 {
            self.yyerror0(DiagnosticMessage::InvalidUnicodeCodepoint {});
            return wide;
        }
        if regexp_literal != 0 {
            self.tokcopy(numlen);
        } else if codepoint >= 0x80 {
            // if self.buffer.encoding != "utf-8" {
            //     panic!("UTF-8 mixed within source");
            // }
            self.tokaddmbc(codepoint);
        } else {
            self.tokadd(codepoint as u8)
        }

        true
    }

    pub(crate) fn tokadd_utf8(
        &mut self,
        term: Option<u8>,
        _symbol_literal: u32,
        regexp_literal: u32,
    ) {
        let open_brace = b'{';
        let close_brace = b'}';
        let mut err_multiple_codepoints = false;

        if regexp_literal != 0 {
            self.tokadd(b'\\');
            self.tokadd(b'u')
        }

        if self.buffer.peek(open_brace) {
            let mut second: Option<u32> = None;
            let mut c;
            let mut last = self.nextc();
            if self.buffer.pcur >= self.buffer.pend {
                return self.tokadd_utf8_unterminated();
            }
            loop {
                c = self.buffer.byte_at(self.buffer.pcur);
                if !c.is_space() {
                    break;
                }
                self.buffer.pcur += 1;
                if self.buffer.pcur >= self.buffer.pend {
                    break;
                }
            }
            while c != close_brace {
                if c == term {
                    return self.tokadd_utf8_unterminated();
                }
                if err_multiple_codepoints {
                    second = Some(self.buffer.pcur);
                }
                if regexp_literal != 0 {
                    self.tokadd(last)
                }
                if !self.tokadd_codepoint(regexp_literal, true) {
                    break;
                }
                loop {
                    c = self.char_at(self.buffer.pcur);
                    if !c.is_space() {
                        break;
                    }
                    self.buffer.pcur += 1;
                    if self.buffer.pcur >= self.buffer.pend {
                        return self.tokadd_utf8_unterminated();
                    }
                    last = c;
                }
                if term.is_none() && second.is_none() {
                    err_multiple_codepoints = true;
                }
            }

            if c != close_brace {
                return self.tokadd_utf8_unterminated();
            }
            if let Some(second) = second {
                if err_multiple_codepoints {
                    let pcur = self.buffer.pcur;
                    self.buffer.pcur = second;
                    self.token_flush();
                    self.buffer.pcur = pcur;
                    self.yyerror0(DiagnosticMessage::MultipleCodepointAtSingleChar {});
                    self.token_flush();
                }
            }

            if regexp_literal != 0 {
                self.tokadd(close_brace)
            }
            self.nextc();
        } else if !self.tokadd_codepoint(regexp_literal, false) {
            self.token_flush();
        }
    }

    fn simple_re_meta(&mut self, c: MaybeByte) -> bool {
        matches!(
            c,
            MaybeByte::Some(b'$')
                | MaybeByte::Some(b'*')
                | MaybeByte::Some(b'+')
                | MaybeByte::Some(b'.')
                | MaybeByte::Some(b'?')
                | MaybeByte::Some(b'^')
                | MaybeByte::Some(b'|')
                | MaybeByte::Some(b')')
                | MaybeByte::Some(b']')
                | MaybeByte::Some(b'}')
                | MaybeByte::Some(b'>')
        )
    }

    fn tokadd_escape_eof(&mut self) -> Result<(), ()> {
        self.yyerror0(DiagnosticMessage::InvalidEscapeCharacter {});
        self.token_flush();
        Err(())
    }

    fn tokadd_escape(&mut self) -> Result<(), ()> {
        let mut numlen = 0;

        let c = self.nextc();
        match c.as_option() {
            Some(b'\n') => Ok(()),

            Some(octal) if (b'0'..b'8').contains(&octal) => {
                self.buffer.pcur -= 1;
                self.scan_oct(self.buffer.pcur, 3, &mut numlen);
                self.buffer.pcur += numlen;
                self.tokcopy(numlen + 1);
                Ok(())
            }

            Some(b'x') => {
                self.tok_hex(&mut numlen);
                if numlen == 0 {
                    return Err(());
                }
                self.tokcopy(numlen + 2);
                Ok(())
            }

            // eof:
            None => self.tokadd_escape_eof(),

            Some(other) => {
                self.tokadd(b'\\');
                self.tokadd(other);
                Ok(())
            }
        }
    }

    fn read_escape_eof(&mut self) -> MaybeByte {
        self.yyerror0(DiagnosticMessage::InvalidEscapeCharacter {});
        self.token_flush();
        MaybeByte::new(0)
    }

    fn tok_hex(&mut self, numlen: &mut u32) -> MaybeByte {
        let c = self.scan_hex(self.buffer.pcur, 2, numlen);
        if *numlen == 0 {
            self.yyerror1(DiagnosticMessage::InvalidHexEscape {}, self.current_loc());
            self.token_flush();
            return MaybeByte::new(0);
        }
        self.buffer.pcur += *numlen;
        MaybeByte::new(c as u8)
    }

    pub(crate) fn read_escape(&mut self, flags: u32) -> MaybeByte {
        let mut numlen: u32 = 0;

        let mut c = self.nextc();
        match c.as_option() {
            Some(b'\\') => c,
            Some(b'n') => MaybeByte::new(b'\n'),
            Some(b't') => MaybeByte::new(b'\t'),
            Some(b'r') => MaybeByte::new(b'\r'),
            Some(b'f') => MaybeByte::new(Self::LF_CHAR),
            Some(b'v') => MaybeByte::new(Self::VTAB_CHAR),
            Some(b'a') => MaybeByte::new(0x07_u8),
            Some(b'e') => MaybeByte::new(0x1b_u8),

            Some(b'0') | Some(b'1') | Some(b'2') | Some(b'3') | Some(b'4') | Some(b'5')
            | Some(b'6') | Some(b'7') | Some(b'8') | Some(b'9') => {
                self.buffer.pushback(c);
                let c = self.scan_oct(self.buffer.pcur, 3, &mut numlen);
                self.buffer.pcur += numlen;
                MaybeByte::new(c as u8)
            }

            Some(b'x') => {
                let c = self.tok_hex(&mut numlen);
                if numlen == 0 {
                    return MaybeByte::new(0);
                }
                c
            }

            Some(b'b') => MaybeByte::new(0x08),
            Some(b's') => MaybeByte::new(b' '),

            Some(b'M') => {
                if (flags & ESCAPE_META) != 0 {
                    return self.read_escape_eof();
                }
                c = self.nextc();
                if c != b'-' {
                    return self.read_escape_eof();
                }
                c = self.nextc();
                if c == b'\\' {
                    match self.buffer.peekc() {
                        MaybeByte::Some(b'u') | MaybeByte::Some(b'U') => {
                            self.nextc();
                            return self.read_escape_eof();
                        }
                        _ => {}
                    }
                    self.read_escape(flags | ESCAPE_META)
                        .map(|byte| MaybeByte::Some(byte | 0x80))
                } else if c.is_eof() || !c.is_ascii() {
                    self.read_escape_eof()
                } else {
                    if let Some(c2) = c.escaped_control_code() {
                        if c.is_control() || (flags & ESCAPE_CONTROL) == 0 {
                            self.warn_space_char(c2, "\\M-");
                        } else {
                            self.warn_space_char(c2, "\\C-\\M-");
                        }
                    } else if c.is_control() {
                        return self.read_escape_eof();
                    }
                    c.map(|c| MaybeByte::Some(c | 0x80))
                }
            }

            Some(b'C') | Some(b'c') => {
                if c == b'C' {
                    // C fallthrough
                    c = self.nextc();
                    if c != b'-' {
                        return self.read_escape_eof();
                    }
                }
                if (flags & ESCAPE_CONTROL) != 0 {
                    return self.read_escape_eof();
                }
                c = self.nextc();
                if c == b'\\' {
                    match self.buffer.peekc() {
                        MaybeByte::Some(b'u') | MaybeByte::Some(b'U') => {
                            self.nextc();
                            return self.read_escape_eof();
                        }
                        _ => {}
                    }
                    c = self.read_escape(flags | ESCAPE_CONTROL)
                } else if c == b'?' {
                    return MaybeByte::new(0x7f_u8);
                } else if c.is_eof() || !c.is_ascii() {
                    return self.read_escape_eof();
                } else if let Some(c2) = c.escaped_control_code() {
                    if c.is_control() {
                        if (flags & ESCAPE_META) != 0 {
                            self.warn_space_char(c2, "\\M-");
                        } else {
                            self.warn_space_char(c2, "");
                        }
                    } else if (flags & ESCAPE_META) != 0 {
                        self.warn_space_char(c2, "\\M-\\C-");
                    } else {
                        self.warn_space_char(c2, "\\C-");
                    }
                } else if c.is_control() {
                    return self.read_escape_eof();
                }
                c.map(|c| MaybeByte::Some(c & 0x9f))
            }

            None => self.read_escape_eof(),

            _ => c,
        }
    }

    pub(crate) fn is_ascii(&self) -> bool {
        self.char_at(self.buffer.pcur - 1).is_ascii()
    }

    pub(crate) fn warn_space_char(&mut self, c: u8, prefix: &'static str) {
        let suggestion = ByteArray::new(self.blob);
        suggestion.push_str(prefix, self.blob);
        suggestion.push_byte(b'\\', self.blob);
        suggestion.push_byte(c, self.blob);
        self.warn(
            DiagnosticMessage::InvalidCharacterSyntax {
                suggestion: suggestion.try_as_str().unwrap(),
            },
            self.current_loc(),
        )
    }
}
