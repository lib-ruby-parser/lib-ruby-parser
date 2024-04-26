use core::convert::TryFrom;

use lib_ruby_parser_ast::Blob;
use lib_ruby_parser_ast::SingleLinkedIntrusiveListItem;

use crate::maybe_byte::*;
use crate::source::input::Input;
use crate::source::Decoder;
use crate::source::InputError;
use crate::source::SourceLine;

#[derive(Debug)]
pub(crate) struct Buffer<'b> {
    pub(crate) input: Input<'b>,

    pub(crate) line_count: u32,
    pub(crate) prevline: Option<&'b SourceLine>, // index
    pub(crate) lastline: Option<&'b SourceLine>, // index
    pub(crate) nextline: Option<&'b SourceLine>, // index
    pub(crate) workline: Option<&'b SourceLine>, // index
    pub(crate) pbeg: u32,
    pub(crate) pcur: u32,
    pub(crate) pend: u32,
    pub(crate) ptok: u32,

    pub(crate) eofp: bool,
    pub(crate) cr_seen: bool,

    pub(crate) heredoc_end: u32,
    pub(crate) heredoc_indent: i32,
    pub(crate) heredoc_line_indent: i32,

    pub(crate) tokidx: u32,
    // pub(crate) toksize: u32,
    pub(crate) tokline: u32,

    pub(crate) has_shebang: bool,

    /* current line no. */
    pub(crate) ruby_sourceline: u32,
    // pub(crate) ruby_sourcefile: Vec<char>, /* current source file */
    // pub(crate) ruby_sourcefile_string: Vec<char>,
}

#[cfg(feature = "debug-buffer")]
macro_rules! println_if_debug_buffer {
    ($fmt_string:expr, $( $arg:expr ),*) => {
        eprintln!($fmt_string, $( $arg ),*);
    };
}
#[cfg(not(feature = "debug-buffer"))]
macro_rules! println_if_debug_buffer {
    ($fmt_string:expr, $( $arg:expr ),*) => {};
}

impl<'b> Buffer<'b> {
    const CTRL_Z_CHAR: u8 = 0x1a;
    const CTRL_D_CHAR: u8 = 0x04;

    pub(crate) fn new(
        name: &'b str,
        bytes: &'b [u8],
        decoder: Option<Decoder<'b>>,
        blob: &'b Blob<'b>,
    ) -> Self {
        let input = Input::new(name, bytes, decoder, blob);

        let mut this = Self {
            input,
            line_count: 0,
            prevline: None,
            lastline: None,
            nextline: None,
            workline: None,
            pbeg: 0,
            pcur: 0,
            pend: 0,
            ptok: 0,
            eofp: false,
            cr_seen: false,
            heredoc_end: 0,
            heredoc_indent: 0,
            heredoc_line_indent: 0,
            tokidx: 0,
            tokline: 0,
            has_shebang: false,
            ruby_sourceline: 0,
        };

        this.prepare();

        this
    }

    fn prepare(&mut self) {
        let c = self.nextc();
        match c.as_option() {
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

        self.pushback(c)
    }

    pub(crate) fn nextc(&mut self) -> MaybeByte {
        if self.pcur == self.pend || self.eofp || self.nextline.is_some() {
            let n = self.nextline();
            println_if_debug_buffer!("nextline = {:?}", n);
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
        println_if_debug_buffer!("nextc = {:?}", c);
        MaybeByte::new(c)
    }

    pub(crate) fn goto_eol(&mut self) {
        self.pcur = self.pend;
    }

    pub(crate) fn is_eol(&self) -> bool {
        self.pcur >= self.pend
    }

    pub(crate) fn is_eol_n(&self, n: u32) -> bool {
        self.pcur + n >= self.pend
    }

    pub(crate) fn peek(&self, c: u8) -> bool {
        self.peek_n(c, 0)
    }
    pub(crate) fn peek_n(&self, c: u8, n: u32) -> bool {
        !self.is_eol_n(n) && c == self.input.unchecked_byte_at(self.pcur + n)
    }
    pub(crate) fn peekc(&self) -> MaybeByte {
        self.peekc_n(0)
    }
    pub(crate) fn peekc_n(&self, n: u32) -> MaybeByte {
        if self.is_eol_n(n) {
            MaybeByte::EndOfInput
        } else {
            self.byte_at(self.pcur + n)
        }
    }

    pub(crate) fn nextline(&mut self) -> Result<(), ()> {
        let mut v = self.nextline;
        self.nextline = None;

        if v.is_none() {
            if self.eofp {
                return Err(());
            }

            if self.pend > self.pbeg && self.input.unchecked_byte_at(self.pend - 1) != b'\n' {
                self.eofp = true;
                self.goto_eol();
                return Err(());
            }

            match self.getline() {
                Ok(line) => v = Some(line),
                Err(_) => {
                    self.eofp = true;
                    self.goto_eol();
                    return Err(());
                }
            }

            self.cr_seen = false;
        }
        // TODO: after here-document without terminator

        let line = v.unwrap();

        if self.heredoc_end > 0 {
            self.ruby_sourceline = self.heredoc_end;
            self.heredoc_end = 0;
        }
        self.ruby_sourceline += 1;
        self.workline = Some(line);
        self.pbeg = line.start as u32;
        self.pcur = line.start as u32;
        self.pend = line.end as u32;
        self.token_flush();
        self.prevline = self.lastline;
        self.lastline = v;

        Ok(())
    }

    pub(crate) fn getline(&mut self) -> Result<&'b SourceLine, ()> {
        if self.line_count < self.input.lines_count() {
            self.line_count += 1;
            println_if_debug_buffer!("line_count = {}", self.line_count);
            if let Some(workline) = self.workline {
                if let Some(next) = workline.next() {
                    return Ok(next.as_ref());
                }
            }
            if self.line_count == 1 {
                Ok(self.input.decoded.lines.first().unwrap())
            } else {
                unreachable!("no lines")
            }
        } else {
            Err(())
        }
    }

    pub(crate) fn token_flush(&mut self) {
        self.set_ptok(self.pcur);
    }

    pub(crate) fn set_ptok(&mut self, ptok: u32) {
        println_if_debug_buffer!("set_ptok({})", ptok);
        self.ptok = ptok;
    }

    pub(crate) fn parser_cr(&mut self, mut c: u8) -> u8 {
        if self.peek(b'\n') {
            self.pcur += 1;
            c = b'\n';
        }
        c
    }

    pub(crate) fn byte_at(&self, idx: u32) -> MaybeByte {
        match self.input.byte_at(idx) {
            Some(byte) => MaybeByte::Some(byte),
            None => MaybeByte::EndOfInput,
        }
    }

    pub(crate) fn substr_at(&self, start: u32, end: u32) -> Option<&'b [u8]> {
        self.input.substr_at(start, end)
    }

    pub(crate) fn was_bol(&self) -> bool {
        self.pcur == self.pbeg + 1
    }

    pub(crate) fn is_word_match(&self, word: &str) -> bool {
        let len = word.len() as u32;

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

    pub(crate) fn is_whole_match(&self, eos: &[u8], indent: u32) -> bool {
        let mut ptr = self.pbeg;
        let len = eos.len() as u32;

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
                self.lastline = Some(prevline);
            }
        }
        self.pbeg = self.lastline.unwrap().start as u32;
        self.pend = self.pbeg + self.lastline.unwrap().len() as u32;
        self.pcur = self.pend;
        self.pushback(1);
        self.set_ptok(self.pcur);
    }

    pub(crate) fn is_identchar(&self, begin: u32, _end: u32) -> bool {
        let byte = match self.input.byte_at(begin) {
            Some(byte) => byte,
            None => return false,
        };

        byte.is_ascii_alphanumeric() || byte == b'_' || !byte.is_ascii()
    }

    pub(crate) fn set_encoding(&mut self, encoding: &'b str) -> Result<(), InputError<'b>> {
        self.input.set_encoding(encoding)
    }
}

pub(crate) trait Pushback<T> {
    fn pushback(&mut self, c: T);
}

impl Pushback<u8> for Buffer<'_> {
    fn pushback(&mut self, _c: u8) {
        self.pcur -= 1;
        if self.pcur > self.pbeg
            && self.byte_at(self.pcur) == b'\n'
            && self.byte_at(self.pcur - 1) == b'\r'
        {
            self.pcur -= 1;
        }
        println_if_debug_buffer!("pushback({:?}) pcur = {}", _c, self.pcur);
    }
}

impl Pushback<Option<u8>> for Buffer<'_> {
    fn pushback(&mut self, c: Option<u8>) {
        if let Some(c) = c {
            self.pushback(c)
        }
    }
}

impl Pushback<MaybeByte> for Buffer<'_> {
    fn pushback(&mut self, c: MaybeByte) {
        self.pushback(c.as_option())
    }
}

impl Pushback<&mut MaybeByte> for Buffer<'_> {
    fn pushback(&mut self, c: &mut MaybeByte) {
        self.pushback(c.as_option())
    }
}
