use crate::lex_char::*;
use crate::source::SourceLine;
use crate::source::{decode_input, InputError};
use std::convert::TryFrom;

#[derive(Debug, Clone, Default)]
pub struct Buffer {
    pub name: String,
    pub input: Vec<char>,
    pub input_s: String,
    pub encoding: String,

    pub(crate) lines: Vec<SourceLine>,
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

    pub(crate) ruby_sourceline: usize,     /* current line no. */
    pub(crate) ruby_sourcefile: Vec<char>, /* current source file */
    pub(crate) ruby_sourcefile_string: Vec<char>,

    pub(crate) debug: bool,
}

impl Buffer {
    const CTRL_Z_CHAR: char = 0x1a as char;
    const CTRL_D_CHAR: char = 0x04 as char;

    pub fn new(
        name: &str,
        bytes: Vec<u8>,
        known_encoding: Option<String>,
    ) -> Result<Self, InputError> {
        let (input_s, encoding) = decode_input(&bytes, known_encoding)?;
        let input = input_s.chars().collect::<Vec<_>>();

        let mut line = SourceLine { start: 0, end: 0 };
        let mut lines: Vec<SourceLine> = vec![];

        for (idx, c) in input.iter().enumerate() {
            line.end = idx + 1;
            if *c == '\n' {
                lines.push(line);
                line = SourceLine {
                    start: idx + 1,
                    end: 0,
                }
            }
        }
        line.end = input.len();
        if !line.is_empty() {
            lines.push(line);
        }

        Ok(Self {
            name: name.to_owned(),
            encoding,
            input,
            input_s,
            lines,
            ..Self::default()
        })
    }

    pub(crate) fn nextc(&mut self) -> LexChar {
        if self.pcur == self.pend || self.eofp || self.nextline != 0 {
            let n = self.nextline();
            if self.debug {
                println!("nextline = {:?}", n);
            }
            if n.is_err() {
                return LexChar::EOF;
            }
        }
        let mut c = self.input[self.pcur];
        self.pcur += 1;
        if c == '\r' {
            c = self.parser_cr(&mut c);
        }
        if self.debug {
            println!("nextc = {:?}", c);
        }
        return LexChar::new(c);
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

    pub(crate) fn peek(&self, c: char) -> bool {
        self.peek_n(c, 0)
    }
    pub(crate) fn peek_n(&self, c: char, n: usize) -> bool {
        !self.is_eol_n(n) && c == self.input[self.pcur + n]
    }

    pub(crate) fn nextline(&mut self) -> Result<(), ()> {
        let mut v = self.nextline;
        self.nextline = 0;

        if v == 0 {
            if self.eofp {
                return Err(());
            }

            if self.pend > self.pbeg && self.input[self.pend - 1] != '\n' {
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
        self.prevline = Some(self.lastline);
        self.lastline = v;

        Ok(())
    }

    pub(crate) fn getline(&mut self) -> Result<usize, ()> {
        if self.line_count < self.lines.len() {
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

    pub(crate) fn parser_cr(&mut self, c: &mut char) -> char {
        if self.peek('\n') {
            self.pcur += 1;
            *c = '\n';
        }
        *c
    }

    pub(crate) fn char_at(&self, idx: usize) -> LexChar {
        if let Some(c) = self.input.get(idx) {
            LexChar::new(*c)
        } else {
            LexChar::EOF
        }
    }

    pub(crate) fn substr_at(&self, start: usize, end: usize) -> Option<&str> {
        if start <= end && end <= self.input.len() {
            Some(&self.input_s[start..end])
        } else {
            None
        }
    }

    pub(crate) fn was_bol(&self) -> bool {
        self.pcur == self.pbeg + 1
    }

    pub(crate) fn is_word_match(&self, word: &str) -> bool {
        let len = word.len();

        if self.substr_at(self.pcur, self.pcur + len) != Some(word) {
            return false;
        }
        if self.pcur + len == self.pend {
            return true;
        }
        let c = self.char_at(self.pcur + len);
        if c.is_space() {
            return true;
        }
        if c == '\0' || c == Self::CTRL_Z_CHAR || c == Self::CTRL_D_CHAR {
            return true;
        }
        false
    }

    pub(crate) fn is_looking_at_eol(&self) -> bool {
        let mut ptr = self.pcur;
        while ptr < self.pend {
            let c = self.input.get(ptr);
            ptr += 1;
            if let Some(c) = c {
                let eol = *c == '\n' || *c == '#';
                if eol || !c.is_ascii_whitespace() {
                    return eol;
                }
            };
        }
        true
    }

    pub(crate) fn is_whole_match(&self, eos: &str, indent: usize) -> bool {
        let mut ptr = self.pbeg;
        let len = eos.len();

        if indent > 0 {
            while let Some(c) = self.input.get(ptr) {
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
            let last_char = self.input.get(ptr + len);
            let char_after_last_char = self.input.get(ptr + len + 1);

            if n > 0 && last_char != Some(&'\n') {
                if last_char != Some(&'\r') {
                    return false;
                }
                if n <= 1 || char_after_last_char != Some(&'\n') {
                    return false;
                }
            }

            let next_len_chars = self.substr_at(ptr, ptr + len);
            return Some(eos) == next_len_chars;
        } else {
            return false;
        }
    }

    pub(crate) fn eof_no_decrement(&mut self) {
        if let Some(prevline) = self.prevline {
            if !self.eofp {
                self.lastline = prevline;
            }
        }
        self.pbeg = self.lines[self.lastline].start;
        self.pend = self.pbeg + self.lines[self.lastline].len();
        self.pcur = self.pend;
        self.pushback(&LexChar::new(1));
        self.set_ptok(self.pcur);
    }

    pub fn line_col_for_pos(&self, mut pos: usize) -> Option<(usize, usize)> {
        for (lineno, line) in self.lines.iter().enumerate() {
            if pos >= line.len() {
                pos -= line.len()
            } else {
                return Some((lineno + 1, pos));
            }
        }

        None
    }
}

pub trait Pushback<T> {
    fn pushback(&mut self, c: &T);
}

impl Pushback<Option<char>> for Buffer {
    fn pushback(&mut self, c: &Option<char>) {
        if c.is_none() {
            return;
        };
        self.pcur -= 1;
        if self.pcur > self.pbeg
            && self.input[self.pcur] == '\n'
            && self.input[self.pcur - 1] == '\r'
        {
            self.pcur -= 1;
        }
        if self.debug {
            println!("pushback({:?}) pcur = {}", c, self.pcur);
        }
    }
}

impl Pushback<LexChar> for Buffer {
    fn pushback(&mut self, c: &LexChar) {
        self.pushback(&c.to_option())
    }
}

impl Pushback<char> for Buffer {
    fn pushback(&mut self, c: &char) {
        self.pushback(&Some(*c))
    }
}
