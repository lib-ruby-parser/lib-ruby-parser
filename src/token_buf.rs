use crate::parser::TokenValue;

#[derive(Debug, Clone, Default)]
pub(crate) struct TokenBuf {
    bytes: Vec<u8>,
}

impl TokenBuf {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub(crate) fn push(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    pub(crate) fn append(&mut self, part: &str) {
        self.bytes.append(&mut part.to_string().into_bytes())
    }

    pub(crate) fn prepend(&mut self, part: &str) {
        let mut tmp = part.as_bytes().to_vec();
        tmp.extend(self.bytes.iter());
        self.bytes = tmp;
    }

    pub(crate) fn borrow_string(&self) -> Result<&str, &Vec<u8>> {
        match std::str::from_utf8(&self.bytes) {
            Ok(s) => Ok(s),
            Err(_) => Err(&self.bytes),
        }
    }

    pub(crate) fn to_string(self) -> Result<String, Vec<u8>> {
        match std::str::from_utf8(&self.bytes) {
            Ok(s) => Ok(s.to_owned()),
            Err(_) => Err(self.bytes),
        }
    }

    pub(crate) fn to_token_value(self) -> TokenValue {
        match self.to_string() {
            Ok(s) => TokenValue::String(s),
            Err(bytes) => TokenValue::InvalidString(bytes),
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
        other.as_bytes() == self.bytes
    }
}
