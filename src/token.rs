use crate::{token_name, LexState, Loc};

#[derive(Debug, Clone)]
pub enum TokenValue {
    String(String),
    InvalidString(Vec<u8>),
}
impl TokenValue {
    /// Converts TokenValue to string, replaces unknown chars to `U+FFFD`
    pub fn to_string_lossy(&self) -> String {
        match &self {
            Self::String(s) => s.clone(),
            Self::InvalidString(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
        }
    }

    /// Converts TokenValue to a vector of bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        match &self {
            Self::String(s) => s.as_bytes().to_vec(),
            Self::InvalidString(bytes) => bytes.clone(),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        match &self {
            Self::String(s) => s.as_bytes(),
            Self::InvalidString(bytes) => bytes,
        }
    }

    pub fn into_string_lossy(self) -> String {
        match self {
            Self::String(s) => s,
            Self::InvalidString(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            Self::String(s) => s.into_bytes(),
            Self::InvalidString(bytes) => bytes,
        }
    }
}

/// A token that is emitted by a lexer and consumed by a parser
#[derive(Clone)]
pub struct Token {
    pub token_type: i32,
    pub token_value: TokenValue,
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
            self.token_value,
            self.loc.begin,
            self.loc.end,
            self.lex_state_before,
            self.lex_state_after
        ))
    }
}

impl Token {
    /// Converts Token to a string, replaces unknown chars to `U+FFFD`
    pub fn to_string_lossy(&self) -> String {
        self.token_value.to_string_lossy()
    }

    /// Converts Token to a vector of bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        self.token_value.to_bytes()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.token_value.as_bytes()
    }

    pub fn into_string_lossy(self) -> String {
        self.token_value.into_string_lossy()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.token_value.into_bytes()
    }
}
