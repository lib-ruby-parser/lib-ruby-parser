use crate::containers::{List, MaybePtr, String};
use crate::source::SourceLine;
use crate::source::{decode_input, CustomDecoder, InputError};

/// Representation of the source code.
#[derive(Debug, Default)]
pub struct Input {
    pub(crate) name: String,
    bytes: List<u8>,
    lines: List<SourceLine>,
    decoder: MaybePtr<CustomDecoder>,
}

impl Input {
    /// Constructs a new input
    pub fn new<Name, Decoder>(name: Name, decoder: Decoder) -> Self
    where
        Name: Into<String>,
        Decoder: Into<MaybePtr<CustomDecoder>>,
    {
        Self {
            name: name.into(),
            decoder: decoder.into(),
            ..Default::default()
        }
    }

    /// Populates `Input` with a given byte array
    pub fn set_bytes(&mut self, bytes: List<u8>) {
        let mut line = SourceLine {
            start: 0,
            end: 0,
            ends_with_eof: true,
        };
        let mut lines = List::<SourceLine>::new();

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

    pub(crate) fn byte_at(&self, idx: usize) -> Option<u8> {
        if let Some(c) = self.bytes.get(idx) {
            Some(*c)
        } else {
            None
        }
    }

    pub(crate) fn unchecked_byte_at(&self, idx: usize) -> u8 {
        self.bytes[idx]
    }

    pub(crate) fn substr_at(&self, start: usize, end: usize) -> Option<&[u8]> {
        if start <= end && end <= self.bytes.len() {
            Some(&self.bytes[start..end])
        } else {
            None
        }
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

    pub(crate) fn len(&self) -> usize {
        self.bytes.len()
    }

    // pub(crate) fn is_empty(&self) -> bool {
    //     self.bytes.is_empty()
    // }

    pub(crate) fn line_at(&self, idx: usize) -> &SourceLine {
        &self.lines[idx]
    }

    pub(crate) fn lines_count(&self) -> usize {
        self.lines.len()
    }

    pub(crate) fn set_encoding(&mut self, encoding: &str) -> Result<(), InputError> {
        let new_input = decode_input(
            self.bytes.take(),
            String::from(encoding),
            self.decoder.take(),
        )
        .to_result()?;
        self.set_bytes(new_input);
        Ok(())
    }

    /// Returns raw bytes after decoding
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Converts itself into owned vector of bytes
    pub fn into_bytes(self) -> List<u8> {
        self.bytes
    }
}
