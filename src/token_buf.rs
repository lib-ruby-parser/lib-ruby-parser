use crate::Bytes;

#[derive(Debug, Default)]
pub(crate) struct TokenBuf {
    pub(crate) bytes: Bytes,
}

impl TokenBuf {
    pub(crate) fn new(bytes: &[u8]) -> Self {
        Self {
            bytes: Bytes::new(Vec::from(bytes)),
        }
    }

    pub(crate) fn take(&mut self) -> Self {
        std::mem::take(self)
    }

    pub(crate) fn push(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    pub(crate) fn append(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.push(*byte)
        }
    }

    pub(crate) fn prepend(&mut self, part: &[u8]) {
        let mut tmp = part.to_vec();
        tmp.extend(self.bytes.as_raw().iter());
        self.bytes.set_raw(tmp);
    }

    pub(crate) fn borrow_string(&self) -> Result<&str, &[u8]> {
        match std::str::from_utf8(self.bytes.as_raw()) {
            Ok(s) => Ok(s),
            Err(_) => Err(self.bytes.as_raw()),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.bytes.len()
    }

    pub(crate) fn clear(&mut self) {
        self.bytes.clear()
    }
}
