use crate::Lexer;
use crate::lexer::{StrTerm, HeredocLiteral, StringLiteral};
use crate::lexer::lex_char::LexChar;
use crate::lexer::lex_states::*;
use crate::lexer::str_types::*;
use crate::parser::TokenValue;

impl Lexer {
    pub const TAB_WIDTH: i32 = 8;

    pub fn parse_string(&mut self, quote: StringLiteral) -> i32 {
        let func = quote.func();
        let term = quote.term();
        let paren = quote.paren();
        let mut c: LexChar;
        let mut space = false;

        if self.debug { println!("func = {}, pcur = {}, ptok = {}", func, self.buffer.pcur, self.buffer.ptok); }

        if (func & STR_FUNC_TERM) != 0 {
            if (func & STR_FUNC_QWORDS) != 0 { self.nextc(); } /* delayed term */
            self.set_lex_state(EXPR_END);
            self.strterm = None;
            if (func & STR_FUNC_REGEXP) != 0 {
                return Self::tREGEXP_END
            } else {
                return Self::tSTRING_END;
            }
        }
        c = self.nextc();
        if (func & STR_FUNC_QWORDS) != 0 && c.is_space() {
            loop {
                c = self.nextc();

                if !c.is_space() { break }
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
            return self.parser_string_term(func);
        }
        if space {
            self.buffer.pushback(&c);
            return Self::tSPACE;
        }
        self.newtok();
        if ((func & STR_FUNC_EXPAND) != 0) && c == '#' {
            if let Some(t) = self.parser_peek_variable_name() {
                return t;
            }
            self.tokadd(&LexChar::Some('#'));
            c = self.nextc();
        }
        self.buffer.pushback(&c);

        let mut nest = quote.nest();
        let added = self.tokadd_string(func, term, paren, &mut nest);
        quote.set_nest(nest);

        if added.is_some() {
            if self.buffer.eofp {
                self.literal_flush(self.buffer.pcur);
                if (func & STR_FUNC_QWORDS) != 0 {
                    /* no content to add, bailing out here */
                    self.yyerror0("unterminated list meets end of file");
                    self.strterm = None;
                    return Self::tSTRING_END;
                }
                if (func & STR_FUNC_REGEXP) != 0 {
                    self.yyerror0("unterminated regexp meets end of file");
                } else {
                    self.yyerror0("unterminated string meets end of file");
                }
                quote.set_func(quote.func() | STR_FUNC_TERM);
            }
        }

        self.tokfix();
        self.set_yylval_str(self.tok());
        self.flush_string_content();

        Self::tSTRING_CONTENT
    }

    pub fn parser_string_term(&mut self, func: usize) -> i32 {
        self.strterm = None;
        if (func & STR_FUNC_REGEXP) != 0 {
            let flags = self.regx_options();
            self.set_yylval_num(flags);
            self.set_lex_state(EXPR_END);
            return Self::tREGEXP_END;
        }
        if (func & STR_FUNC_LABEL) != 0 && self.is_label_suffix(0) {
            self.nextc();
            self.set_lex_state(EXPR_BEG|EXPR_LABEL);
            return Self::tLABEL_END;
        }
        self.set_lex_state(EXPR_END);
        return Self::tSTRING_END;
    }

    pub fn set_yylval_num(&mut self, flags: String) {
        if self.debug { println!("set_yylval_num {:#?}", flags); }
        self.lval = Some(TokenValue::String(flags));
    }

    pub fn regx_options(&mut self) -> String {
        let mut c: LexChar;
        let mut result = String::from("");

        self.newtok();
        loop {
            c = self.nextc();
            if !c.is_alpha() { break }

            let ch =  c.unwrap();

            match ch {
                'o' | 'n' | 'e' | 's' | 'u' | 'i' | 'x' | 'm' => {
                    result.push(ch);
                },
                _ => {
                    self.tokadd(&c);
                }
            }
        }

        self.buffer.pushback(&c);
        if self.toklen() > 0 {
            self.tokfix();
            self.compile_error(&format!("unknown regexp options - {:#?}", self.tok()));
        }

        return result;
    }

    pub fn parser_peek_variable_name(&mut self) -> Option<i32> {
        let mut c: LexChar;
        let mut ptr: usize = self.buffer.pcur;

        if ptr + 1 >= self.buffer.pend { return None }
        c = self.char_at(ptr);
        ptr += 1;

        match c {
            LexChar::Some('$') => {
                c = self.char_at(ptr);
                if c == '-' {
                    ptr += 1;
                    if ptr >= self.buffer.pend { return None }
                    // c = self.char_at(ptr);
                } else if c.is_global_name_punct() || c.is_digit() {
                    return Some(Self::tSTRING_DVAR);
                }
            },

            LexChar::Some('@') => {
                c = self.char_at(ptr);
                if c == '@' {
                    ptr += 1;
                    if ptr >= self.buffer.pend { return None }
                    // c = self.char_at(ptr);
                }
            },

            LexChar::Some('{') => {
                self.buffer.pcur = ptr;
                self.command_start = true;
                return Some(Self::tSTRING_DBEG)
            },

            _ => return None
        }

        None
    }

    pub fn tokadd_string(&mut self, func: usize, term: char, paren: Option<char>, nest: &mut usize) -> Option<char> {
        let mut c;
        let _erred = false;

        loop {
            c = self.nextc();
            if c.is_eof() { break; }

            if self.buffer.heredoc_indent > 0 {
                self.parser_update_heredoc_indent(&c);
            }

            if c == paren {
                *nest += 1;
                // literal.set_nest(literal.nest() + 1);
            } else if c == term {
                if *nest == 0 {
                    self.buffer.pushback(&c);
                    break;
                }
                // literal.set_nest(literal.nest() - 1);
                *nest -= 1;
            } else if ((func & STR_FUNC_EXPAND) != 0) && c == '#' && self.buffer.pcur < self.buffer.pend {
                let c2 = self.char_at(self.buffer.pcur);
                if c2 == '$' || c2 == '@' || c2 == '{' {
                    self.buffer.pushback(&c);
                    break;
                }
            } else if c == '\\' {
                self.literal_flush(self.buffer.pcur - 1);
                c = self.nextc();
                match c {
                    LexChar::Some('\n') => {
                        if (func & STR_FUNC_QWORDS) != 0 { break }
                        if (func & STR_FUNC_EXPAND) != 0 {
                            if (func & STR_FUNC_INDENT) == 0 || self.buffer.heredoc_indent < 0 {
                                continue;
                            }
                            if c == term {
                                c = LexChar::Some('\\');
                                return c.to_option();
                            }
                        }
                        self.tokadd(&LexChar::Some('\\'));
                        break;
                    },

                    LexChar::Some('\\') => {
                        if (func & STR_FUNC_ESCAPE) != 0 { self.tokadd(&c) }
                        break;
                    },

                    LexChar::Some('u') => {
                        if (func & STR_FUNC_EXPAND) == 0 {
                            self.tokadd(&LexChar::Some('\\'));
                            break;
                        }
                        self.tokadd_utf8(term, func & STR_FUNC_SYMBOL, func & STR_FUNC_REGEXP);
                        continue;
                    },

                    LexChar::EOF => {
                        return None;
                    },

                    LexChar::Some(_) => {
                        if !c.is_ascii() {
                            if (func & STR_FUNC_EXPAND) == 0 {
                                self.tokadd(&LexChar::Some('\\'));
                                // goto non_ascii (inlined)
                                unimplemented!("non_ascii1");
                            }
                            if (func & STR_FUNC_REGEXP) != 0 {
                                if c == term && !self.is_simple_re_match(&c) {
                                    self.tokadd(&c);
                                    continue;
                                }
                                self.buffer.pushback(&c);
                                c = self.tokadd_escape();
                                if c.is_eof() {
                                    return None;
                                }
                                // TODO: compare encodings
                                continue;
                            } else if (func & STR_FUNC_EXPAND) != 0 {
                                self.buffer.pushback(&c);
                                if (func & STR_FUNC_ESCAPE) != 0 { self.tokadd(&LexChar::Some('\\')) }
                                c = self.read_escape(0);
                            } else if (func & STR_FUNC_QWORDS) != 0 && c.is_space() {
                                // ignore backslashed spaces in %w
                            } else if c != term && c != paren {
                                self.tokadd(&LexChar::Some('\\'));
                                self.buffer.pushback(&c);
                                continue;
                            }
                        }
                    }
                }
            } else if !self.parser_is_ascii() {
                unimplemented!("non_ascii1");
            } else if (func & STR_FUNC_QWORDS) != 0 && c.is_space() {
                self.buffer.pushback(&c);
                break;
            }
            if let LexChar::Some(c) = c {
                if (c as u8 & 0x80) != 0 {
                    unimplemented!("part of non_ascii related to encodings");
                }
            }
            self.tokadd(&c);
        }

        c.to_option()
    }
    pub fn set_yylval_str(&mut self, value: String) {
        if self.debug { println!("set_yylval_str {:#?}", &value); }
        self.lval = Some(TokenValue::String(value));
    }

    pub fn flush_string_content(&mut self) {
        // noop
    }

    pub fn parser_update_heredoc_indent(&mut self, c: &LexChar) -> bool {
        if self.buffer.heredoc_line_indent == -1 {
            if *c == '\n' { self.buffer.heredoc_line_indent = 0 }
        } else {
            if *c == ' ' {
                self.buffer.heredoc_line_indent += 1;
                return true;
            } else if *c == '\t' {
                let w = (self.buffer.heredoc_line_indent / Self::TAB_WIDTH) + 1;
                self.buffer.heredoc_line_indent = w * Self::TAB_WIDTH;
                return true;
            } else if *c != '\n' {
                if self.buffer.heredoc_indent > self.buffer.heredoc_line_indent {
                    self.buffer.heredoc_indent = self.buffer.heredoc_line_indent
                }
                self.buffer.heredoc_line_indent = -1;
            }
        }
        true
    }

    pub fn tokadd_utf8(&mut self, _c: char, _func1: usize, _func2: usize) {
        unimplemented!("tokadd_utf8")
    }

    pub fn is_simple_re_match(&mut self, _c: &LexChar) -> bool {
        unimplemented!("is_simple_re_match")
    }

    pub fn tokadd_escape(&mut self) -> LexChar {
        unimplemented!("tokadd_escape")
    }

    pub fn read_escape(&mut self, _x: usize) -> LexChar {
        unimplemented!("read_escape")
    }

    pub fn parser_is_ascii(&self) -> bool {
        self.char_at(self.buffer.pcur - 1).is_ascii()
    }

    pub fn heredoc_identifier(&mut self) -> Option<i32> {
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

        if c == '-' {
            c = self.nextc();
            func = STR_FUNC_INDENT;
            offset += 1;
        } else if c == '~' {
            c = self.nextc();
            func = STR_FUNC_INDENT;
            offset += 1;
            indent = std::i32::MAX;
        }

        if c == '\'' || c == '"' || c == '`' {
            if c == '\'' { func |= str_squote }
            if c == '"'  { func |= str_dquote }
            if c == '`'  { func |= str_xquote; token = Self::tXSTRING_BEG }

            quote += 1;
            offset += 1;
            term = c;

            loop {
                c = self.nextc();
                if c == term { break }

                if c.is_eof() || c == '\r' || c == '\n' {
                    self.yyerror0("unterminated here document identifier");
                    return None;
                }
            }
        } else {
            if !self.parser_is_identchar() {
                self.buffer.pushback(&c);
                if (func & STR_FUNC_INDENT) != 0 {
                    self.buffer.pushback(&LexChar::Some(if indent > 0 { '~' } else { '-' }));
                }
                return Some(Self::END_OF_INPUT);
            }
            func |= str_dquote;
            loop {
                if let Some(n) = self.parser_precise_mbclen(self.buffer.pcur - 1) {
                    self.buffer.pcur += n - 1;
                } else {
                    return Some(Self::END_OF_INPUT)
                }
                c = self.nextc();
                if c.is_eof() || !self.parser_is_identchar() { break }
            }
            self.buffer.pushback(&c);
        }

        len = self.buffer.pcur - (self.buffer.pbeg + offset) - quote;
        self.buffer.goto_eol();

        self.strterm = Some(
            StrTerm::new_heredoc(
                HeredocLiteral::new(
                    self.buffer.lastline,
                    offset,
                    self.buffer.ruby_sourceline,
                    len,
                    quote,
                    func
                )
            )
        );

        self.token_flush();
        self.buffer.heredoc_indent = indent;
        self.buffer.heredoc_line_indent = 0;
        return Some(token);
    }

    pub fn here_document(&mut self, here: HeredocLiteral) -> i32 {
        let mut c;
        let func;
        let indent;
        let eos;
        let mut ptr;
        let mut ptr_end;
        let len;
        let mut str_: String;
        // let enc = self.p.enc;
        // let base_enc = 0;
        let bol;

        eos = self.buffer.lines[here.lastline()].start + here.offset();
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
        } else if self.buffer.is_whole_match(&self.buffer.substr_at(eos, eos+len).unwrap(), indent) {
            return self.here_document_restore(&here);
        }

        if (func & STR_FUNC_EXPAND) == 0 {
            loop {
                ptr = self.buffer.lines[self.buffer.lastline].start;
                ptr_end = self.buffer.pend;
                if ptr_end > ptr {
                    match self.buffer.input[ptr_end - 1] {
                        '\n' => {
                            ptr_end -= 1;
                            if ptr_end == ptr || self.buffer.input[ptr_end - 1] != '\r' {
                                ptr_end += 1;
                            } else {
                                ptr_end -= 1;
                            }
                        },
                        '\r' => {
                            ptr_end -= 1;
                        },
                        _ => {}
                    }
                }

                if self.buffer.heredoc_indent > 0 {
                    let mut i = 0;
                    while (ptr + i < ptr_end) && self.parser_update_heredoc_indent(&self.char_at(ptr + i)) {
                        i += 1;
                    }
                    self.buffer.heredoc_line_indent = 0;
                }

                str_ = self.buffer.substr_at(ptr, ptr_end).unwrap();
                if ptr_end < self.buffer.pend { str_.push('\n') }
                self.buffer.goto_eol();
                if self.buffer.heredoc_indent > 0 {
                    return self.heredoc_flush_str(str_);
                }
                if self.nextc().is_eof() {
                    str_.clear();
                    return self.here_document_error(&here, eos, len);
                }

                if self.buffer.is_whole_match(&self.buffer.substr_at(eos, eos+len).unwrap(), indent) {
                    break;
                }
            }
        } else {
            self.newtok();
            if c == '#' {
                let t = self.parser_peek_variable_name();
                if self.buffer.heredoc_line_indent != -1 {
                    if self.buffer.heredoc_indent > self.buffer.heredoc_line_indent {
                        self.buffer.heredoc_indent = self.buffer.heredoc_line_indent;
                    }
                    self.buffer.heredoc_line_indent = -1;
                }
                if let Some(t) = t { return t }
                self.tokadd(&LexChar::Some('#'));
                c = self.nextc();
            }
            loop {
                self.buffer.pushback(&c);
                // enc = self.p.enc;
                if self.tokadd_string(func, '\n', None, &mut 0).is_none() {
                    if self.buffer.eofp { return self.here_document_error(&here, eos, len) }
                    return self.here_document_restore(&here);
                }
                if c != '\n' {
                    if c == '\\' { self.buffer.heredoc_line_indent = -1 }
                    return self.heredoc_flush();
                }
                let cc = self.nextc();
                self.tokadd(&cc);
                if self.buffer.heredoc_indent > 0 {
                    self.buffer.goto_eol();
                    return self.heredoc_flush();
                }
                c = self.nextc();
                println!("eos = {}, len = {}", eos, len);
                if c.is_eof() { return self.here_document_error(&here, eos, len) }

                if self.buffer.is_whole_match(&self.buffer.substr_at(eos, eos+len).unwrap(), indent) {
                    break;
                }
            }
            str_ = self.tok();
        }

        self.heredoc_restore(&here);
        self.token_flush();
        self.strterm = self.new_strterm(func | STR_FUNC_TERM, 0 as char, Some(0 as char));
        self.set_yylval_str(str_);
        return Self::tSTRING_CONTENT;
    }

    pub fn here_document_error(&mut self, here: &HeredocLiteral, eos: usize, len: usize) -> i32 {
        self.heredoc_restore(&here);
        self.compile_error(&format!("can't find string \"{:#?}\" anywhere before EOF", self.buffer.substr_at(eos, eos+len)));
        self.token_flush();
        self.strterm = None;
        self.set_lex_state(EXPR_END);
        return Self::tSTRING_END;
    }

    pub fn here_document_restore(&mut self, here: &HeredocLiteral) -> i32 {
        self.heredoc_restore(&here);
        self.token_flush();
        self.strterm = None;
        self.set_lex_state(EXPR_END);
        return Self::tSTRING_END;
    }

    pub fn heredoc_flush_str(&mut self, str_: String) -> i32 {
        self.set_yylval_str(str_);
        self.flush_string_content();
        return Self::tSTRING_CONTENT;
    }

    pub fn heredoc_flush(&mut self) -> i32 {
        let str_ = self.tok();
        return self.heredoc_flush_str(str_)
    }

    pub fn heredoc_restore(&mut self, here: &HeredocLiteral) {
        self.strterm = None;
        let line = here.lastline();
        self.buffer.lastline = line;
        self.buffer.pbeg = self.buffer.lines[line].start;
        self.buffer.pend = self.buffer.pbeg + self.buffer.lines[line].len();
        self.buffer.pcur = self.buffer.pbeg + here.offset() + here.length() + here.quote();
        self.buffer.ptok = self.buffer.pbeg + here.offset() - here.quote();
        self.buffer.heredoc_end = self.buffer.ruby_sourceline;
        self.buffer.ruby_sourceline = here.sourceline();
        if self.buffer.eofp { self.buffer.nextline = 0 }
        self.buffer.eofp = false;
    }

}
