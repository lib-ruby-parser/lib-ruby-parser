// use crate::Bytes;
use lib_ruby_parser_ast_arena::{Blob, Bytes};

#[derive(Debug)]
pub(crate) struct TokenBuf<'b, 'i> {
    pub(crate) bytes: &'b mut Bytes<'b, 'i>,
    blob: &'b Blob<'b>,
}

impl<'b, 'i> TokenBuf<'b, 'i> {
    pub(crate) fn empty(blob: &'b Blob<'b>) -> Self {
        Self {
            bytes: unsafe { blob.alloc().as_mut() },
            blob,
        }
    }

    #[allow(mutable_transmutes)]
    pub(crate) fn take(&mut self) -> Self {
        let result = Self {
            bytes: unsafe { core::mem::transmute(&*self.bytes) },
            blob: self.blob,
        };
        self.clear();
        result
    }

    pub(crate) fn append_valid_escaped(&mut self, c: char) {
        self.bytes.append_valid_escaped(c, self.blob);
    }

    pub(crate) fn append_invalid_escaped(&mut self, b: u8) {
        self.bytes.append_invalid_escaped(b, self.blob);
    }

    pub(crate) fn append_borrowed(&mut self, bytes: &[u8]) {
        let s: &'static str = unsafe { core::mem::transmute(core::str::from_utf8(bytes).unwrap()) };
        self.bytes.append_borrowed(s, self.blob);
    }

    pub(crate) fn prepend_valid_escaped(&mut self, c: char) {
        self.bytes.prepend_valid_escaped(c, self.blob);
    }

    pub(crate) fn prepend_invalid_escaped(&mut self, b: u8) {
        self.bytes.prepend_invalid_escaped(b, self.blob);
    }

    pub(crate) fn prepend_borrowed(&mut self, bytes: &[u8]) {
        let s: &'static str = unsafe { core::mem::transmute(core::str::from_utf8(bytes).unwrap()) };
        self.bytes.prepend_borrowed(s, self.blob);
    }

    pub(crate) fn as_string(&self) -> Option<String> {
        self.bytes.try_to_string().ok()
    }

    // pub(crate) fn borrow_string(&self) -> Result<&str, &[u8]> {
    //     match std::str::from_utf8(self.bytes.as_raw()) {
    //         Ok(s) => Ok(s),
    //         Err(_) => Err(self.bytes.as_raw()),
    //     }
    // }

    pub(crate) fn len(&self) -> usize {
        self.bytes.len()
    }

    pub(crate) fn clear(&mut self) {
        self.bytes = unsafe { self.blob.alloc().as_mut() };
    }
}
