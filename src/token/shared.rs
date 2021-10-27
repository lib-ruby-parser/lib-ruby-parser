use super::Token;
use crate::parser::token_name;

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "[{}, {:?}, {}...{}]",
            self.token_name(),
            self.token_value().to_string_lossy(),
            self.loc().begin,
            self.loc().end,
        ))
    }
}

impl Token {
    /// Returns a byte array of the token value
    pub fn as_bytes(&self) -> &Vec<u8> {
        self.token_value().as_raw()
    }

    /// Consumes a token and returns an owned byte array of the token value
    pub fn into_bytes(self) -> Vec<u8> {
        self.into_token_value().into_raw()
    }

    /// Converts token value into `&str`
    pub fn as_str_lossy(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.token_value().as_raw())
    }

    /// Converts token to a string, replaces unknown chars to `U+FFFD`
    pub fn to_string_lossy(&self) -> String {
        self.token_value().to_string_lossy()
    }

    /// Converts token to a string
    pub fn to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        self.token_value().to_string()
    }

    /// Consumes a token and converts it into a string
    pub fn into_string(self) -> Result<String, std::string::FromUtf8Error> {
        self.into_token_value().into_string()
    }

    /// Returns name of the token
    pub fn token_name(&self) -> &'static str {
        token_name(self.token_type())
    }
}
