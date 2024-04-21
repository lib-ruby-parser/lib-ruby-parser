use crate::source::SourceLine;

/// Decoded input
#[derive(Debug, Default)]
#[repr(C)]
pub struct DecodedInput {
    /// Name of the input
    pub name: String,

    /// Lines list
    pub lines: Vec<SourceLine>,

    /// Decoded bytes
    pub bytes: Vec<u8>,
}

impl DecodedInput {
    /// Constructs empty DecodedInput with given name
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Converts itself into owned vector of bytes
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub(crate) fn take_bytes(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.bytes)
    }

    /// Populates `Input` with a given byte array
    pub fn update_bytes(&mut self, bytes: Vec<u8>) {
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

    pub(crate) fn substr_at(&self, start: usize, end: usize) -> Option<&[u8]> {
        if start <= end && end <= self.bytes.len() {
            Some(&self.bytes[start..end])
        } else {
            None
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns raw bytes after decoding
    pub fn as_shared_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}
