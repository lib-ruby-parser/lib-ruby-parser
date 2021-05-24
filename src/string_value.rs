#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalPtr;
#[cfg(feature = "compile-with-external-structures")]
type Ptr<T> = ExternalPtr<T>;
#[cfg(not(feature = "compile-with-external-structures"))]
type Ptr<T> = Box<T>;

use crate::{containers::ptr::UnPtr, Bytes, Token};

/// Representation of the value of the string literal
///
/// In Ruby string literals don't have to be valid in their encoding.
/// Because of that we don't even try to convert them into string.
/// Instead, they are emitted as byte arrays that (if you want)
/// can be converted to a string.
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct StringValue {
    /// Byte array, can be converted to a string
    pub bytes: Bytes,
}

impl StringValue {
    pub(crate) fn new(token: Ptr<Token>) -> Self {
        Self {
            bytes: token.unptr().token_value,
        }
    }

    pub(crate) fn empty() -> Self {
        Self {
            bytes: Bytes::empty(),
        }
    }
}
