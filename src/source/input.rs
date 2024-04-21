use crate::source::Decoder;
use crate::source::SourceLine;
use crate::source::{decode_input, DecodedInput, InputError};

/// Representation of the source code.
#[derive(Debug, Default)]
#[repr(C)]
pub struct Input {
    pub(crate) decoded: DecodedInput,
    decoder: Option<Decoder>,
}

impl Input {
    /// Constructs a new input
    pub fn new(name: impl Into<String>, decoder: Option<Decoder>) -> Self {
        Self {
            decoded: DecodedInput::named(name),
            decoder,
        }
    }

    /// Populates `Input` with a given byte array
    pub fn update_bytes(&mut self, bytes: Vec<u8>) {
        self.decoded.update_bytes(bytes)
    }

    pub(crate) fn byte_at(&self, idx: usize) -> Option<u8> {
        self.decoded.bytes.get(idx).copied()
    }

    pub(crate) fn unchecked_byte_at(&self, idx: usize) -> u8 {
        self.decoded.bytes[idx]
    }

    pub(crate) fn substr_at(&self, start: usize, end: usize) -> Option<&[u8]> {
        self.decoded.substr_at(start, end)
    }

    /// Returns (line, col) pair for a given byte offset.
    ///
    /// Returns None if given offset is out of range.
    pub fn line_col_for_pos(&self, pos: usize) -> Option<(usize, usize)> {
        self.decoded.line_col_for_pos(pos)
    }

    pub(crate) fn len(&self) -> usize {
        self.decoded.len()
    }

    // pub(crate) fn is_empty(&self) -> bool {
    //     self.decoded.bytes.is_empty()
    // }

    pub(crate) fn line_at(&self, idx: usize) -> &SourceLine {
        self.decoded.line_at(idx)
    }

    pub(crate) fn lines_count(&self) -> usize {
        self.decoded.lines.len()
    }

    pub(crate) fn set_encoding(&mut self, encoding: &str) -> Result<(), InputError> {
        let new_input = decode_input(
            self.decoded.take_bytes(),
            String::from(encoding),
            &mut self.decoder,
        )
        .into_result()?;
        self.update_bytes(new_input);
        Ok(())
    }

    /// Returns raw bytes after decoding
    pub fn as_shared_bytes(&self) -> &[u8] {
        self.decoded.as_shared_bytes()
    }

    /// Converts itself into owned vector of bytes
    pub fn into_bytes(self) -> Vec<u8> {
        self.decoded.into_bytes()
    }
}
