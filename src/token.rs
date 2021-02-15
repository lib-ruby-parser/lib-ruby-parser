use crate::{token_name, Bytes, LexState, Loc};

/// A token that is emitted by a lexer and consumed by a parser
#[derive(Clone)]
pub struct Token {
    pub token_type: i32,
    pub token_value: Bytes,
    pub loc: Loc,
    pub lex_state_before: LexState,
    pub lex_state_after: LexState,
}

use std::fmt;
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!(
            "[{}, {:?}, {}...{}, {:?} -> {:?}]",
            token_name(self.token_type),
            self.token_value.to_string_lossy(),
            self.loc.begin,
            self.loc.end,
            self.lex_state_before,
            self.lex_state_after
        ))
    }
}

impl Token {
    pub fn as_bytes(&self) -> &[u8] {
        &self.token_value.raw
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.token_value.raw
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.token_value.raw
    }

    pub fn as_str_lossy(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.token_value.raw)
    }

    /// Converts Token to a string, replaces unknown chars to `U+FFFD`
    pub fn to_string_lossy(&self) -> String {
        self.token_value.to_string_lossy()
    }

    pub fn to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        self.token_value.to_string()
    }

    pub fn into_string(self) -> Result<String, std::string::FromUtf8Error> {
        self.token_value.into_string()
    }
}
