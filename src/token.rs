use crate::parser::token_name;
use crate::{Bytes, Loc};

/// A token that is emitted by a lexer and consumed by a parser
#[derive(PartialEq, Eq)]
#[repr(C)]
pub struct Token {
    /// Numeric representation of the token type,
    /// e.g. 42 (for example) for tINTEGER
    pub token_type: i32,

    /// Value of the token,
    /// e.g "42" for 42
    pub token_value: Bytes,

    /// Location of the token
    pub loc: Loc,
}

impl Token {
    /// Returns a byte array of the token value
    pub fn as_bytes(&self) -> &Vec<u8> {
        self.token_value.as_raw()
    }

    /// Consumes a token and returns an owned byte array of the token value
    pub fn into_bytes(self) -> Vec<u8> {
        self.token_value.into_raw()
    }

    /// Converts token value into `&str`
    pub fn as_str_lossy(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.token_value.as_raw())
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

    /// Returns name of the token
    pub fn token_name(&self) -> &'static str {
        token_name(self.token_type)
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "[{}, {:?}, {}...{}]",
            self.token_name(),
            self.token_value.to_string_lossy(),
            self.loc.begin,
            self.loc.end,
        ))
    }
}

#[cfg(test)]
fn new_token() -> Token {
    Token {
        token_type: crate::Lexer::tINTEGER,
        token_value: Bytes::new(vec![42]),
        loc: Loc { begin: 1, end: 2 },
    }
}

#[test]
fn test_as_bytes() {
    let token = new_token();
    assert_eq!(token.as_bytes(), &vec![42]);
}

#[test]
fn test_into_bytes() {
    let token = new_token();
    assert_eq!(token.into_bytes(), vec![42]);
}

#[test]
fn test_as_str_lossy() {
    let token = new_token();
    assert_eq!(token.as_str_lossy(), Ok("*"));
}

#[test]
fn test_to_string_lossy() {
    let token = new_token();
    assert_eq!(token.to_string_lossy(), String::from("*"));
}

#[test]
fn test_fmt() {
    let token = new_token();
    assert_eq!(format!("{:?}", token), "[tINTEGER, \"*\", 1...2]");
}
