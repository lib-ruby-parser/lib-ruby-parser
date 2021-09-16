crate::use_native_or_external!(List);
crate::use_native_or_external!(SharedByteList);

use super::DecodedInput;
use crate::source::SourceLine;

impl DecodedInput {
    /// Populates `Input` with a given byte array
    pub fn update_bytes(&mut self, bytes: List<u8>) {
        let mut line = SourceLine::new(0, 0, true);
        let mut lines = list![];

        for (idx, c) in bytes.iter().enumerate() {
            line.set_end(idx + 1);
            if *c == b'\n' {
                line.set_ends_with_eof(false);
                lines.push(line);
                line = SourceLine::new(idx + 1, 0, true)
            }
        }
        line.set_end(bytes.len());
        line.set_ends_with_eof(true);
        lines.push(line);

        self.set_bytes(bytes);
        self.set_lines(lines);
    }

    /// Returns (line, col) pair for a given byte offset.
    ///
    /// Returns None if given offset is out of range.
    pub fn line_col_for_pos(&self, mut pos: usize) -> Option<(usize, usize)> {
        if pos == self.len() {
            // EOF loc
            let last_line = self.lines().last()?;
            return Some((self.lines().len() - 1, last_line.len()));
        }

        for (lineno, line) in self.lines().iter().enumerate() {
            if line.len() > pos {
                return Some((lineno, pos));
            } else {
                pos -= line.len()
            }
        }

        None
    }

    pub(crate) fn line_at(&self, idx: usize) -> &SourceLine {
        &self.lines()[idx]
    }

    pub(crate) fn substr_at(&self, start: usize, end: usize) -> Option<&[u8]> {
        if start <= end && end <= self.bytes().len() {
            Some(&self.bytes()[start..end])
        } else {
            None
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.bytes().len()
    }

    /// Returns raw bytes after decoding
    pub fn as_shared_bytes(&self) -> SharedByteList {
        self.bytes().as_slice()
    }
}
