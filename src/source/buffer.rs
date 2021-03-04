use crate::maybe_byte::*;
use crate::source::input::Input;
use crate::source::{CustomDecoder, InputError};
use std::convert::TryFrom;

#[derive(Debug, Default)]
pub(crate) struct Buffer {
    pub input: Input,

    pub(crate) line_count: usize,
    pub(crate) prevline: Option<usize>, // index
    pub(crate) lastline: usize,         // index
    pub(crate) nextline: usize,         // index
    pub(crate) pbeg: usize,
    pub(crate) pcur: usize,
    pub(crate) pend: usize,
    pub(crate) ptok: usize,

    pub(crate) eofp: bool,
    pub(crate) cr_seen: bool,

    pub(crate) heredoc_end: usize,
    pub(crate) heredoc_indent: i32,
    pub(crate) heredoc_line_indent: i32,

    pub(crate) tokidx: usize,
    pub(crate) toksize: usize,
    pub(crate) tokline: usize,

    pub(crate) has_shebang: bool,

    pub(crate) ruby_sourceline: usize,     /* current line no. */
    pub(crate) ruby_sourcefile: Vec<char>, /* current source file */
    pub(crate) ruby_sourcefile_string: Vec<char>,

    pub(crate) debug: bool,
}

impl Buffer {
    const CTRL_Z_CHAR: char = 0x1a as char;
    const CTRL_D_CHAR: char = 0x04 as char;

    pub fn new(name: &str, bytes: Vec<u8>, decoder: Option<Box<dyn CustomDecoder>>) -> Self {
        let mut input = Input::new(name, decoder);

        input.set_bytes(bytes);

        let mut this = Self {
            input,
            ..Self::default()
        };

        this.prepare();

        this
    }

    fn prepare(&mut self) {
        let c = self.nextc();
        match c.to_option() {
            Some(b'#') => {
                if self.peek(b'!') {
                    self.has_shebang = true;
                }
            }
            Some(0xef) => {
                // handle UTF-8 BOM marker
                if self.pend - self.pcur >= 2
                    && self.byte_at(self.pcur) == 0xbb
                    && self.byte_at(self.pcur + 1) == 0xbf
                {
                    self.pcur += 2;
                    self.pbeg = self.pcur;
                    return;
                }
            }
            None => return,
            _ => {}
        }

        self.pushback(&c)
    }

    pub(crate) fn nextc(&mut self) -> MaybeByte {
        if self.pcur == self.pend || self.eofp || self.nextline != 0 {
            let n = self.nextline();
            if self.debug {
                println!("nextline = {:?}", n);
            }
            if n.is_err() {
                return MaybeByte::EndOfInput;
            }
        }
        let mut c = match self.input.byte_at(self.pcur) {
            Some(c) => c,
            None => return MaybeByte::EndOfInput,
        };
        self.pcur += 1;
        if c == b'\r' {
            c = self.parser_cr(c);
        }
        if self.debug {
            println!("nextc = {:?}", c);
        }
        MaybeByte::new(c)
    }

    pub(crate) fn goto_eol(&mut self) {
        self.pcur = self.pend;
    }

    pub(crate) fn is_eol(&self) -> bool {
        self.pcur >= self.pend
    }

    pub(crate) fn is_eol_n(&self, n: usize) -> bool {
        self.pcur + n >= self.pend
    }

    pub(crate) fn peek(&self, c: u8) -> bool {
        self.peek_n(c, 0)
    }
    pub(crate) fn peek_n(&self, c: u8, n: usize) -> bool {
        !self.is_eol_n(n) && c == self.input.unchecked_byte_at(self.pcur + n)
    }
    pub(crate) fn peekc_n(&self, n: usize) -> MaybeByte {
        if self.is_eol_n(n) {
            MaybeByte::EndOfInput
        } else {
            self.byte_at(self.pcur + n)
        }
    }

    pub(crate) fn nextline(&mut self) -> Result<(), ()> {
        let mut v = self.nextline;
        self.nextline = 0;

        if v == 0 {
            if self.eofp {
                return Err(());
            }

            if self.pend > self.pbeg && self.input.unchecked_byte_at(self.pend - 1) != b'\n' {
                self.eofp = true;
                self.goto_eol();
                return Err(());
            }

            match self.getline() {
                Ok(line) => v = line,
                Err(_) => {
                    self.eofp = true;
                    self.goto_eol();
                    return Err(());
                }
            }

            self.cr_seen = false;
        }
        // TODO: after here-document without terminator

        let line = self.input.line_at(v);

        if self.heredoc_end > 0 {
            self.ruby_sourceline = self.heredoc_end;
            self.heredoc_end = 0;
        }
        self.ruby_sourceline += 1;
        self.pbeg = line.start;
        self.pcur = line.start;
        self.pend = line.end;
        self.token_flush();
        self.prevline = Some(self.lastline);
        self.lastline = v;

        Ok(())
    }

    pub(crate) fn getline(&mut self) -> Result<usize, ()> {
        if self.line_count < self.input.lines_count() {
            self.line_count += 1;
            if self.debug {
                println!("line_count = {}", self.line_count)
            }
            Ok(self.line_count - 1)
        } else {
            Err(())
        }
    }

    pub(crate) fn token_flush(&mut self) {
        self.set_ptok(self.pcur);
    }

    pub(crate) fn set_ptok(&mut self, ptok: usize) {
        if self.debug {
            println!("set_ptok({})", ptok);
        }
        self.ptok = ptok;
    }

    pub(crate) fn parser_cr(&mut self, mut c: u8) -> u8 {
        if self.peek(b'\n') {
            self.pcur += 1;
            c = b'\n';
        }
        c
    }

    pub(crate) fn byte_at(&self, idx: usize) -> MaybeByte {
        match self.input.byte_at(idx) {
            Some(byte) => MaybeByte::Some(byte),
            None => MaybeByte::EndOfInput,
        }
    }

    pub(crate) fn substr_at(&self, start: usize, end: usize) -> Option<&[u8]> {
        self.input.substr_at(start, end)
    }

    pub(crate) fn was_bol(&self) -> bool {
        self.pcur == self.pbeg + 1
    }

    pub(crate) fn is_word_match(&self, word: &str) -> bool {
        let len = word.len();

        if self.substr_at(self.pcur, self.pcur + len) != Some(word.as_bytes()) {
            return false;
        }
        if self.pcur + len == self.pend {
            return true;
        }
        let c = self.byte_at(self.pcur + len);
        if c.is_space() {
            return true;
        }
        if c == b'\0' || c == Self::CTRL_Z_CHAR || c == Self::CTRL_D_CHAR {
            return true;
        }
        false
    }

    pub(crate) fn is_looking_at_eol(&self) -> bool {
        let mut ptr = self.pcur;
        while ptr < self.pend {
            let c = self.input.byte_at(ptr);
            ptr += 1;
            if let Some(c) = c {
                let eol = c == b'\n' || c == b'#';
                if eol || !c.is_ascii_whitespace() {
                    return eol;
                }
            };
        }
        true
    }

    pub(crate) fn is_whole_match(&self, eos: &[u8], indent: usize) -> bool {
        let mut ptr = self.pbeg;
        let len = eos.len();

        if indent > 0 {
            while let Some(c) = self.input.byte_at(ptr) {
                if !c.is_ascii_whitespace() {
                    break;
                }
                ptr += 1;
            }
        }

        if self.pend < ptr + len {
            return false;
        }

        if let Ok(n) = isize::try_from(self.pend - (ptr + len)) {
            if n < 0 {
                return false;
            }
            let last_char = self.byte_at(ptr + len);
            let char_after_last_char = self.byte_at(ptr + len + 1);

            if n > 0 && last_char != b'\n' {
                if last_char != b'\r' {
                    return false;
                }
                if n <= 1 || char_after_last_char != b'\n' {
                    return false;
                }
            }

            let next_len_chars = self.substr_at(ptr, ptr + len);
            Some(eos) == next_len_chars
        } else {
            false
        }
    }

    pub(crate) fn eof_no_decrement(&mut self) {
        if let Some(prevline) = self.prevline {
            if !self.eofp {
                self.lastline = prevline;
            }
        }
        self.pbeg = self.input.line_at(self.lastline).start;
        self.pend = self.pbeg + self.input.line_at(self.lastline).len();
        self.pcur = self.pend;
        self.pushback(&MaybeByte::new(1));
        self.set_ptok(self.pcur);
    }

    pub(crate) fn is_identchar(&self, begin: usize, _end: usize) -> bool {
        let byte = match self.input.byte_at(begin) {
            Some(byte) => byte,
            None => return false,
        };

        byte.is_ascii_alphanumeric() || byte == b'_' || !byte.is_ascii()
    }

    pub(crate) fn set_encoding(&mut self, encoding: &str) -> Result<(), InputError> {
        self.input.set_encoding(encoding)
    }
}

pub(crate) trait Pushback<T> {
    fn pushback(&mut self, c: &T);
}

impl Pushback<Option<u8>> for Buffer {
    fn pushback(&mut self, c: &Option<u8>) {
        if c.is_none() {
            return;
        };
        self.pcur -= 1;
        if self.pcur > self.pbeg
            && self.byte_at(self.pcur) == b'\n'
            && self.byte_at(self.pcur - 1) == b'\r'
        {
            self.pcur -= 1;
        }
        if self.debug {
            println!("pushback({:?}) pcur = {}", c, self.pcur);
        }
    }
}

impl Pushback<MaybeByte> for Buffer {
    fn pushback(&mut self, c: &MaybeByte) {
        self.pushback(&c.to_option())
    }
}

impl Pushback<char> for Buffer {
    fn pushback(&mut self, _c: &char) {
        self.pushback(&Some(1))
    }
}
