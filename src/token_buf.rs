use lib_ruby_parser_ast::{Blob, ByteArray};

#[derive(Debug)]
pub(crate) struct TokenBuf<'b> {
    pub(crate) bytes: &'b ByteArray<'b>,
    blob: &'b Blob<'b>,
}

impl<'b> TokenBuf<'b> {
    pub(crate) fn empty(blob: &'b Blob<'b>) -> Self {
        Self {
            bytes: ByteArray::new(blob),
            blob,
        }
    }

    pub(crate) fn take(&mut self) -> Self {
        let mut result = Self::empty(self.blob);
        core::mem::swap(&mut result, self);
        result
    }

    pub(crate) fn push_char(&mut self, c: char) {
        self.bytes.push_char(c, self.blob);
    }

    pub(crate) fn push_byte(&mut self, b: u8) {
        self.bytes.push_byte(b, self.blob);
    }

    pub(crate) fn push_bytes(&mut self, bytes: &'b [u8]) {
        self.bytes.push_bytes(bytes, self.blob);
    }

    pub(crate) fn prepend_byte(&mut self, b: u8) {
        self.bytes.prepend_bytes(&[b], self.blob);
    }

    pub(crate) fn as_whole_string(&self) -> Option<&'b str> {
        self.bytes.try_as_str()
    }

    pub(crate) fn len(&self) -> usize {
        self.bytes.len()
    }

    pub(crate) fn clear(&mut self) {
        self.bytes = ByteArray::new(self.blob);
    }
}
