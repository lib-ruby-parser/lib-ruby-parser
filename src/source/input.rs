use lib_ruby_parser_ast::Blob;

use crate::source::Decoder;
use crate::source::SourceLine;
use crate::source::{decode_input, DecodedInput, InputError};

/// Representation of the source code.
#[derive(Debug)]
#[repr(C)]
pub struct Input<'b> {
    pub(crate) decoded: DecodedInput<'b>,
    decoder: Option<Decoder<'b>>,
    blob: &'b Blob<'b>,
}

impl<'b> Input<'b> {
    /// Constructs a new input
    pub fn new(
        name: &'b str,
        bytes: &'b [u8],
        decoder: Option<Decoder<'b>>,
        blob: &'b Blob<'b>,
    ) -> Self {
        Self {
            decoded: DecodedInput::new(name, bytes, blob),
            decoder,
            blob,
        }
    }

    pub(crate) fn set_bytes(&mut self, bytes: &'b [u8]) {
        self.decoded.set_bytes(bytes)
    }

    pub(crate) fn byte_at(&self, idx: u32) -> Option<u8> {
        self.decoded.bytes.get(idx as usize).copied()
    }

    pub(crate) fn unchecked_byte_at(&self, idx: u32) -> u8 {
        self.decoded.bytes[idx as usize]
    }

    pub(crate) fn substr_at(&self, start: u32, end: u32) -> Option<&'b [u8]> {
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

    pub(crate) fn line_at(&self, idx: u32) -> &SourceLine {
        self.decoded.line_at(idx)
    }

    pub(crate) fn lines_count(&self) -> u32 {
        self.decoded.lines.len() as u32
    }

    pub(crate) fn set_encoding(&mut self, encoding: &'b str) -> Result<(), InputError<'b>> {
        let new_input = decode_input(self.decoded.bytes, encoding, &mut self.decoder, self.blob)
            .into_result()?;
        self.set_bytes(new_input);
        Ok(())
    }

    /// Returns raw bytes after decoding
    pub fn as_shared_bytes(&self) -> &'b [u8] {
        self.decoded.bytes
    }
}
