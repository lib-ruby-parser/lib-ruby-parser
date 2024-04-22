use lib_ruby_parser_ast_arena::{Blob, Bytes};

#[derive(Debug)]
pub(crate) struct TokenBuf<'b> {
    pub(crate) bytes: &'b mut Bytes<'b>,
    blob: &'b Blob<'b>,
}

impl<'b> TokenBuf<'b> {
    pub(crate) fn empty(blob: &'b Blob<'b>) -> Self {
        Self {
            bytes: blob.alloc_ref(),
            blob,
        }
    }

    pub(crate) fn take(&mut self) -> Self {
        let mut result = Self::empty(self.blob);
        core::mem::swap(&mut result, self);
        result
    }

    pub(crate) fn append_valid_escaped(&mut self, c: char) {
        self.bytes.append_valid_escaped(c, self.blob);
    }

    pub(crate) fn append_invalid_escaped(&mut self, b: u8) {
        self.bytes.append_invalid_escaped(b, self.blob);
    }

    pub(crate) fn append_borrowed(&mut self, bytes: &'b [u8]) {
        if bytes.is_empty() {
            // FIXME: just strip out callers that make redundant pushes
            return;
        }
        let s = core::str::from_utf8(bytes).unwrap();
        self.bytes.append_borrowed(s, self.blob);
    }

    pub(crate) fn prepend_valid_escaped(&mut self, c: char) {
        self.bytes.prepend_valid_escaped(c, self.blob);
    }

    pub(crate) fn prepend_invalid_escaped(&mut self, b: u8) {
        self.bytes.prepend_invalid_escaped(b, self.blob);
    }

    pub(crate) fn prepend_borrowed(&mut self, bytes: &'b [u8]) {
        let s = core::str::from_utf8(bytes).unwrap();
        self.bytes.prepend_borrowed(s, self.blob);
    }

    pub(crate) fn as_string(&self) -> Option<String> {
        self.bytes.try_to_string().ok()
    }

    pub(crate) fn len(&self) -> usize {
        self.bytes.len()
    }

    pub(crate) fn clear(&mut self) {
        self.bytes = self.blob.alloc_ref();
    }
}
