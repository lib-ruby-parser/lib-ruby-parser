use crate::Lexer;
use crate::lexer::lex_char::LexChar;

struct MagicComment {
    pub name: &'static str
}

const MAGIC_COMMENTS: [MagicComment; 4] = [
    MagicComment { name: "coding" },
    MagicComment { name: "encoding" },
    MagicComment { name: "frozen_string_literal" },
    MagicComment { name: "warn_indent" },
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
                LexChar::Some(b'C') | LexChar::Some(b'c') => { str_ += 6; continue; },
                LexChar::Some(b'O') | LexChar::Some(b'o') => { str_ += 5; continue; },
                LexChar::Some(b'D') | LexChar::Some(b'd') => { str_ += 4; continue; },
                LexChar::Some(b'I') | LexChar::Some(b'i') => { str_ += 3; continue; },
                LexChar::Some(b'N') | LexChar::Some(b'n') => { str_ += 2; continue; },
                LexChar::Some(b'G') | LexChar::Some(b'g') => { str_ += 1; continue; },
                LexChar::Some(b'=') | LexChar::Some(b':') => {
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
            if self.buffer.input[str_-6..str_] == b"coding"[..] {
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
            if c != b'=' && c != b':' { return }
            sep = true;
            str_ += 1;
        }
        beg = str_;

        while self.char_at(str_) == b'-' || self.char_at(str_) == b'_' || self.char_at(str_).is_alnum() && str_ + 1 < send {
            str_ += 1;
        }

        let enc_name = self.buffer.input[beg..str_].to_vec();
        println!("enc = {}", String::from_utf8(enc_name).unwrap());

    }

    pub fn magic_comment_marker(&self, str_: usize, len: usize) -> usize {
        let mut i = 2;

        while i < len {
            match self.char_at(str_ + i) {
                LexChar::Some(b'-') => {
                    if self.char_at(str_ + i - 1) == b'*' && self.char_at(str_ + i - 2) == b'-' {
                        return str_ + i + 1;
                    }
                    i += 2
                },
                LexChar::Some(b'*') => {
                    if i + 1 >= len { return 0 }
                    if self.char_at(str_ + i + 1) != b'-' {
                        i += 4;
                    } else if self.char_at(str_ + i - 1) != b'-' {
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
            let mut mc_idx = 0;
            let n;

            loop {
                let c = self.char_at(str_);
                if !( len > 0 && !c.is_eof() ) { break }

                if c == b'\'' || c == b'"' || c == b':' || c == b';' {
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
                if c == b'\'' || c == b'"' || c == b':' || c == b';' {
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
            if self.char_at(str_) != b':' {
                if !indicator { return false }
                continue;
            }

            loop {
                str_ += 1;
                len -= 1;

                if !( len > 0 && self.char_at(str_).is_space() ) { break }
            }
            if len == 0 { break }
            if self.char_at(str_) == b'"' {
                str_ += 1;
                vbeg = str_;

                loop {
                    let c = self.char_at(str_);
                    len -= 1;
                    if !( len > 0 && c != b'"' ) { break }

                    if c == b'\\' {
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
                    if !( len > 0 && c != b'"' && c != b';' && !c.is_space() ) { break }

                    // empty for loop body

                    len -= 1; str_ += 1;
                }
                vend = str_;
            }
            if indicator {
                while len > 0 && (self.char_at(str_) == b';' || self.char_at(str_).is_space()) { len -= 1; str_ += 1; }
            } else {
                while len > 0 && self.char_at(str_).is_space() { len -= 1; str_ += 1; }
                if len != 0 { return false }
            }

            n = end - beg;
            name = self.buffer.input[beg..beg+n].to_vec();
            for c in name.iter_mut() {
                if *c == b'-' { *c = b'_' }
            }
            loop {
                let mc = MAGIC_COMMENTS[mc_idx].name;
                if n < mc.len() && mc.as_bytes()[0..n] == name[0..n] {
                    match mc {
                        "coding" | "encoding" => {

                        },
                        "frozen_string_literal" => {}
                        "warn_indent" => {}
                        _ => {}
                    }
                    println!("magic comment loc: {}..{}", vbeg, vend);
                    break
                }

                mc_idx += 1;
                if !( mc_idx < MAGIC_COMMENTS.len() ) { break }
            }

            println!("magic comment {:#?}", String::from_utf8(name.clone()).unwrap());
        };

        false
    }
}
