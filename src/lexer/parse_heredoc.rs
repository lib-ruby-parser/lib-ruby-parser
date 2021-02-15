use crate::lexer::{ParseIdent, ParseString, TokAdd, Yylval};
use crate::maybe_byte::MaybeByte;
use crate::source::buffer::*;
use crate::str_term::{str_types::*, HeredocEnd, HeredocLiteral, StrTerm};
use crate::Lexer;
use crate::TokenBuf;
use crate::{lex_states::*, DiagnosticMessage};

pub(crate) trait ParseHeredoc {
    fn heredoc_identifier(&mut self) -> Option<i32>;
    fn here_document(&mut self) -> i32;
    fn compute_heredoc_end(&self) -> HeredocEnd;
    fn here_document_error(&mut self, here: &HeredocLiteral, eos: usize, len: usize) -> i32;
    fn here_document_restore(&mut self, here: &HeredocLiteral) -> i32;
    fn heredoc_flush_str(&mut self, str_: &TokenBuf) -> i32;
    fn heredoc_flush(&mut self) -> i32;
    fn heredoc_restore(&mut self, here: &HeredocLiteral);
    fn update_heredoc_indent(&mut self, c: &MaybeByte) -> bool;
}

const TAB_WIDTH: i32 = 8;

impl ParseHeredoc for Lexer {
    fn heredoc_identifier(&mut self) -> Option<i32> {
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
                    return Some(Self::END_OF_INPUT);
                }
            }
        } else {
            if !self.is_identchar() {
                self.buffer.pushback(&c);
                if (func & STR_FUNC_INDENT) != 0 {
                    self.buffer.pushback(&if indent > 0 { '~' } else { '-' });
                }
                return None;
            }
            func |= str_dquote;
            loop {
                let n = self.multibyte_char_len(self.buffer.pcur - 1);
                self.buffer.pcur += n - 1;
                c = self.nextc();
                if c.is_eof() || !self.is_identchar() {
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
        self.set_yylval_str(&id);
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

    fn here_document(&mut self) -> i32 {
        let here = match self.strterm.as_ref().unwrap() {
            StrTerm::StringLiteral(_) => unreachable!("strterm must be heredoc"),
            StrTerm::HeredocLiteral(h) => h.clone(),
        };
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

        eos = self.buffer.input.line_at(here.lastline).start + here.offset;
        len = here.length;
        func = here.func;
        indent = here.func & STR_FUNC_INDENT;

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
                ptr = self.buffer.input.line_at(self.buffer.lastline).start;
                ptr_end = self.buffer.pend;
                if ptr_end > ptr {
                    match self.buffer.input.unchecked_byte_at(ptr_end - 1) {
                        b'\n' => {
                            ptr_end -= 1;
                            if ptr_end == ptr
                                || self.buffer.input.unchecked_byte_at(ptr_end - 1) != b'\r'
                            {
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
                    while (ptr + i < ptr_end) && self.update_heredoc_indent(&self.char_at(ptr + i))
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
                        self.buffer.input.len()
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
                let t = self.peek_variable_name();
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
        self.set_yylval_str(&str_);
        Self::tSTRING_CONTENT
    }

    fn compute_heredoc_end(&self) -> HeredocEnd {
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

    fn here_document_error(&mut self, here: &HeredocLiteral, eos: usize, len: usize) -> i32 {
        self.heredoc_restore(&here);
        self.compile_error(
            DiagnosticMessage::UnterminatedHeredoc(
                String::from_utf8_lossy(
                    self.buffer
                        .substr_at(eos, eos + len)
                        .expect("failed to get heredoc id for comparison"),
                )
                .into_owned(),
            ),
            self.current_loc(),
        );
        self.token_flush();
        self.strterm = None;
        self.lex_state.set(EXPR_END);
        Self::tSTRING_END
    }

    fn here_document_restore(&mut self, here: &HeredocLiteral) -> i32 {
        let heredoc_end = self.compute_heredoc_end();
        self.lval_start = Some(heredoc_end.start);
        self.lval_end = Some(heredoc_end.end);
        self.set_yylval_str(&TokenBuf::new(heredoc_end.value.as_bytes()));

        self.heredoc_restore(&here);
        self.token_flush();
        self.strterm = None;
        self.lex_state.set(EXPR_END);

        Self::tSTRING_END
    }

    fn heredoc_flush_str(&mut self, str_: &TokenBuf) -> i32 {
        self.set_yylval_str(str_);
        self.flush_string_content();
        Self::tSTRING_CONTENT
    }

    fn heredoc_flush(&mut self) -> i32 {
        let tokenbuf = self.tokenbuf.take();
        self.heredoc_flush_str(&tokenbuf)
    }

    fn heredoc_restore(&mut self, here: &HeredocLiteral) {
        self.strterm = None;
        let line = here.lastline;
        self.buffer.lastline = line;
        self.buffer.pbeg = self.buffer.input.line_at(line).start;
        self.buffer.pend = self.buffer.pbeg + self.buffer.input.line_at(line).len();
        self.buffer.pcur = self.buffer.pbeg + here.offset + here.length + here.quote;
        self.buffer.ptok = self.buffer.pbeg + here.offset - here.quote;
        self.buffer.heredoc_end = self.buffer.ruby_sourceline;
        self.buffer.ruby_sourceline = here.sourceline;
        if self.buffer.eofp {
            self.buffer.nextline = 0
        }
        self.buffer.eofp = false;
    }

    fn update_heredoc_indent(&mut self, c: &MaybeByte) -> bool {
        if self.buffer.heredoc_line_indent == -1 {
            if *c == b'\n' {
                self.buffer.heredoc_line_indent = 0
            }
        } else if *c == b' ' {
            self.buffer.heredoc_line_indent += 1;
            return true;
        } else if *c == b'\t' {
            let w = (self.buffer.heredoc_line_indent / TAB_WIDTH) + 1;
            self.buffer.heredoc_line_indent = w * TAB_WIDTH;
            return true;
        } else if *c != '\n' {
            if self.buffer.heredoc_indent > self.buffer.heredoc_line_indent {
                self.buffer.heredoc_indent = self.buffer.heredoc_line_indent
            }
            self.buffer.heredoc_line_indent = -1;
        }
        true
    }
}
