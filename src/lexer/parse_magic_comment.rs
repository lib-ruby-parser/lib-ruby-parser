use crate::source::{MagicComment, MagicCommentKind};
use crate::DiagnosticMessage;
use crate::Lexer;

const MAGIC_COMMENTS: &[(&str, MagicCommentKind)] = &[
    ("coding", MagicCommentKind::Encoding),
    ("encoding", MagicCommentKind::Encoding),
    (
        "frozen_string_literal",
        MagicCommentKind::FrozenStringLiteral,
    ),
    (
        "shareable_constant_value",
        MagicCommentKind::ShareableContstantValue,
    ),
    ("warn_indent", MagicCommentKind::WarnIndent),
];

pub(crate) trait ParseMagicComment {
    fn comment_at_top(&self) -> bool;
    fn set_file_encoding(&mut self, str_: usize, send: usize);
    fn magic_comment_marker(&self, str_: usize, len: usize) -> usize;
    fn magic_comment(&mut self, str_: usize, len: usize) -> Result<bool, ()>;
}

impl ParseMagicComment for Lexer {
    fn comment_at_top(&self) -> bool {
        let mut ptr = self.buffer.pbeg;
        let ptr_end = self.buffer.pcur - 1;
        if self.buffer.line_count != (if self.buffer.has_shebang { 2 } else { 1 }) {
            return false;
        }
        while ptr < ptr_end {
            if !self.char_at(ptr).is_space() {
                return false;
            }
            ptr += 1;
        }
        true
    }

    fn set_file_encoding(&mut self, mut str_: usize, send: usize) {
        let mut sep = false;
        let beg;

        loop {
            if send - str_ <= 6 {
                return;
            }
            match self.char_at(str_ + 6).to_option() {
                Some(b'C') | Some(b'c') => {
                    str_ += 6;
                    continue;
                }
                Some(b'O') | Some(b'o') => {
                    str_ += 5;
                    continue;
                }
                Some(b'D') | Some(b'd') => {
                    str_ += 4;
                    continue;
                }
                Some(b'I') | Some(b'i') => {
                    str_ += 3;
                    continue;
                }
                Some(b'N') | Some(b'n') => {
                    str_ += 2;
                    continue;
                }
                Some(b'G') | Some(b'g') => {
                    str_ += 1;
                    continue;
                }
                Some(b'=') | Some(b':') => {
                    sep = true;
                    str_ += 6;
                }
                _ => {
                    str_ += 6;
                    if self.char_at(str_).is_space() {
                        // nothing
                    } else {
                        continue;
                    }
                }
            }
            if self.buffer.substr_at(str_ - 6, str_) == Some(b"coding") {
                break;
            }
        }
        loop {
            loop {
                str_ += 1;
                if str_ >= send {
                    return;
                }
                if !(self.char_at(str_).is_space()) {
                    break;
                }
            }
            if sep {
                break;
            }
            let c = self.char_at(str_);
            if c != '=' && c != ':' {
                return;
            }
            sep = true;
            str_ += 1;
        }
        beg = str_;

        while self.char_at(str_) == b'-'
            || self.char_at(str_) == b'_'
            || self.char_at(str_).is_alnum() && str_ + 1 < send
        {
            str_ += 1;
        }

        let _enc_name = self
            .buffer
            .substr_at(beg, str_)
            .expect("failed to get encoding comment value");
    }

    fn magic_comment_marker(&self, str_: usize, len: usize) -> usize {
        let mut i = 2;

        while i < len {
            match self.char_at(str_ + i).to_option() {
                Some(b'-') => {
                    if self.char_at(str_ + i - 1) == b'*' && self.char_at(str_ + i - 2) == b'-' {
                        return str_ + i + 1;
                    }
                    i += 2
                }
                Some(b'*') => {
                    if i + 1 >= len {
                        return 0;
                    }
                    if self.char_at(str_ + i + 1) != '-' {
                        i += 4;
                    } else if self.char_at(str_ + i - 1) != '-' {
                        i += 2;
                    } else {
                        return str_ + i + 2;
                    }
                }
                _ => i += 3,
            }
        }
        0
    }

    fn magic_comment(&mut self, mut str_: usize, mut len: usize) -> Result<bool, ()> {
        let mut indicator = false;
        let mut name;
        let mut beg;
        let mut end;
        let mut vbeg;
        let mut vend;

        if len <= 7 {
            return Ok(false);
        }
        beg = self.magic_comment_marker(str_, len);
        if beg != 0 {
            end = self.magic_comment_marker(beg, str_ + len - beg);
            if end == 0 {
                return Ok(false);
            }
            indicator = true;
            str_ = beg;
            len = end - beg - 3;
        }

        while len > 0 {
            let n;

            loop {
                let c = self.char_at(str_);
                if len == 0 || c.is_eof() {
                    break;
                }

                if c == b'\'' || c == b'"' || c == b':' || c == b';' {
                    // noop
                } else {
                    if !c.is_space() {
                        break;
                    }
                    str_ += 1;
                    len -= 1;
                    continue;
                }

                str_ += 1;
                len -= 1;
            }

            beg = str_;
            loop {
                if len == 0 {
                    break;
                }

                let c = self.char_at(str_);
                if c == b'\'' || c == b'"' || c == b':' || c == b';' {
                    // noop
                } else {
                    if c.is_space() {
                        break;
                    }
                    str_ += 1;
                    len -= 1;
                    continue;
                }

                break;
            }

            end = str_;
            loop {
                let c = self.char_at(str_);
                if !(len > 0 && c.is_space()) {
                    break;
                }

                // empty for loop body

                str_ += 1;
                len -= 1;
            }

            if len == 0 {
                break;
            }
            if self.char_at(str_) != ':' {
                if !indicator {
                    return Ok(false);
                }
                continue;
            }

            loop {
                str_ += 1;
                len -= 1;

                if !(len > 0 && self.char_at(str_).is_space()) {
                    break;
                }
            }
            if len == 0 {
                break;
            }
            if self.char_at(str_) == b'"' {
                str_ += 1;
                vbeg = str_;

                loop {
                    let c = self.char_at(str_);
                    len -= 1;
                    if !(len > 0 && c != '"') {
                        break;
                    }

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
                    if !(len > 0 && c != '"' && c != ';' && !c.is_space()) {
                        break;
                    }

                    // empty for loop body

                    len -= 1;
                    str_ += 1;
                }
                vend = str_;
            }
            if indicator {
                while len > 0 && (self.char_at(str_) == b';' || self.char_at(str_).is_space()) {
                    len -= 1;
                    str_ += 1;
                }
            } else {
                while len > 0 && self.char_at(str_).is_space() {
                    len -= 1;
                    str_ += 1;
                }
                if len != 0 {
                    return Ok(false);
                }
            }

            n = end - beg;
            name = String::from_utf8(
                self.buffer
                    .substr_at(beg, beg + n)
                    .expect("failed to get magic comment name")
                    .to_vec(),
            )
            .expect("expected source to be encoded in utf-8");
            let name_to_compare = name.replace("-", "_");
            for (name, kind) in MAGIC_COMMENTS.iter() {
                if &name_to_compare == name {
                    if kind == &MagicCommentKind::Encoding {
                        let encoding = String::from_utf8(
                            self.buffer
                                .substr_at(vbeg, vend)
                                .expect("bug: Can't be None")
                                .to_vec(),
                        )
                        .unwrap();
                        match self.buffer.set_encoding(&encoding) {
                            Ok(_) => {}
                            Err(err) => {
                                self.yyerror1(
                                    DiagnosticMessage::EncodingError(err.to_string()),
                                    self.range(vbeg, vend),
                                );
                                return Err(());
                            }
                        }
                    }

                    let key_l = self.range(beg, beg + n);
                    let value_l = self.range(vbeg, vend);

                    let magic_comment = MagicComment::new(kind.clone(), key_l, value_l);
                    self.magic_comments.push(magic_comment);
                }
            }
        }

        Ok(true)
    }
}
