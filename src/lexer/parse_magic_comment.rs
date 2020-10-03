use crate::Lexer;
use crate::lexer::lex_char::LexChar;

const MAGIC_COMMENTS: [&'static str; 4] = [
    "coding",
    "encoding",
    "frozen_string_literal",
    "warn_indent",
];

impl Lexer {
    pub fn comment_at_top(&self) -> bool {
        let mut ptr = self.buffer.pbeg;
        let ptr_end = self.buffer.pcur - 1;
        if self.buffer.line_count != (if self.has_shebang { 2 } else { 1 }) { return false }
        while ptr < ptr_end {
            if !self.char_at(ptr).is_space() { return false }
            ptr += 1;
        }
        return true;
    }

    pub fn set_file_encoding(&mut self, mut str_: usize, send: usize) {
        let mut sep = false;
        let beg;

        loop {
            if send - str_ <= 6 { return }
            match self.char_at(str_ + 6) {
                LexChar::Some('C') | LexChar::Some('c') => { str_ += 6; continue; },
                LexChar::Some('O') | LexChar::Some('o') => { str_ += 5; continue; },
                LexChar::Some('D') | LexChar::Some('d') => { str_ += 4; continue; },
                LexChar::Some('I') | LexChar::Some('i') => { str_ += 3; continue; },
                LexChar::Some('N') | LexChar::Some('n') => { str_ += 2; continue; },
                LexChar::Some('G') | LexChar::Some('g') => { str_ += 1; continue; },
                LexChar::Some('=') | LexChar::Some(':') => {
                    sep = true;
                    str_ += 6;
                },
                _ => {
                    str_ += 6;
                    if self.char_at(str_).is_space(){
                        // nothing
                    } else {
                        continue;
                    }
                }
            }
            if self.buffer.substr_at(str_-6, str_) == Some("coding".to_owned()) {
                break;
            }
        }
        loop {
            loop {
                str_ += 1;
                if str_ >= send { return }
                if !( self.char_at(str_).is_space() ) { break }
            }
            if sep { break }
            let c = self.char_at(str_);
            if c != '=' && c != ':' { return }
            sep = true;
            str_ += 1;
        }
        beg = str_;

        while self.char_at(str_) == '-' || self.char_at(str_) == '_' || self.char_at(str_).is_alnum() && str_ + 1 < send {
            str_ += 1;
        }

        let enc_name = self.buffer.substr_at(beg, str_).unwrap();
        println!("enc = {:#?}", enc_name);

    }

    pub fn magic_comment_marker(&self, str_: usize, len: usize) -> usize {
        let mut i = 2;

        while i < len {
            match self.char_at(str_ + i) {
                LexChar::Some('-') => {
                    if self.char_at(str_ + i - 1) == '*' && self.char_at(str_ + i - 2) == '-' {
                        return str_ + i + 1;
                    }
                    i += 2
                },
                LexChar::Some('*') => {
                    if i + 1 >= len { return 0 }
                    if self.char_at(str_ + i + 1) != '-' {
                        i += 4;
                    } else if self.char_at(str_ + i - 1) != '-' {
                        i += 2;
                    } else {
                        return str_ + i + 2;
                    }
                }
                _ => i += 3
            }
        }
        0
    }

    pub fn parser_magic_comment(&self, mut str_: usize, mut len: usize) -> bool {
        let mut indicator = false;
        let mut name;
        let mut beg;
        let mut end;
        let mut vbeg;
        let mut vend;

        if len <= 7 { return false }
        beg = self.magic_comment_marker(str_, len);
        if beg != 0 {
            end = self.magic_comment_marker(beg, str_ + len - beg);
            if end == 0 {
                return false;
            }
            indicator = true;
            str_ = beg;
            len = end - beg - 3;
        }

        while len > 0 {
            let n;

            loop {
                let c = self.char_at(str_);
                if !( len > 0 && !c.is_eof() ) { break }

                if c == '\'' || c == '"' || c == ':' || c == ';' {
                    // noop
                } else {
                    if !c.is_space() { break }
                    str_ += 1; len -= 1;
                    continue;
                }

                str_ += 1; len -= 1;
            }

            beg = str_;
            loop {
                if !( len > 0 ) { break }

                let c = self.char_at(str_);
                if c == '\'' || c == '"' || c == ':' || c == ';' {
                    // noop
                } else {
                    if c.is_space() { break }
                    str_ += 1; len -= 1;
                    continue;
                }

                break;
            }

            end = str_;
            loop {
                let c = self.char_at(str_);
                if !( len > 0 && c.is_space() ) { break }

                // empty for loop body

                str_ += 1; len -= 1;
            }

            if len == 0 { break }
            if self.char_at(str_) != ':' {
                if !indicator { return false }
                continue;
            }

            loop {
                str_ += 1;
                len -= 1;

                if !( len > 0 && self.char_at(str_).is_space() ) { break }
            }
            if len == 0 { break }
            if self.char_at(str_) == '"' {
                str_ += 1;
                vbeg = str_;

                loop {
                    let c = self.char_at(str_);
                    len -= 1;
                    if !( len > 0 && c != '"' ) { break }

                    if c == '\\' {
                        len -= 1;
                        str_ += 1;
                    }

                    str_ += 1;
                }

                vend = str_;
                if len != 0 {
                    len -= 1;
                    str_ += 1;
                }
            } else {
                vbeg = str_;
                loop {
                    let c = self.char_at(str_);
                    if !( len > 0 && c != '"' && c != ';' && !c.is_space() ) { break }

                    // empty for loop body

                    len -= 1; str_ += 1;
                }
                vend = str_;
            }
            if indicator {
                while len > 0 && (self.char_at(str_) == ';' || self.char_at(str_).is_space()) { len -= 1; str_ += 1; }
            } else {
                while len > 0 && self.char_at(str_).is_space() { len -= 1; str_ += 1; }
                if len != 0 { return false }
            }

            n = end - beg;
            name = self.buffer.substr_at(beg, beg + n).unwrap();
            name = name.replace("-", "_");
            for known in MAGIC_COMMENTS.iter() {
                if &&name == known {
                    // TODO: emit magic comment
                    println!("magic comment {:#?} {:#?}", name, self.buffer.substr_at(vbeg, vend).unwrap());
                }
            }
        };

        false
    }
}
