use crate::containers::{list::AsSharedByteList, StringPtr};

#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalSharedByteList;
#[cfg(feature = "compile-with-external-structures")]
type SharedByteList = ExternalSharedByteList;
#[cfg(not(feature = "compile-with-external-structures"))]
type SharedByteList<'a> = &'a [u8];

#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalList;
#[cfg(feature = "compile-with-external-structures")]
type List<T> = ExternalList<T>;
#[cfg(not(feature = "compile-with-external-structures"))]
type List<T> = Vec<T>;

use crate::source::SourceLine;

/// Decoded input
#[derive(Debug, Default)]
#[repr(C)]
pub struct DecodedInput {
    /// Name of the input
    pub name: StringPtr,

    /// Lines list
    pub lines: List<SourceLine>,

    /// Decoded bytes
    pub bytes: List<u8>,
}

impl DecodedInput {
    /// Constructs empty DecodedInput with given name
    pub fn new<Name>(name: Name) -> Self
    where
        Name: Into<StringPtr>,
    {
        Self {
            name: name.into(),
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
    pub fn as_shared_bytes(&self) -> SharedByteList {
        self.bytes.shared()
    }

    /// Converts itself into owned vector of bytes
    pub fn into_bytes(self) -> List<u8> {
        self.bytes
    }
}
