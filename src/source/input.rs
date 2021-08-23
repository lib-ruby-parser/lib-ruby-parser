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

#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalStringPtr;
#[cfg(feature = "compile-with-external-structures")]
type StringPtr = ExternalStringPtr;
#[cfg(not(feature = "compile-with-external-structures"))]
type StringPtr = String;

use crate::source::SourceLine;
use crate::source::{decode_input, DecodedInput, Decoder, InputError};

/// Representation of the source code.
#[derive(Debug, Default)]
#[repr(C)]
pub struct Input {
    pub(crate) decoded: DecodedInput,
    decoder: Decoder,
}

impl Input {
    /// Constructs a new input
    pub fn new<Name>(name: Name, decoder: Decoder) -> Self
    where
        Name: Into<StringPtr>,
    {
        Self {
            decoded: DecodedInput {
                name: name.into(),
                ..Default::default()
            },
            decoder,
        }
    }

    /// Populates `Input` with a given byte array
    pub fn set_bytes(&mut self, bytes: List<u8>) {
        self.decoded.set_bytes(bytes)
    }

    pub(crate) fn byte_at(&self, idx: usize) -> Option<u8> {
        if let Some(c) = self.decoded.bytes.get(idx) {
            Some(*c)
        } else {
            None
        }
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
        &self.decoded.line_at(idx)
    }

    pub(crate) fn lines_count(&self) -> usize {
        self.decoded.lines.len()
    }

    pub(crate) fn set_encoding(&mut self, encoding: &str) -> Result<(), InputError> {
        let new_input = decode_input(
            std::mem::take(&mut self.decoded.bytes),
            StringPtr::from(encoding),
            self.decoder.take(),
        )
        .to_result()?;
        self.set_bytes(new_input);
        Ok(())
    }

    /// Returns raw bytes after decoding
    pub fn as_shared_bytes(&self) -> SharedByteList {
        self.decoded.as_shared_bytes()
    }

    /// Converts itself into owned vector of bytes
    pub fn into_bytes(self) -> List<u8> {
        self.decoded.into_bytes()
    }
}
