use crate::Bytes;

#[derive(Debug, Clone, Default)]
pub(crate) struct TokenBuf {
    pub(crate) bytes: Bytes,
}

impl TokenBuf {
    pub(crate) fn new(bytes: &[u8]) -> Self {
        Self {
            bytes: Bytes::new(bytes.to_vec()),
        }
    }

    pub(crate) fn take(&mut self) -> Self {
        std::mem::take(self)
    }

    pub(crate) fn push(&mut self, byte: u8) {
        self.bytes.raw.push(byte);
    }

    pub(crate) fn append(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.push(*byte)
        }
    }

    pub(crate) fn prepend(&mut self, part: &[u8]) {
        let mut tmp = part.to_vec();
        tmp.extend(self.bytes.raw.iter());
        self.bytes.raw = tmp.into();
    }

    pub(crate) fn borrow_string(&self) -> Result<&str, &[u8]> {
        match std::str::from_utf8(&self.bytes.raw) {
            Ok(s) => Ok(s),
            Err(_) => Err(&self.bytes.raw),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.bytes.len()
    }

    pub(crate) fn clear(&mut self) {
        self.bytes.clear()
    }
}

impl PartialEq<str> for TokenBuf {
    fn eq(&self, other: &str) -> bool {
        other.as_bytes() == self.bytes.raw.as_ref()
    }
}
