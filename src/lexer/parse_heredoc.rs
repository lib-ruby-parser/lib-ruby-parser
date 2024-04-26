use crate::lex_states::*;
use crate::lexer::TokAdd;
use crate::maybe_byte::MaybeByte;
use crate::source::buffer::*;
use crate::str_term::{str_types::*, HeredocEnd, HeredocLiteral, StrTerm};
use crate::Lexer;
use crate::TokenBuf;
use lib_ruby_parser_ast::DiagnosticMessage;

const TAB_WIDTH: i32 = 8;

impl<'b> Lexer<'b> {
    pub(crate) fn heredoc_identifier(&mut self) -> Option<i32> {
        /*
         * term_len is length of `<<"END"` except `END`,
         * in this case term_len is 4 (<, <, " and ").
         */
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
            indent = i32::MAX;
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
                    self.yyerror0(DiagnosticMessage::UnterminatedHeredocId {});
                    return Some(Self::END_OF_INPUT);
                }
            }
        } else {
            if !self.is_identchar() {
                self.buffer.pushback(c);
                if (func & STR_FUNC_INDENT) != 0 {
                    self.buffer.pushback(if indent > 0 { b'~' } else { b'-' });
                }
                return None;
            }
            func |= str_dquote;
            loop {
                let n = self.multibyte_char_len(self.buffer.pcur - 1);
                match n {
                    Some(n) => self.buffer.pcur += n - 1,
                    None => return Some(Self::END_OF_INPUT),
                }
                c = self.nextc();
                if c.is_eof() || !self.is_identchar() {
                    break;
                }
            }
            self.buffer.pushback(c);
        }

        let len = self.buffer.pcur - (self.buffer.pbeg + offset) - quote;

        let id_bytes = self
            .buffer
            .substr_at(self.buffer.ptok, self.buffer.pcur)
            .expect("failed to get heredoc id");
        let mut id = TokenBuf::empty(self.blob);
        id.push_bytes(id_bytes);
        self.set_yylval_str(&mut id);
        self.lval_start = Some(self.buffer.ptok);
        self.lval_end = Some(self.buffer.pcur);

        self.buffer.goto_eol();

        self.strterm = Some(StrTerm::new_heredoc(HeredocLiteral::new(
            self.buffer.lastline.unwrap(),
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

    pub(crate) fn here_document(&mut self) -> i32 {
        let here = match self.strterm.as_ref().unwrap() {
            StrTerm::StringLiteral(_) => unreachable!("strterm must be heredoc"),
            StrTerm::HeredocLiteral(h) => h.clone(),
        };
        self.lval_start = Some(self.buffer.pcur);

        let mut ptr;
        let mut ptr_end;
        let mut str_ = TokenBuf::empty(self.blob);

        let heredoc_end: HeredocEnd;

        let eos = here.lastline.start as u32 + here.offset;
        let len = here.length;
        let func = here.func;
        let indent = here.func & STR_FUNC_INDENT;

        let mut c = self.nextc();
        if c.is_eof() {
            return self.here_document_error(&here, eos, len);
        }
        let bol = self.buffer.was_bol();
        if !bol {
            /* not beginning of line, cannot be the terminator */
        } else if self.buffer.heredoc_line_indent == -1 {
            /* `heredoc_line_indent == -1` means
             * - "after an interpolation in the same line", or
             * - "in a continuing line"
             */
            self.buffer.heredoc_line_indent = 0;
        } else if self.buffer.is_whole_match(
            self.buffer
                .substr_at(eos, eos + len)
                .expect("failed to get heredoc id for comparison"),
            indent,
        ) {
            return self.here_document_restore(&here);
        }

        if (func & STR_FUNC_EXPAND) == 0 {
            loop {
                ptr = self.buffer.lastline.unwrap().start as u32;
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
                    while (ptr + i < ptr_end) && self.update_heredoc_indent(self.char_at(ptr + i)) {
                        i += 1;
                    }
                    self.buffer.heredoc_line_indent = 0;
                }

                match self.buffer.substr_at(ptr, ptr_end) {
                    Some(s) => str_.push_bytes(s),
                    _ => panic!(
                        "no substr {}..{} (len = {})",
                        ptr,
                        ptr_end,
                        self.buffer.input.len()
                    ),
                };
                if ptr_end < self.buffer.pend {
                    str_.push_char('\n')
                }
                self.buffer.goto_eol();
                if self.buffer.heredoc_indent > 0 {
                    return self.heredoc_flush_str(&mut str_);
                }
                if self.nextc().is_eof() {
                    str_.clear();
                    return self.here_document_error(&here, eos, len);
                }

                if self.buffer.is_whole_match(
                    self.buffer
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
                self.buffer.pushback(c);
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
                if c != b'\n' {
                    if c == b'\\' {
                        self.buffer.heredoc_line_indent = -1
                    }
                    return self.heredoc_flush();
                }
                let cc = self.nextc();
                self.tokadd(cc);
                if self.buffer.heredoc_indent > 0 {
                    self.buffer.goto_eol();
                    return self.heredoc_flush();
                }
                c = self.nextc();
                if c.is_eof() {
                    return self.here_document_error(&here, eos, len);
                }

                if self.buffer.is_whole_match(
                    self.buffer
                        .substr_at(eos, eos + len)
                        .expect("failed to get heredoc id for comparison"),
                    indent,
                ) {
                    heredoc_end = self.compute_heredoc_end();

                    break;
                }
            }
            str_ = self.tokenbuf.take();
        }

        self.heredoc_restore(&here);
        self.token_flush();
        self.strterm = self.new_strterm(func | STR_FUNC_TERM, 0, Some(0), Some(heredoc_end));
        self.set_yylval_str(&mut str_);
        Self::tSTRING_CONTENT
    }

    fn compute_heredoc_end(&self) -> HeredocEnd<'b> {
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

        HeredocEnd { start, end, value }
    }

    fn here_document_error(&mut self, here: &HeredocLiteral<'b>, eos: u32, len: u32) -> i32 {
        self.heredoc_restore(here);
        let heredoc_id = core::str::from_utf8(
            self.buffer
                .substr_at(eos, eos + len)
                .expect("failed to get heredoc id for comparison"),
        )
        .unwrap();
        self.compile_error(
            DiagnosticMessage::UnterminatedHeredoc { heredoc_id },
            self.current_loc(),
        );
        self.token_flush();
        self.strterm = None;
        self.lex_state.set(EXPR_END);
        Self::tSTRING_END
    }

    fn here_document_restore(&mut self, here: &HeredocLiteral<'b>) -> i32 {
        let heredoc_end = self.compute_heredoc_end();
        self.lval_start = Some(heredoc_end.start);
        self.lval_end = Some(heredoc_end.end);
        let mut token_buf = TokenBuf::empty(self.blob);
        token_buf.push_bytes(heredoc_end.value);
        self.set_yylval_str(&mut token_buf);

        self.heredoc_restore(here);
        self.token_flush();
        self.strterm = None;
        self.lex_state.set(EXPR_END);

        Self::tSTRING_END
    }

    fn heredoc_flush_str(&mut self, str_: &mut TokenBuf<'b>) -> i32 {
        self.set_yylval_str(str_);
        self.flush_string_content();
        Self::tSTRING_CONTENT
    }

    fn heredoc_flush(&mut self) -> i32 {
        let mut tokenbuf = self.tokenbuf.take();
        self.heredoc_flush_str(&mut tokenbuf)
    }

    fn heredoc_restore(&mut self, here: &HeredocLiteral<'b>) {
        self.strterm = None;
        let line = here.lastline;
        self.buffer.lastline = Some(line);
        self.buffer.pbeg = line.start as u32;
        self.buffer.pend = self.buffer.pbeg + line.len() as u32;
        self.buffer.pcur = self.buffer.pbeg + here.offset + here.length + here.quote;
        self.buffer.ptok = self.buffer.pbeg + here.offset - here.quote;
        self.buffer.heredoc_end = self.buffer.ruby_sourceline;
        self.buffer.ruby_sourceline = here.sourceline;
        if self.buffer.eofp {
            self.buffer.nextline = None
        }
        self.buffer.eofp = false;
    }

    pub(crate) fn update_heredoc_indent(&mut self, c: MaybeByte) -> bool {
        if self.buffer.heredoc_line_indent == -1 {
            if c == b'\n' {
                self.buffer.heredoc_line_indent = 0
            }
        } else if c == b' ' {
            self.buffer.heredoc_line_indent += 1;
            return true;
        } else if c == b'\t' {
            let w = (self.buffer.heredoc_line_indent / TAB_WIDTH) + 1;
            self.buffer.heredoc_line_indent = w * TAB_WIDTH;
            return true;
        } else if c != b'\n' {
            if self.buffer.heredoc_indent > self.buffer.heredoc_line_indent {
                self.buffer.heredoc_indent = self.buffer.heredoc_line_indent
            }
            self.buffer.heredoc_line_indent = -1;
        }
        true
    }
}
