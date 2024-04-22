use core::convert::TryInto;

use lib_ruby_parser_ast_arena::write_to;

use crate::source::{MagicComment, MagicCommentKind};
use crate::Lexer;
use lib_ruby_parser_ast_arena::DiagnosticMessage;

type MagicCommentData = (&'static str, MagicCommentKind);

const MAGIC_COMMENTS: &[MagicCommentData] = &[
    ("coding", MagicCommentKind::Encoding),
    ("encoding", MagicCommentKind::Encoding),
    (
        "frozen_string_literal",
        MagicCommentKind::FrozenStringLiteral,
    ),
    (
        "frozen-string-literal",
        MagicCommentKind::FrozenStringLiteral,
    ),
    (
        "shareable_constant_value",
        MagicCommentKind::ShareableConstantValue,
    ),
    (
        "shareable-constant-value",
        MagicCommentKind::ShareableConstantValue,
    ),
    ("warn_indent", MagicCommentKind::WarnIndent),
    ("warn-indent", MagicCommentKind::WarnIndent),
];

impl<'b> Lexer<'b> {
    pub(crate) fn comment_at_top(&self) -> bool {
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

    pub(crate) fn set_file_encoding(&mut self, mut str_: u32, send: u32) {
        let mut sep = false;

        loop {
            if send - str_ <= 6 {
                return;
            }
            match self.char_at(str_ + 6).as_option() {
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
            sep = false;
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
            if c != b'=' && c != b':' {
                return;
            }
            sep = true;
            str_ += 1;
        }
        let beg = str_;

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

    fn magic_comment_marker(&self, str_: u32, len: u32) -> u32 {
        let mut i = 2;

        while i < len {
            match self.char_at(str_ + i).as_option() {
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
                    if self.char_at(str_ + i + 1) != b'-' {
                        i += 4;
                    } else if self.char_at(str_ + i - 1) != b'-' {
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

    pub(crate) fn magic_comment(&mut self, mut str_: u32, mut len: u32) -> Result<bool, ()> {
        let mut indicator = false;
        let mut end;
        let mut vbeg;
        let mut vend;

        if len <= 7 {
            return Ok(false);
        }
        let mut beg = self.magic_comment_marker(str_, len);
        if beg != 0 {
            end = self.magic_comment_marker(beg, str_ + len - beg);
            if end == 0 {
                return Ok(false);
            }
            indicator = true;
            str_ = beg;
            len = end - beg - 3;
        }

        let mut len: i32 = len.try_into().unwrap();

        while len > 0 {
            loop {
                let c = self.char_at(str_);
                if !(len > 0 && c.is_some()) {
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
                if len <= 0 {
                    break;
                }

                let c = self.char_at(str_);
                if c == b'\'' || c == b'"' || c == b':' || c == b';' {
                    // noop
                } else {
                    if c.is_space() {
                        // break from C switch;
                    } else {
                        str_ += 1;
                        len -= 1;
                        continue;
                    }
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
            if self.char_at(str_) != b':' {
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
                    if !(len > 0 && c != b'"') {
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
                    if !(len > 0 && c != b'"' && c != b';' && !c.is_space()) {
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

            let n = end - beg;
            let name_to_compare = core::str::from_utf8(
                self.buffer
                    .substr_at(beg, beg + n)
                    .expect("failed to get magic comment name"),
            )
            .map_err(|_| ())?;

            for (name, kind) in MAGIC_COMMENTS.iter() {
                if &name_to_compare == name {
                    if kind == &MagicCommentKind::Encoding && self.comment_at_top() {
                        let encoding_bytes = self
                            .buffer
                            .substr_at(vbeg, vend)
                            .expect("bug: Can't be None");
                        let encoding = match core::str::from_utf8(encoding_bytes) {
                            Ok(encoding) => encoding,
                            Err(err) => {
                                let encoding_name =
                                    core::str::from_utf8(&encoding_bytes[..err.valid_up_to()])
                                        .unwrap();

                                debug_assert!(encoding_name.len() < 20);
                                let mut mem = [0; 50];
                                let error = write_to(
                                    &mut mem,
                                    format_args!("unknown encoding name: {}", encoding_name),
                                )
                                .unwrap();
                                let error = self.blob.push_str(error);

                                self.yyerror1(
                                    DiagnosticMessage::EncodingError { error },
                                    self.loc(vbeg, vend),
                                );

                                return Err(());
                            }
                        };
                        match self.buffer.set_encoding(encoding) {
                            Ok(_) => {}
                            Err(err) => {
                                self.yyerror1(
                                    DiagnosticMessage::EncodingError {
                                        error: err.as_str(),
                                    },
                                    self.loc(vbeg, vend),
                                );
                                return Err(());
                            }
                        }
                    }

                    let key_l = self.loc(beg, beg + n);
                    let value_l = self.loc(vbeg, vend);

                    let magic_comment = self.blob.alloc_mut::<MagicComment>();
                    *magic_comment = MagicComment::new(*kind, key_l, value_l);
                    self.magic_comments.push(&*magic_comment);
                }
            }
        }

        Ok(true)
    }
}
