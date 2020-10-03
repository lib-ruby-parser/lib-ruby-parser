use std::convert::{TryFrom};
use crate::lexer::LexChar;

#[derive(Debug, Clone)]
#[allow(dead_code, non_camel_case_types)]
pub enum BufferEncoding {
    ASCII,
    BIG5_2003,
    ERROR,
    EUC_JP,
    GB18030,
    GBK,
    HZ,
    IBM866,
    ISO_2022_JP,
    ISO_8859_1,
    ISO_8859_2,
    ISO_8859_3,
    ISO_8859_4,
    ISO_8859_5,
    ISO_8859_6,
    ISO_8859_7,
    ISO_8859_8,
    ISO_8859_10,
    ISO_8859_13,
    ISO_8859_14,
    ISO_8859_15,
    ISO_8859_16,
    KOI8_R,
    KOI8_U,
    MAC_CYRILLIC,
    MAC_ROMAN,
    UTF_8,
    UTF_16BE,
    UTF_16LE,
    WINDOWS_874,
    WINDOWS_949,
    WINDOWS_1250,
    WINDOWS_1251,
    WINDOWS_1252,
    WINDOWS_1253,
    WINDOWS_1254,
    WINDOWS_1255,
    WINDOWS_1256,
    WINDOWS_1257,
    WINDOWS_1258,
    WINDOWS_31J,

    Unknown,
}

impl Default for BufferEncoding {
    fn default() -> Self {
        Self::Unknown
    }
}


#[derive(Debug, Clone, Default)]
pub struct SourceLine {
    pub start: usize,
    pub end: usize,
}

impl SourceLine {
    pub fn source(&self, source: &Vec<u8>) -> Vec<u8> {
        source[self.start..self.end].to_owned()
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

#[derive(Debug, Clone, Default)]
pub struct Buffer {
    pub name: String,
    pub encoding: BufferEncoding,
    pub input: Vec<u8>,

    pub lines: Vec<SourceLine>,
    pub line_count: usize,
    pub prevline: usize, // index
    pub lastline: usize, // index
    pub nextline: usize, // index
    pub pbeg: usize,
    pub pcur: usize,
    pub pend: usize,
    pub ptok: usize,

    pub eofp: bool,
    pub cr_seen: bool,

    pub heredoc_end: usize,
    pub heredoc_indent: i32,
    pub heredoc_line_indent: i32,

    pub tokidx: usize,
    pub toksize: usize,
    pub tokline: usize,

    pub ruby_sourceline: usize,        /* current line no. */
    pub ruby_sourcefile: Vec<u8>,      /* current source file */
    pub ruby_sourcefile_string: Vec<u8>,

    pub debug: bool,
}

impl Buffer {
    const CTRL_Z_CHAR: u8 = 0x1a;
    const CTRL_D_CHAR: u8 = 0x04;

    pub fn new(name: &str, bytes: Vec<u8>, encoding: BufferEncoding) -> Self {
        let mut line = SourceLine { start: 0, end: 0 };
        let mut lines: Vec<SourceLine> = vec![];

        for (idx, c) in bytes.iter().enumerate() {
            line.end = idx + 1;
            if *c == b'\n' {
                lines.push(line);
                line = SourceLine { start: idx + 1, end: 0 }
            }
        };
        line.end = bytes.len();
        if !line.is_empty() {
            lines.push(line);
        }

        Self { name: name.to_owned(), input: bytes, lines, encoding, ..Self::default() }
    }

    pub fn nextc(&mut self) -> LexChar {
        if self.pcur == self.pend || self.eofp || self.nextline != 0 {
            let n = self.nextline();
            if self.debug { println!("nextline = {:?}", n); }
            if n.is_err() {
                return LexChar::EOF;
            }
        }
        let mut c: u8 = self.input[self.pcur];
        self.pcur += 1;
        if c == b'\r' {
            c = self.parser_cr(c);
        }
        if self.debug { println!("nextc = {:?}", c as char); }
        return LexChar::Some(c);
    }

    pub fn goto_eol(&mut self) {
        self.pcur = self.pend;
    }

    pub fn is_eol(&self) -> bool {
        self.pcur >= self.pend
    }

    pub fn is_eol_n(&self, n: usize) -> bool {
        self.pcur + n >= self.pend
    }

    pub fn peek(&self, c: u8) -> bool {
        self.peek_n(c, 0)
    }
    pub fn peek_n(&self, c: u8, n: usize) -> bool {
        !self.is_eol_n(n) && c == self.input[self.pcur + n]
    }

    pub fn nextline(&mut self) -> Result<(), ()> {
        let mut v = self.nextline;
        self.nextline = 0;

        if v == 0 {
            if self.eofp {
                return Err(());
            }

            if self.pend > self.pbeg && self.input[self.pend - 1] != b'\n' {
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

        let line = &self.lines[v];

        if self.heredoc_end > 0 {
            self.ruby_sourceline = self.heredoc_end;
            self.heredoc_end = 0;
        }
        self.ruby_sourceline += 1;
        self.pbeg = line.start;
        self.pcur = line.start;
        self.pend = line.end;
        self.token_flush();
        self.prevline = self.lastline;
        self.lastline = v;


        Ok(())
    }

    pub fn getline(&mut self) -> Result<usize, ()> {
        if self.line_count < self.lines.len() {
            self.line_count += 1;
            if self.debug { println!("line_count = {}", self.line_count) }
            Ok(self.line_count - 1)
        } else {
            Err(())
        }
    }

    pub fn token_flush(&mut self) {
        self.set_ptok(self.pcur);
    }

    pub fn set_ptok(&mut self, ptok: usize) {
        if self.debug { println!("set_ptok({})", ptok); }
        self.ptok = ptok;
    }

    pub fn parser_cr(&mut self, _c: u8) -> u8 {
        unimplemented!("parser_cr")
    }


    pub fn char_at(&self, idx: usize) -> LexChar {
        if let Some(c) = self.input.get(idx) {
            LexChar::Some(c.clone())
        } else {
            LexChar::EOF
        }
    }

    pub fn substr_at(&self, start: usize, end: usize) -> Option<Vec<u8>> {
        if start < end && end < self.input.len() {
            Some(self.input[start..end].to_owned())
        } else {
            None
        }
    }

    pub fn pushback(&mut self, c: &LexChar) {
        if c.is_eof() { return };
        self.pcur -= 1;
        if self.pcur > self.pbeg && self.input[self.pcur] == b'\n' && self.input[self.pcur - 1] == b'\r' {
            self.pcur -= 1;
        }
        if self.debug { println!("pushback({:?}) pcur = {}", c, self.pcur); }
    }

    pub fn was_bol(&self) -> bool {
        self.pcur == self.pbeg + 1
    }

    pub fn is_word_match(&self, word: &Vec<u8>) -> bool {
        let len = word.len();

        if self.substr_at(self.pcur, self.pcur + len).as_ref() != Some(word) { return false }
        if self.pcur + len == self.pend { return true }
        let c = self.char_at(self.pcur + len);
        if c.is_space() { return true }
        if c == b'\0' || c == Self::CTRL_Z_CHAR || c == Self::CTRL_D_CHAR {
            return true;
        }
        false
    }

    pub fn is_looking_at_eol(&self) -> bool {
        let mut ptr = self.pcur;
        while ptr < self.pend {
            let c = self.input.get(ptr);
            ptr += 1;
            if let Some(c) = c {
                let eol = *c == b'\n' || *c == b'#';
                if eol || !c.is_ascii_whitespace() {
                    return eol
                }
            };
        };
        true
    }

    pub fn is_whole_match(&self, eos: &Vec<u8>, indent: usize) -> bool {
        let mut ptr = self.pbeg;
        let len = eos.len();

        if indent > 0 {
            while let Some(c) = self.input.get(ptr) {
                if !c.is_ascii_whitespace() { break }
                ptr += 1;
            }
        }

        if self.pend < ptr + len { return false }

        if let Ok(n) = isize::try_from(self.pend - (ptr + len)) {
            if n < 0 { return false }
            let last_char = self.input.get(ptr + len);
            let char_after_last_char = self.input.get(ptr + len + 1);

            if n > 0 && last_char != Some(&b'\n') {
                if last_char != Some(&b'\r') { return false }
                if n <= 1 || char_after_last_char != Some(&b'\n') { return false }
            }

            let next_len_chars: Vec<u8> = self.input[ptr..ptr+len].to_owned();
            return *eos == next_len_chars
        } else {
            return false
        }
    }

    pub fn eof_no_decrement(&mut self) {
        if self.prevline != 0 && !self.eofp {
            self.lastline = self.prevline;
        }

        self.pbeg = self.lines[self.lastline].start;
        self.pend = self.pbeg + self.lines[self.lastline].len();
        self.pcur = self.pend;
        self.pushback(&LexChar::Some(1 as u8));
        self.set_ptok(self.pcur);
    }

    pub fn line_col_for_pos(&self, mut pos: usize) -> Option<(usize, usize)> {
        for (lineno, line) in self.lines.iter().enumerate() {
            if pos < line.len() {
                pos -= line.len()
            } else {
                return Some(( lineno, pos ))
            }
        }

        None
    }
}
