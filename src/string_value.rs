use crate::{Bytes, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct StringValue {
    pub bytes: Bytes,
}

impl StringValue {
    pub fn new(token: Box<Token>) -> Self {
        Self {
            bytes: token.token_value,
        }
    }

    pub fn empty() -> Self {
        Self {
            bytes: Bytes::empty(),
        }
    }
}
