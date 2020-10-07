use crate::parser::TokenValue;

#[derive(Debug, Clone)]
pub enum TokenBuf {
    String(String),
    Bytes(Vec<u8>),
}

impl Default for TokenBuf {
    fn default() -> Self {
        TokenBuf::String("".to_owned())
    }
}

impl TokenBuf {
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            TokenBuf::String(s) => s.into_bytes(),
            TokenBuf::Bytes(bytes) => bytes
        }
    }

    pub fn push(&mut self, c: char) {
        match self {
            TokenBuf::String(s) => s.push(c),
            TokenBuf::Bytes(bytes) => bytes.append(&mut c.to_string().into_bytes())
        }
    }

    pub fn append(&mut self, part: &str) {
        match self {
            TokenBuf::String(s) => s.push_str(part),
            TokenBuf::Bytes(bytes) => bytes.append(&mut part.to_string().into_bytes())
        }
    }

    pub fn prepend(&mut self, s: &str) {
        match self {
            TokenBuf::String(s2) => {
                *s2 = format!("{}{}", s, s2);
            }
            TokenBuf::Bytes(bytes) => {
                let mut tmp = s.as_bytes().to_vec();
                tmp.extend(bytes.iter());
                *bytes = tmp;
            }
        }
    }

    pub fn to_token_value(self) -> TokenValue {
        match self {
            TokenBuf::String(s) => TokenValue::String(s),
            TokenBuf::Bytes(bytes) => TokenValue::InvalidString(bytes)
        }
    }

    pub fn clear(&mut self) {
        match self {
            TokenBuf::String(s) => s.clear(),
            TokenBuf::Bytes(bytes) => bytes.clear()
        }
    }
}
