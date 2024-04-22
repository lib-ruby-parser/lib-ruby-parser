use lib_ruby_parser_ast_arena::{Blob, SingleLinkedIntrusiveList};

use crate::source::SourceLine;

/// Decoded input
#[derive(Debug)]
#[repr(C)]
pub struct DecodedInput<'b> {
    /// Name of the input
    pub name: &'b str,

    /// Lines list
    pub lines: &'b SingleLinkedIntrusiveList<'b, SourceLine>,

    /// Decoded bytes
    pub bytes: &'b [u8],

    pub(crate) blob: &'b Blob<'b>,
}

impl<'b> DecodedInput<'b> {
    /// Constructs empty DecodedInput with given name
    pub fn new(name: &'b str, bytes: &'b [u8], blob: &'b Blob<'b>) -> Self {
        let mut this = Self {
            name,
            lines: blob.alloc_ref(),
            bytes: b"",
            blob,
        };
        this.set_bytes(bytes);
        this
    }

    /// Populates `Input` with a given byte array
    pub(crate) fn set_bytes(&mut self, bytes: &'b [u8]) {
        let mut line = SourceLine::new(0, 0, true, self.blob);
        let lines: &'b SingleLinkedIntrusiveList<'b, SourceLine> = self.blob.alloc_ref();

        for (idx, c) in bytes.iter().enumerate() {
            line.end = idx + 1;
            if *c == b'\n' {
                line.ends_with_eof = false;
                lines.push(line);
                line = SourceLine::new(idx + 1, 0, true, self.blob)
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

    pub(crate) fn line_at(&self, idx: u32) -> &SourceLine {
        self.lines.item_at(idx as usize).unwrap()
    }

    pub(crate) fn substr_at(&self, start: u32, end: u32) -> Option<&'b [u8]> {
        if start <= end && end as usize <= self.bytes.len() {
            Some(&self.bytes[start as usize..end as usize])
        } else {
            None
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.bytes.len()
    }
}
