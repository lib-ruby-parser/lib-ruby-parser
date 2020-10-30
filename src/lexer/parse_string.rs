use std::convert::TryInto;

use crate::lexer::*;
use crate::maybe_byte::*;
use crate::parser::TokenValue;
use crate::source::buffer::*;
use crate::str_term::{str_types::*, HeredocEnd, HeredocLiteral, StrTerm, StringLiteral};
use crate::TokenBuf;
use crate::{lex_states::*, DiagnosticMessage};

const ESCAPE_CONTROL: usize = 1;
const ESCAPE_META: usize = 2;

impl Lexer {
    pub(crate) const TAB_WIDTH: i32 = 8;

    pub(crate) fn parse_string(&mut self, quote: StringLiteral) -> i32 {
        let func = quote.func();
        let term = quote.term();
        let paren = quote.paren();
        let mut c: MaybeByte;
        let mut space = false;
        self.lval_start = Some(self.buffer.pcur);

        if self.debug {
            println!(
                "func = {}, pcur = {}, ptok = {}, term = {}",
                func,
                self.buffer.pcur,
                self.buffer.ptok,
                quote.term()
            );
        }

        if (func & STR_FUNC_TERM) != 0 {
            if (func & STR_FUNC_QWORDS) != 0 {
                self.nextc();
            } /* delayed term */
            self.set_lex_state(EXPR_END);
            self.strterm = None;
            if (func & STR_FUNC_REGEXP) != 0 {
                return Self::tREGEXP_END;
            } else {
                if let Some(heredoc_end) = quote.heredoc_end() {
                    self.lval_start = Some(heredoc_end.start);
                    self.lval_end = Some(heredoc_end.end);
                    self.set_yylval_str(TokenBuf::new(heredoc_end.value.as_bytes()));
                }
                return Self::tSTRING_END;
            }
        }
        c = self.nextc();
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
            quote.set_func(quote.func() & !STR_FUNC_LIST);
            space = true;
        }
        if c == term && quote.nest() == 0 {
            if (func & STR_FUNC_QWORDS) != 0 {
                quote.set_func(quote.func() | STR_FUNC_TERM);
                self.buffer.pushback(&c); /* dispatch the term at tSTRING_END */
                return Self::tSPACE;
            }
            return self.parser_string_term(term, func);
        }
        if space {
            self.buffer.pushback(&c);
            return Self::tSPACE;
        }
        self.newtok();
        if ((func & STR_FUNC_EXPAND) != 0) && c == b'#' {
            if let Some(t) = self.parser_peek_variable_name() {
                return t;
            }
            self.tokadd(b'#');
            c = self.nextc();
        }
        self.buffer.pushback(&c);

        let mut nest = quote.nest();
        let added = self.tokadd_string(func, term, paren, &mut nest);
        quote.set_nest(nest);

        if added.is_some() && self.buffer.eofp {
            self.literal_flush(self.buffer.pcur);
            if (func & STR_FUNC_QWORDS) != 0 {
                /* no content to add, bailing out here */
                self.yyerror0(DiagnosticMessage::UnterminatedList);
                self.strterm = None;
                return Self::tSTRING_END;
            }
            if (func & STR_FUNC_REGEXP) != 0 {
                self.yyerror0(DiagnosticMessage::UnterminatedRegexp);
            } else {
                self.yyerror0(DiagnosticMessage::UnterminatedString);
            }
            quote.set_func(quote.func() | STR_FUNC_TERM);
        }

        self.tokfix();
        self.set_yylval_str(self.tokenbuf.clone());
        self.flush_string_content();

        Self::tSTRING_CONTENT
    }

    pub(crate) fn parser_string_term(&mut self, term: u8, func: usize) -> i32 {
        self.strterm = None;
        if (func & STR_FUNC_REGEXP) != 0 {
            let flags = self.regx_options();
            self.set_yylval_num(format!("{}{}", term as char, flags));
            self.set_lex_state(EXPR_END);
            return Self::tREGEXP_END;
        }
        if (func & STR_FUNC_LABEL) != 0 && self.is_label_suffix(0) {
            self.nextc();
            self.set_lex_state(EXPR_BEG | EXPR_LABEL);
            return Self::tLABEL_END;
        }
        self.set_lex_state(EXPR_END);
        Self::tSTRING_END
    }

    pub(crate) fn set_yylval_num(&mut self, flags: String) {
        if self.debug {
            println!("set_yylval_num {:#?}", flags);
        }
        self.lval = Some(TokenValue::String(flags));
    }

    pub(crate) fn regx_options(&mut self) -> String {
        let mut c: MaybeByte;
        let mut result = String::from("");

        self.newtok();
        loop {
            c = self.nextc();

            let ch = match c.to_option() {
                Some(_) if !c.is_alpha() => break,
                None => break,
                Some(ch) => ch,
            };

            match ch {
                b'o' | b'n' | b'e' | b's' | b'u' | b'i' | b'x' | b'm' => {
                    result.push(ch as char);
                }
                _ => {
                    self.tokadd(&c);
                }
            }
        }

        self.buffer.pushback(&c);
        if self.toklen() > 0 {
            self.tokfix();
            self.compile_error(DiagnosticMessage::UnknownRegexOptions(
                self.tokenbuf
                    .borrow_string()
                    .expect("expected buffer to have only utf-8 chars")
                    .to_owned(),
            ));
        }

        result
    }

    pub(crate) fn parser_peek_variable_name(&mut self) -> Option<i32> {
        let mut c: MaybeByte;
        let mut ptr: usize = self.buffer.pcur;

        if ptr + 1 >= self.buffer.pend {
            return None;
        }
        c = self.char_at(ptr);
        ptr += 1;

        match c.to_option() {
            Some(b'$') => {
                c = self.char_at(ptr);
                if c == b'-' {
                    ptr += 1;
                    if ptr >= self.buffer.pend {
                        return None;
                    }
                // c = self.char_at(ptr);
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
        func: usize,
        term: u8,
        paren: Option<u8>,
        nest: &mut usize,
    ) -> Option<MaybeByte> {
        let mut c: MaybeByte;
        let _erred = false;

        loop {
            c = self.nextc();
            if c.is_eof() {
                break;
            }

            if self.buffer.heredoc_indent > 0 {
                self.parser_update_heredoc_indent(&c);
            }

            if c == paren {
                *nest += 1;
            } else if c == term {
                if *nest == 0 {
                    self.buffer.pushback(&c);
                    break;
                }
                *nest -= 1;
            } else if ((func & STR_FUNC_EXPAND) != 0)
                && c == b'#'
                && self.buffer.pcur < self.buffer.pend
            {
                let c2 = self.char_at(self.buffer.pcur);
                if c2 == b'$' || c2 == b'@' || c2 == b'{' {
                    self.buffer.pushback(&c);
                    break;
                }
            } else if c == b'\\' {
                self.literal_flush(self.buffer.pcur - 1);
                c = self.nextc();
                match c.to_option() {
                    Some(b'\n') => {
                        if (func & STR_FUNC_QWORDS) != 0 {
                            // break;
                        } else {
                            if (func & STR_FUNC_EXPAND) != 0 {
                                if (func & STR_FUNC_INDENT) == 0 || self.buffer.heredoc_indent < 0 {
                                    continue;
                                }
                                if c == term {
                                    return Some(MaybeByte::new('\\'));
                                }
                            }
                            self.tokadd(b'\\');
                        }
                    }
                    Some(b'\\') => {
                        if (func & STR_FUNC_ESCAPE) != 0 {
                            self.tokadd(&c)
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
                            self.tokadd(&c);
                        }
                        if (func & STR_FUNC_REGEXP) != 0 {
                            if c == term && !self.simple_re_meta(&c) {
                                self.tokadd(&c);
                                continue;
                            }
                            self.buffer.pushback(&c);
                            if self.tokadd_escape().is_err() {
                                return None;
                            }
                            continue;
                        } else if (func & STR_FUNC_EXPAND) != 0 {
                            self.buffer.pushback(&c);
                            if (func & STR_FUNC_ESCAPE) != 0 {
                                self.tokadd(b'\\')
                            }
                            c = self.read_escape(0);
                        } else if (func & STR_FUNC_QWORDS) != 0 && c.is_space() {
                            // ignore backslashed spaces in %w
                        } else if c != term && c != paren {
                            self.tokadd(b'\\');
                            self.buffer.pushback(&c);
                            continue;
                        }
                    }
                }
            } else if !self.parser_is_ascii() {
                self.tokadd(&c);
                continue;
            } else if (func & STR_FUNC_QWORDS) != 0 && c.is_space() {
                self.buffer.pushback(&c);
                break;
            }
            self.tokadd(&c);
        }

        Some(c)
    }

    pub(crate) fn set_yylval_str(&mut self, value: TokenBuf) {
        if self.debug {
            println!("set_yylval_str {:#?}", value);
        }
        self.lval = Some(value.to_token_value());
    }

    pub(crate) fn flush_string_content(&mut self) {
        // noop
    }

    pub(crate) fn parser_update_heredoc_indent(&mut self, c: &MaybeByte) -> bool {
        if self.buffer.heredoc_line_indent == -1 {
            if *c == b'\n' {
                self.buffer.heredoc_line_indent = 0
            }
        } else if *c == b' ' {
            self.buffer.heredoc_line_indent += 1;
            return true;
        } else if *c == b'\t' {
            let w = (self.buffer.heredoc_line_indent / Self::TAB_WIDTH) + 1;
            self.buffer.heredoc_line_indent = w * Self::TAB_WIDTH;
            return true;
        } else if *c != '\n' {
            if self.buffer.heredoc_indent > self.buffer.heredoc_line_indent {
                self.buffer.heredoc_indent = self.buffer.heredoc_line_indent
            }
            self.buffer.heredoc_line_indent = -1;
        }
        true
    }

    pub(crate) fn tokadd_utf8_unterminated(&mut self) {
        unimplemented!("tokadd_utf8_unterminated")
    }

    pub(crate) fn scan_hex(&mut self, start: usize, len: usize, numlen: &mut usize) -> usize {
        let mut s = start;
        let mut result = 0;

        for _ in 0..len {
            match self.buffer.byte_at(s).to_option() {
                None => break,
                Some(c) => match usize::from_str_radix(&(c as char).to_string(), 16) {
                    Ok(hex) => {
                        result <<= 4;
                        result |= hex;
                    }
                    Err(_) => break,
                },
            }
            s += 1;
        }

        *numlen = s - start;
        result
    }

    pub(crate) fn scan_oct(&mut self, start: usize, len: usize, numlen: &mut usize) -> usize {
        let mut s = start;
        let mut result: usize = 0;

        for _ in 0..len {
            match self.buffer.byte_at(s).to_option() {
                Some(c) if (c >= b'0' && c <= b'7') => {
                    result <<= 3;
                    result |= ((c as u8) - b'0') as usize;
                }
                _ => break,
            }
            s += 1;
        }

        *numlen = s - start;
        result
    }

    pub(crate) fn tokcopy(&mut self, n: usize) {
        let substr = self
            .buffer
            .substr_at(self.buffer.pcur - n, self.buffer.pcur)
            .unwrap_or_else(|| panic!("no substr {}..{}", self.buffer.pcur - n, self.buffer.pcur));
        self.tokenbuf.append(&substr);
    }

    pub(crate) fn tokaddmbc(&mut self, codepoint: usize) {
        let utf8_char =
            std::char::from_u32(codepoint.try_into().expect("expected codepoint to be u32"))
                .expect("expected codepoint to have digits");
        let utf8_bytes = utf8_char.to_string().into_bytes();
        for byte in utf8_bytes {
            self.tokadd(byte)
        }
    }

    pub(crate) fn tokadd_codepoint(&mut self, regexp_literal: usize, wide: bool) -> bool {
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
            self.yyerror0(DiagnosticMessage::InvalidUnicodeEscape);
            return wide && numlen > 0;
        }
        if codepoint > 0x10ffff {
            self.yyerror0(DiagnosticMessage::TooLargeUnicodeCodepoint);
            return wide;
        }
        if (codepoint & 0xfffff800) == 0xd800 {
            self.yyerror0(DiagnosticMessage::InvalidUnicodeCodepoint);
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
        _symbol_literal: usize,
        regexp_literal: usize,
    ) {
        let open_brace = b'{';
        let close_brace = b'}';
        let mut got_multiple_codepoints = false;

        if regexp_literal != 0 {
            self.tokadd(b'\\');
            self.tokadd(b'u')
        }

        if self.buffer.peek(open_brace) {
            let mut second: Option<usize> = None;
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
                if got_multiple_codepoints {
                    second = Some(self.buffer.pcur);
                }
                if regexp_literal != 0 {
                    self.tokadd(&last)
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
                if term.is_none() && second.is_some() {
                    got_multiple_codepoints = true;
                }
            }

            if c != close_brace {
                return self.tokadd_utf8_unterminated();
            }
            if let Some(second) = second {
                if !got_multiple_codepoints {
                    let pcur = self.buffer.pcur;
                    self.buffer.pcur = second;
                    self.token_flush();
                    self.buffer.pcur = pcur;
                    self.yyerror0(DiagnosticMessage::MultipleCodepointAtSingleChar);
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

    pub(crate) fn simple_re_meta(&mut self, c: &MaybeByte) -> bool {
        matches!(
            c.to_option(),
            Some(b'$')
                | Some(b'*')
                | Some(b'+')
                | Some(b'.')
                | Some(b'?')
                | Some(b'^')
                | Some(b'|')
                | Some(b')')
                | Some(b']')
                | Some(b'}')
                | Some(b'>')
        )
    }

    pub(crate) fn tokadd_escape_eof(&mut self) -> Result<(), ()> {
        self.yyerror0(DiagnosticMessage::InvalidEscapeCharacter);
        self.token_flush();
        Err(())
    }

    pub(crate) fn tokadd_escape(&mut self) -> Result<(), ()> {
        let mut c;
        let mut flags = 0;
        let mut numlen = 0;

        loop {
            c = self.nextc();
            match c.to_option() {
                Some(b'\n') => return Ok(()),

                Some(b'0') | Some(b'1') | Some(b'2') | Some(b'3') | Some(b'4') | Some(b'5')
                | Some(b'6') | Some(b'7') => {
                    self.buffer.pcur -= 1;
                    self.scan_oct(self.buffer.pcur, 3, &mut numlen);
                    self.buffer.pcur += numlen;
                    self.tokcopy(numlen + 1);
                    return Ok(());
                }

                Some(b'x') => {
                    self.tok_hex(&mut numlen);
                    if numlen == 0 {
                        return Err(());
                    }
                    self.tokcopy(numlen + 2);
                    return Ok(());
                }

                Some(b'M') => {
                    if (flags & ESCAPE_META) != 0 {
                        return self.tokadd_escape_eof();
                    }
                    c = self.nextc();
                    if c != '-' {
                        self.buffer.pushback(&c);
                        return self.tokadd_escape_eof();
                    }
                    self.tokcopy(3);
                    flags |= ESCAPE_META;

                    // goto escaped
                    c = self.nextc();
                    if c == b'\\' {
                        continue;
                    } else if c.is_eof() {
                        return self.tokadd_escape_eof();
                    }
                    self.tokadd(&c);
                    return Ok(());
                }

                Some(b'C') => {
                    if (flags & ESCAPE_CONTROL) != 0 {
                        return self.tokadd_escape_eof();
                    }
                    c = self.nextc();
                    if c != '-' {
                        self.buffer.pushback(&c);
                        return self.tokadd_escape_eof();
                    }
                    self.tokcopy(3);

                    // goto escaped
                    c = self.nextc();
                    if c == b'\\' {
                        continue;
                    } else if c.is_eof() {
                        return self.tokadd_escape_eof();
                    }
                    self.tokadd(&c);
                    return Ok(());
                }

                Some(b'c') => {
                    if (flags & ESCAPE_CONTROL) != 0 {
                        return self.tokadd_escape_eof();
                    }
                    self.tokcopy(2);
                    flags |= ESCAPE_CONTROL;

                    // escaped:
                    c = self.nextc();
                    if c == b'\\' {
                        continue;
                    } else if c.is_eof() {
                        return self.tokadd_escape_eof();
                    }
                    self.tokadd(&c);
                    return Ok(());
                }

                // eof:
                None => return self.tokadd_escape_eof(),

                Some(other) => {
                    self.tokadd(b'\\');
                    self.tokadd(other);
                    return Ok(());
                }
            }
        }
    }

    pub(crate) fn read_escape_eof(&mut self) -> MaybeByte {
        self.yyerror0(DiagnosticMessage::InvalidEscapeCharacter);
        self.token_flush();
        unimplemented!("read_escape_eof")
    }

    pub(crate) fn tok_hex(&mut self, numlen: &mut usize) -> MaybeByte {
        let c;

        c = self.scan_hex(self.buffer.pcur, 2, numlen);
        if *numlen == 0 {
            self.yyerror0(DiagnosticMessage::InvalidHexEscape);
            self.token_flush();
            return MaybeByte::new(0);
        }
        self.buffer.pcur += *numlen;
        MaybeByte::new(c as u8)
    }

    pub(crate) fn read_escape(&mut self, flags: usize) -> MaybeByte {
        let mut c;
        let mut numlen: usize = 0;

        c = self.nextc();
        match c.to_option() {
            Some(b'\\') => c,
            Some(b'n') => MaybeByte::new('\n'),
            Some(b't') => MaybeByte::new('\t'),
            Some(b'r') => MaybeByte::new('\r'),
            Some(b'f') => MaybeByte::new(Self::LF_CHAR),
            Some(b'v') => MaybeByte::new(Self::VTAB_CHAR),
            Some(b'a') => MaybeByte::new(0x07_u8),
            Some(b'e') => MaybeByte::new(0x1b_u8),

            Some(b'0') | Some(b'1') | Some(b'2') | Some(b'3') | Some(b'4') | Some(b'5')
            | Some(b'6') | Some(b'7') | Some(b'8') | Some(b'9') => {
                self.buffer.pushback(&c);
                let c = self.scan_oct(self.buffer.pcur, 3, &mut numlen);
                self.buffer.pcur += numlen;
                MaybeByte::new(c as u8)
            }

            Some(b'x') => {
                let c = self.tok_hex(&mut numlen);
                if numlen == 0 {
                    return MaybeByte::EndOfInput;
                }
                c
            }

            Some(b'b') => MaybeByte::new(0x08_u8),
            Some(b's') => MaybeByte::new(' '),

            Some(b'M') => {
                if (flags & ESCAPE_META) != 0 {
                    return self.read_escape_eof();
                }
                c = self.nextc();
                if c != '-' {
                    return self.read_escape_eof();
                }
                c = self.nextc();
                if c == b'\\' {
                    if self.buffer.peek(b'u') {
                        return self.read_escape_eof();
                    }
                    self.read_escape(flags | ESCAPE_META)
                        .map(|byte| MaybeByte::Some(byte | 0x80))
                } else if c.is_eof() || !c.is_ascii() {
                    self.read_escape_eof()
                } else {
                    if let Some(c2) = self.escaped_control_code(&c) {
                        if c.is_control() || (flags & ESCAPE_CONTROL) == 0 {
                            self.warn(DiagnosticMessage::InvalidCharacterSyntax {
                                suggestion: format!("\\M-\\{}", c2),
                            });
                        } else {
                            self.warn(DiagnosticMessage::InvalidCharacterSyntax {
                                suggestion: format!("\\C-\\M-\\{}", c2),
                            });
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
                    if c != '-' {
                        return self.read_escape_eof();
                    }
                }
                if (flags & ESCAPE_CONTROL) != 0 {
                    return self.read_escape_eof();
                }
                c = self.nextc();
                if c == b'\\' {
                    if self.buffer.peek(b'u') {
                        return self.read_escape_eof();
                    }
                    c = self.read_escape(flags | ESCAPE_CONTROL)
                } else if c == b'?' {
                    return MaybeByte::new(0x7f_u8);
                } else if c.is_eof() || !c.is_ascii() {
                    return self.read_escape_eof();
                } else if let Some(c2) = self.escaped_control_code(&c) {
                    if c.is_control() {
                        if (flags & ESCAPE_META) != 0 {
                            self.warn(DiagnosticMessage::InvalidCharacterSyntax {
                                suggestion: format!("\\M-\\{}", c2),
                            });
                        } else {
                            self.warn(DiagnosticMessage::InvalidCharacterSyntax {
                                suggestion: format!("\\{}", c2),
                            });
                        }
                    } else if (flags & ESCAPE_META) != 0 {
                        self.warn(DiagnosticMessage::InvalidCharacterSyntax {
                            suggestion: format!("\\M-\\C-\\{}", c2),
                        });
                    } else {
                        self.warn(DiagnosticMessage::InvalidCharacterSyntax {
                            suggestion: format!("\\C-\\{}", c2),
                        });
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

    pub(crate) fn parser_is_ascii(&self) -> bool {
        self.char_at(self.buffer.pcur - 1).is_ascii()
    }

    pub(crate) fn heredoc_identifier(&mut self) -> Option<i32> {
        /*
         * term_len is length of `<<"END"` except `END`,
         * in this case term_len is 4 (<, <, " and ").
         */
        let len;
        let mut offset = self.buffer.pcur - self.buffer.pbeg;
        let mut c = self.nextc();
        let term;
        let mut func = 0;
        let mut quote = 0;
        let mut token = Self::tSTRING_BEG;
        let mut indent = 0;

        if c == b'-' {
            c = self.nextc();
            func = STR_FUNC_INDENT;
            offset += 1;
        } else if c == b'~' {
            c = self.nextc();
            func = STR_FUNC_INDENT;
            offset += 1;
            indent = std::i32::MAX;
        }

        if c == b'\'' || c == b'"' || c == b'`' {
            if c == b'\'' {
                func |= str_squote
            }
            if c == b'"' {
                func |= str_dquote
            }
            if c == b'`' {
                func |= str_xquote;
                token = Self::tXSTRING_BEG
            }

            quote += 1;
            offset += 1;
            term = c;

            loop {
                c = self.nextc();
                if c == term {
                    break;
                }

                if c.is_eof() || c == b'\r' || c == b'\n' {
                    self.yyerror0(DiagnosticMessage::UnterminatedHeredocId);
                    return None;
                }
            }
        } else {
            if !self.parser_is_identchar() {
                self.buffer.pushback(&c);
                if (func & STR_FUNC_INDENT) != 0 {
                    self.buffer.pushback(&if indent > 0 { '~' } else { '-' });
                }
                return None;
            }
            func |= str_dquote;
            loop {
                if let Some(n) = self.parser_precise_mbclen(self.buffer.pcur - 1) {
                    self.buffer.pcur += n - 1;
                } else {
                    return None;
                }
                c = self.nextc();
                if c.is_eof() || !self.parser_is_identchar() {
                    break;
                }
            }
            self.buffer.pushback(&c);
        }

        len = self.buffer.pcur - (self.buffer.pbeg + offset) - quote;

        let id = self
            .buffer
            .substr_at(self.buffer.ptok, self.buffer.pcur)
            .expect("failed to get heredoc id");
        let id = TokenBuf::new(id);
        self.set_yylval_str(id);
        self.lval_start = Some(self.buffer.ptok);
        self.lval_end = Some(self.buffer.pcur);

        self.buffer.goto_eol();

        self.strterm = Some(StrTerm::new_heredoc(HeredocLiteral::new(
            self.buffer.lastline,
            offset,
            self.buffer.ruby_sourceline,
            len,
            quote,
            func,
        )));

        self.token_flush();
        self.buffer.heredoc_indent = indent;
        self.buffer.heredoc_line_indent = 0;
        Some(token)
    }

    pub(crate) fn here_document(&mut self, here: HeredocLiteral) -> i32 {
        self.lval_start = Some(self.buffer.pcur);

        let mut c;
        let func;
        let indent;
        let eos;
        let mut ptr;
        let mut ptr_end;
        let len;
        let mut str_ = TokenBuf::new(b"");
        let bol;

        let heredoc_end: HeredocEnd;

        eos = self.buffer.input.lines[here.lastline()].start + here.offset();
        len = here.length();
        func = here.func();
        indent = here.func() & STR_FUNC_INDENT;

        c = self.nextc();
        if c.is_eof() {
            return self.here_document_error(&here, eos, len);
        }
        bol = self.buffer.was_bol();
        if !bol {
            /* not beginning of line, cannot be the terminator */
        } else if self.buffer.heredoc_line_indent == -1 {
            /* `heredoc_line_indent == -1` means
             * - "after an interpolation in the same line", or
             * - "in a continuing line"
             */
            self.buffer.heredoc_line_indent = 0;
        } else if self.buffer.is_whole_match(
            &self
                .buffer
                .substr_at(eos, eos + len)
                .expect("failed to get heredoc id for comparison"),
            indent,
        ) {
            return self.here_document_restore(&here);
        }

        if (func & STR_FUNC_EXPAND) == 0 {
            loop {
                ptr = self.buffer.input.lines[self.buffer.lastline].start;
                ptr_end = self.buffer.pend;
                if ptr_end > ptr {
                    match self.buffer.input.bytes[ptr_end - 1] {
                        b'\n' => {
                            ptr_end -= 1;
                            if ptr_end == ptr || self.buffer.input.bytes[ptr_end - 1] != b'\r' {
                                ptr_end += 1;
                            }
                        }
                        b'\r' => {
                            ptr_end -= 1;
                        }
                        _ => {}
                    }
                }

                if self.buffer.heredoc_indent > 0 {
                    let mut i = 0;
                    while (ptr + i < ptr_end)
                        && self.parser_update_heredoc_indent(&self.char_at(ptr + i))
                    {
                        i += 1;
                    }
                    self.buffer.heredoc_line_indent = 0;
                }

                match self.buffer.substr_at(ptr, ptr_end) {
                    Some(s) => str_.append(&s),
                    _ => panic!(
                        "no substr {}..{} (len = {})",
                        ptr,
                        ptr_end,
                        self.buffer.input.bytes.len()
                    ),
                };
                if ptr_end < self.buffer.pend {
                    str_.push(b'\n')
                }
                self.buffer.goto_eol();
                if self.buffer.heredoc_indent > 0 {
                    return self.heredoc_flush_str(&str_);
                }
                if self.nextc().is_eof() {
                    str_.clear();
                    return self.here_document_error(&here, eos, len);
                }

                if self.buffer.is_whole_match(
                    &self
                        .buffer
                        .substr_at(eos, eos + len)
                        .expect("failed to get heredoc id for comparison"),
                    indent,
                ) {
                    self.lval_end = Some(self.buffer.pend - 1);
                    heredoc_end = self.compute_heredoc_end();
                    break;
                }
            }
        } else {
            self.newtok();
            if c == b'#' {
                let t = self.parser_peek_variable_name();
                if self.buffer.heredoc_line_indent != -1 {
                    if self.buffer.heredoc_indent > self.buffer.heredoc_line_indent {
                        self.buffer.heredoc_indent = self.buffer.heredoc_line_indent;
                    }
                    self.buffer.heredoc_line_indent = -1;
                }
                if let Some(t) = t {
                    return t;
                }
                self.tokadd(b'#');
                c = self.nextc();
            }
            loop {
                self.buffer.pushback(&c);
                // enc = self.p.enc;
                match self.tokadd_string(func, b'\n', None, &mut 0) {
                    Some(cc) => c = cc,
                    None => {
                        if self.buffer.eofp {
                            return self.here_document_error(&here, eos, len);
                        }
                        return self.here_document_restore(&here);
                    }
                }
                self.lval_end = Some(self.buffer.pcur + 1);
                if c != '\n' {
                    if c == b'\\' {
                        self.buffer.heredoc_line_indent = -1
                    }
                    return self.heredoc_flush();
                }
                let cc = self.nextc();
                self.tokadd(&cc);
                if self.buffer.heredoc_indent > 0 {
                    self.buffer.goto_eol();
                    return self.heredoc_flush();
                }
                c = self.nextc();
                if c.is_eof() {
                    return self.here_document_error(&here, eos, len);
                }

                if self.buffer.is_whole_match(
                    &self
                        .buffer
                        .substr_at(eos, eos + len)
                        .expect("failed to get heredoc id for comparison"),
                    indent,
                ) {
                    heredoc_end = self.compute_heredoc_end();

                    break;
                }
            }
            str_ = self.tokenbuf.clone();
        }

        self.heredoc_restore(&here);
        self.token_flush();
        self.strterm = self.new_strterm(
            func | STR_FUNC_TERM,
            0 as u8,
            Some(0 as u8),
            Some(heredoc_end),
        );
        self.set_yylval_str(str_);
        Self::tSTRING_CONTENT
    }

    pub(crate) fn compute_heredoc_end(&self) -> HeredocEnd {
        let start = self.buffer.pbeg;
        let mut end_starts_at = start;
        while self.buffer.byte_at(end_starts_at) == b' ' {
            end_starts_at += 1;
        }
        let mut end = end_starts_at;
        loop {
            let c = self.buffer.byte_at(end);
            if c.is_eof() || c == b'\n' {
                break;
            }
            end += 1;
        }
        let value = self
            .buffer
            .substr_at(end_starts_at, end)
            .expect("failed to get heredoc end");

        let value =
            String::from_utf8(value.to_vec()).expect("expected heredoc id to be valid in utf-8");

        HeredocEnd { start, end, value }
    }

    pub(crate) fn here_document_error(
        &mut self,
        here: &HeredocLiteral,
        eos: usize,
        len: usize,
    ) -> i32 {
        self.heredoc_restore(&here);
        self.compile_error(DiagnosticMessage::UnterminatedHeredoc(
            String::from_utf8_lossy(
                self.buffer
                    .substr_at(eos, eos + len)
                    .expect("failed to get heredoc id for comparison"),
            )
            .into_owned(),
        ));
        self.token_flush();
        self.strterm = None;
        self.set_lex_state(EXPR_END);
        Self::tSTRING_END
    }

    pub(crate) fn here_document_restore(&mut self, here: &HeredocLiteral) -> i32 {
        let heredoc_end = self.compute_heredoc_end();
        self.lval_start = Some(heredoc_end.start);
        self.lval_end = Some(heredoc_end.end);
        self.set_yylval_str(TokenBuf::new(heredoc_end.value.as_bytes()));

        self.heredoc_restore(&here);
        self.token_flush();
        self.strterm = None;
        self.set_lex_state(EXPR_END);

        Self::tSTRING_END
    }

    pub(crate) fn heredoc_flush_str(&mut self, str_: &TokenBuf) -> i32 {
        self.set_yylval_str(str_.clone());
        self.flush_string_content();
        Self::tSTRING_CONTENT
    }

    pub(crate) fn heredoc_flush(&mut self) -> i32 {
        self.heredoc_flush_str(&self.tokenbuf.clone())
    }

    pub(crate) fn heredoc_restore(&mut self, here: &HeredocLiteral) {
        self.strterm = None;
        let line = here.lastline();
        self.buffer.lastline = line;
        self.buffer.pbeg = self.buffer.input.lines[line].start;
        self.buffer.pend = self.buffer.pbeg + self.buffer.input.lines[line].len();
        self.buffer.pcur = self.buffer.pbeg + here.offset() + here.length() + here.quote();
        self.buffer.ptok = self.buffer.pbeg + here.offset() - here.quote();
        self.buffer.heredoc_end = self.buffer.ruby_sourceline;
        self.buffer.ruby_sourceline = here.sourceline();
        if self.buffer.eofp {
            self.buffer.nextline = 0
        }
        self.buffer.eofp = false;
    }
}
