use lib_ruby_parser_ast_arena::Blob;

use crate::source::SourceLine;

/// Decoded input
#[derive(Debug)]
#[repr(C)]
pub struct DecodedInput<'b> {
    /// Name of the input
    pub name: &'b str,

    /// Lines list
    pub lines: Vec<SourceLine>,

    /// Decoded bytes
    pub bytes: &'b [u8],

    pub(crate) blob: &'b Blob<'b>,
}

impl<'b> DecodedInput<'b> {
    /// Constructs empty DecodedInput with given name
    pub fn new(name: &'b str, bytes: &'b [u8], blob: &'b Blob<'b>) -> Self {
        let mut this = Self {
            name,
            lines: vec![],
            bytes: b"",
            blob,
        };
        this.set_bytes(bytes);
        this
    }

    /// Populates `Input` with a given byte array
    pub(crate) fn set_bytes(&mut self, bytes: &'b [u8]) {
        let mut line = SourceLine {
            start: 0,
            end: 0,
            ends_with_eof: true,
        };
        let mut lines = vec![];

        for (idx, c) in bytes.iter().enumerate() {
            line.end = idx + 1;
            if *c == b'\n' {
                line.ends_with_eof = false;
                lines.push(line);
                line = SourceLine {
                    start: idx + 1,
                    end: 0,
                    ends_with_eof: true,
                }
            }
        }
        line.end = bytes.len();
        line.ends_with_eof = true;
        lines.push(line);

        self.bytes = bytes;
        self.lines = lines;
    }

    /// Returns (line, col) pair for a given byte offset.
    ///
    /// Returns None if given offset is out of range.
    pub fn line_col_for_pos(&self, mut pos: usize) -> Option<(usize, usize)> {
        if pos == self.len() {
            // EOF loc
            let last_line = self.lines.last()?;
            return Some((self.lines.len() - 1, last_line.len()));
        }

        for (lineno, line) in self.lines.iter().enumerate() {
            if line.len() > pos {
                return Some((lineno, pos));
            } else {
                pos -= line.len()
            }
        }

        None
    }

    pub(crate) fn line_at(&self, idx: usize) -> &SourceLine {
        &self.lines[idx]
    }

    pub(crate) fn substr_at(&self, start: usize, end: usize) -> Option<&'b [u8]> {
        if start <= end && end <= self.bytes.len() {
            Some(&self.bytes[start..end])
        } else {
            None
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.bytes.len()
    }
}
