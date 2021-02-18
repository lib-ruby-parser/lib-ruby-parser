use crate::{Bytes, Token};

/// Representation of the value of the string literal
///
/// In Ruby string literals don't have to be valid in their encoding.
/// Because of that we don't even try to convert them into string.
/// Instead, they are emitted as byte arrays that (if you want)
/// can be converted to a string.
#[derive(Debug, Clone, PartialEq)]
pub struct StringValue {
    /// Byte array, can be converted to a string
    pub bytes: Bytes,
}

impl StringValue {
    pub(crate) fn new(token: Box<Token>) -> Self {
        Self {
            bytes: token.token_value,
        }
    }

    pub(crate) fn empty() -> Self {
        Self {
            bytes: Bytes::empty(),
        }
    }
}
