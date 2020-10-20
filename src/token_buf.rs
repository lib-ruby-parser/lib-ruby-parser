use crate::parser::TokenValue;

#[derive(Debug, Clone)]
pub(crate) enum TokenBuf {
    String(String),
    Bytes(Vec<u8>),
}

impl Default for TokenBuf {
    fn default() -> Self {
        TokenBuf::String("".to_owned())
    }
}

impl TokenBuf {
    pub(crate) fn into_bytes(self) -> Vec<u8> {
        match self {
            TokenBuf::String(s) => s.into_bytes(),
            TokenBuf::Bytes(bytes) => bytes,
        }
    }

    pub(crate) fn push(&mut self, c: char) {
        match self {
            TokenBuf::String(s) => s.push(c),
            TokenBuf::Bytes(bytes) => bytes.append(&mut c.to_string().into_bytes()),
        }
    }

    pub(crate) fn append(&mut self, part: &str) {
        match self {
            TokenBuf::String(s) => s.push_str(part),
            TokenBuf::Bytes(bytes) => bytes.append(&mut part.to_string().into_bytes()),
        }
    }

    pub(crate) fn prepend(&mut self, part: &str) {
        match self {
            TokenBuf::String(s) => {
                *s = format!("{}{}", part, s);
            }
            TokenBuf::Bytes(bytes) => {
                let mut tmp = part.as_bytes().to_vec();
                tmp.extend(bytes.iter());
                *bytes = tmp;
            }
        }
    }

    pub(crate) fn to_token_value(self) -> TokenValue {
        match self {
            TokenBuf::String(s) => TokenValue::String(s),
            TokenBuf::Bytes(bytes) => TokenValue::InvalidString(bytes),
        }
    }

    pub(crate) fn clear(&mut self) {
        match self {
            TokenBuf::String(s) => s.clear(),
            TokenBuf::Bytes(bytes) => bytes.clear(),
        }
    }
}
