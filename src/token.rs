use crate::bytes::BytesTrait;
use crate::parser::token_name;
use crate::{Bytes, LexState, Loc};

#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalList;
#[cfg(feature = "compile-with-external-structures")]
type List<T> = ExternalList<T>;
#[cfg(not(feature = "compile-with-external-structures"))]
type List<T> = Vec<T>;

/// Trait with common methods of Token (Rust- or external-based)
pub trait TokenTrait {
    /// Constructor
    fn new(
        token_type: i32,
        token_value: Bytes,
        loc: Loc,
        lex_state_before: LexState,
        lex_state_after: LexState,
    ) -> Self;

    /// Returns a byte array of the token value
    fn as_bytes(&self) -> &[u8] {
        self.token_value().as_raw()
    }

    /// Consumes a token and returns an owned byte array of the token value
    fn into_bytes(self) -> List<u8>
    where
        Self: Sized,
    {
        self.into_token_value().into_raw()
    }

    /// Converts token value into `&str`
    fn as_str_lossy(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.token_value().as_raw())
    }

    /// Converts token to a string, replaces unknown chars to `U+FFFD`
    fn to_string_lossy(&self) -> String {
        self.token_value().to_string_lossy()
    }

    /// Converts token to a string
    fn to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        self.token_value().to_string()
    }

    /// Consumes a token and converts it into a string
    fn into_string(self) -> Result<String, std::string::FromUtf8Error>
    where
        Self: Sized,
    {
        self.into_token_value().into_string()
    }

    /// Returns type of the token
    fn token_type(&self) -> i32;

    /// Returns name of the token
    fn token_name(&self) -> &'static str {
        token_name(self.token_type())
    }

    /// Returns value of the token
    fn token_value(&self) -> &Bytes;

    /// Sets token value
    fn set_token_value(&mut self, token_value: Bytes);

    /// Consumes self, returns owned values of the token
    fn into_token_value(self) -> Bytes;

    /// Returns location of the token
    fn loc(&self) -> &Loc;

    /// Returns lex state **before** reading the token
    fn lex_state_before(&self) -> LexState;

    /// Returns lex state **after** reading the token
    fn lex_state_after(&self) -> LexState;
}

/// A token that is emitted by a lexer and consumed by a parser
#[derive(Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Token {
    /// Numeric representation of the token type,
    /// e.g. 42 (for example) for tINTEGER
    token_type: i32,

    /// Value of the token,
    /// e.g "42" for 42
    token_value: Bytes,

    /// Location of the token
    loc: Loc,

    /// Lex state **before** reading the token
    lex_state_before: LexState,

    /// Lex state **after** reading the token
    lex_state_after: LexState,
}

use std::fmt;
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!(
            "[{}, {:?}, {}...{}]",
            self.token_name(),
            self.token_value.to_string_lossy(),
            self.loc.begin,
            self.loc.end,
        ))
    }
}

impl TokenTrait for Token {
    fn new(
        token_type: i32,
        token_value: Bytes,
        loc: Loc,
        lex_state_before: LexState,
        lex_state_after: LexState,
    ) -> Self {
        Self {
            token_type,
            token_value,
            loc,
            lex_state_before,
            lex_state_after,
        }
    }

    fn token_type(&self) -> i32 {
        self.token_type
    }

    fn token_value(&self) -> &Bytes {
        &self.token_value
    }

    fn set_token_value(&mut self, token_value: Bytes) {
        self.token_value = token_value
    }

    fn into_token_value(self) -> Bytes {
        self.token_value
    }

    fn loc(&self) -> &Loc {
        &self.loc
    }

    fn lex_state_before(&self) -> LexState {
        self.lex_state_before
    }

    fn lex_state_after(&self) -> LexState {
        self.lex_state_after
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "compile-with-external-structures")]
    #[test]
    fn test_size() {
        use super::Token;
        assert_eq!(std::mem::size_of::<Token>(), 56);
    }
}
