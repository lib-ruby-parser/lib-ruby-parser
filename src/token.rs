use crate::{
    containers::{loc_ptr::UnPtr, List, LocPtr},
    token_name, Bytes, LexState, Loc,
};

/// A token that is emitted by a lexer and consumed by a parser
#[derive(Clone)]
#[repr(C)]
pub struct Token {
    /// Numeric representation of the token type,
    /// e.g. 42 (for example) for tINTEGER
    pub token_type: i32,

    /// Value of the token,
    /// e.g "42" for 42
    pub token_value: Bytes,

    /// Location of the token
    pub loc: LocPtr,

    /// Lex state **before** reading the token
    pub lex_state_before: LexState,

    /// Lex state **after** reading the token
    pub lex_state_after: LexState,
}

use std::fmt;
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!(
            "[{}, {:?}, {}...{}]",
            token_name(self.token_type),
            self.token_value.to_string_lossy(),
            self.loc.begin,
            self.loc.end,
        ))
    }
}

impl Token {
    /// Returns a byte array of the token value
    pub fn as_bytes(&self) -> &[u8] {
        &self.token_value.raw
    }

    /// Returns a mut byte array of the token value
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.token_value.raw
    }

    /// Consumes a token and returns an owned byte array of the token value
    pub fn into_bytes(self) -> List<u8> {
        self.token_value.raw
    }

    /// Converts token value into `&str`
    pub fn as_str_lossy(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.token_value.raw)
    }

    /// Converts token to a string, replaces unknown chars to `U+FFFD`
    pub fn to_string_lossy(&self) -> String {
        self.token_value.to_string_lossy()
    }

    /// Converts token to a string
    pub fn to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        self.token_value.to_string()
    }

    /// Consumes a token and converts it into a string
    pub fn into_string(self) -> Result<String, std::string::FromUtf8Error> {
        self.token_value.into_string()
    }

    pub(crate) fn get_loc(&self) -> Loc {
        self.loc.clone().unptr()
    }
}
