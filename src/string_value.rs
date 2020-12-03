use crate::{Token, TokenValue};

#[derive(Debug, Clone, PartialEq)]
pub struct StringValue {
    pub valid: bool,
    pub bytes: Vec<u8>,
}

impl StringValue {
    pub fn new(token: Token) -> Self {
        match token.token_value {
            TokenValue::String(s) => StringValue {
                valid: true,
                bytes: s.as_bytes().to_owned(),
            },
            TokenValue::InvalidString(bytes) => StringValue {
                valid: false,
                bytes,
            },
        }
    }

    pub fn empty() -> Self {
        StringValue {
            valid: true,
            bytes: vec![],
        }
    }

    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.bytes).into_owned()
    }

    pub fn to_string(&self) -> Option<String> {
        String::from_utf8(self.bytes.clone()).ok()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}
