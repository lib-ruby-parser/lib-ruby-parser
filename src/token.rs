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
pub trait TokenTrait: Clone + PartialEq + Eq + std::fmt::Debug {
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
    fn loc(&self) -> Loc;

    /// Returns lex state **before** reading the token
    fn lex_state_before(&self) -> LexState;

    /// Returns lex state **after** reading the token
    fn lex_state_after(&self) -> LexState;
}

#[cfg(not(feature = "compile-with-external-structures"))]
mod token {
    use super::{Bytes, BytesTrait, LexState, Loc, TokenTrait};

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

        fn loc(&self) -> Loc {
            self.loc
        }

        fn lex_state_before(&self) -> LexState {
            self.lex_state_before
        }

        fn lex_state_after(&self) -> LexState {
            self.lex_state_after
        }
    }
}

#[cfg(feature = "compile-with-external-structures")]
mod token {
    use super::TokenTrait;
    use crate::containers::size::TOKEN_SIZE;
    use crate::{Bytes, BytesTrait, LexState, Loc};

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct TokenBlob {
        blob: [u8; TOKEN_SIZE],
    }

    /// Byte sequence based on external implementation
    #[repr(C)]
    pub struct Token {
        blob: TokenBlob,
    }

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

    impl Clone for Token {
        fn clone(&self) -> Self {
            Self::new(
                self.token_type(),
                self.token_value().clone(),
                self.loc().clone(),
                self.lex_state_before(),
                self.lex_state_after(),
            )
        }
    }

    impl PartialEq for Token {
        fn eq(&self, other: &Self) -> bool {
            (self.token_type() == other.token_type())
                && (self.token_value() == other.token_value())
                && (self.loc() == other.loc())
                && (self.lex_state_before() == other.lex_state_before())
                && (self.lex_state_after() == other.lex_state_after())
        }
    }

    impl Eq for Token {}

    impl Drop for Token {
        fn drop(&mut self) {
            unsafe { lib_ruby_parser_token_blob_free(self.blob) }
        }
    }

    use crate::bytes::bytes::BytesBlob;
    extern "C" {
        fn lib_ruby_parser_token_blob_new(
            token_type: i32,
            token_value: BytesBlob,
            loc: Loc,
            lex_state_before: i32,
            lex_state_after: i32,
        ) -> TokenBlob;
        fn lib_ruby_parser_token_blob_get_token_type(token_blob: TokenBlob) -> i32;
        fn lib_ruby_parser_token_blob_borrow_token_value(
            token_blob: *const TokenBlob,
        ) -> *const BytesBlob;
        fn lib_ruby_parser_token_set_token_value(
            token_blob: TokenBlob,
            bytes_blob: BytesBlob,
        ) -> TokenBlob;
        fn lib_ruby_parser_token_blob_into_token_value(token_blob: TokenBlob) -> BytesBlob;
        fn lib_ruby_parser_token_blob_borrow_loc(token_blob: TokenBlob) -> Loc;
        fn lib_ruby_parser_token_blob_get_lex_state_before(token_blob: TokenBlob) -> i32;
        fn lib_ruby_parser_token_blob_get_lex_state_after(token_blob: TokenBlob) -> i32;
        fn lib_ruby_parser_token_blob_free(token_blob: TokenBlob);
    }

    impl TokenTrait for Token {
        fn new(
            token_type: i32,
            token_value: Bytes,
            loc: Loc,
            lex_state_before: LexState,
            lex_state_after: LexState,
        ) -> Self {
            let blob = unsafe {
                lib_ruby_parser_token_blob_new(
                    token_type,
                    token_value.into_blob(),
                    loc,
                    lex_state_before.get(),
                    lex_state_after.get(),
                )
            };
            Self { blob }
        }

        fn token_type(&self) -> i32 {
            unsafe { lib_ruby_parser_token_blob_get_token_type(self.blob) }
        }

        fn token_value(&self) -> &Bytes {
            let token_blob_ptr: *const TokenBlob = &self.blob;
            let bytes_ptr = unsafe {
                lib_ruby_parser_token_blob_borrow_token_value(token_blob_ptr) as *const Bytes
            };
            unsafe { bytes_ptr.as_ref().unwrap() }
        }

        fn set_token_value(&mut self, token_value: Bytes) {
            self.blob =
                unsafe { lib_ruby_parser_token_set_token_value(self.blob, token_value.into_blob()) }
        }

        fn into_token_value(self) -> Bytes {
            let bytes_blob = unsafe { lib_ruby_parser_token_blob_into_token_value(self.blob) };
            std::mem::forget(self);
            Bytes { blob: bytes_blob }
        }

        fn loc(&self) -> Loc {
            unsafe { lib_ruby_parser_token_blob_borrow_loc(self.blob) }
        }

        fn lex_state_before(&self) -> LexState {
            let value = unsafe { lib_ruby_parser_token_blob_get_lex_state_before(self.blob) };
            let mut lex_state = LexState::default();
            lex_state.set(value);
            lex_state
        }

        fn lex_state_after(&self) -> LexState {
            let value = unsafe { lib_ruby_parser_token_blob_get_lex_state_after(self.blob) };
            let mut lex_state = LexState::default();
            lex_state.set(value);
            lex_state
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{Bytes, BytesTrait, LexState, Loc, Token, TokenTrait, TOKEN_SIZE};

        #[test]
        fn test_size() {
            assert_eq!(std::mem::size_of::<Token>(), TOKEN_SIZE);
        }

        fn lex_state(value: i32) -> LexState {
            let mut lex_state = LexState::default();
            lex_state.set(value);
            lex_state
        }

        fn new_token() -> Token {
            Token::new(
                1,
                Bytes::new(vec![1, 2, 3]),
                Loc { begin: 1, end: 2 },
                lex_state(1),
                lex_state(2),
            )
        }

        #[test]
        fn test_new() {
            let token = new_token();
            drop(token);
        }

        #[test]
        fn test_token_type() {
            let token = new_token();
            assert_eq!(token.token_type(), 1)
        }

        #[test]
        fn test_token_value() {
            let token = new_token();
            assert_eq!(token.token_value(), &Bytes::new(vec![1, 2, 3]));
        }

        #[test]
        fn test_set_token_value() {
            let mut token = new_token();
            token.set_token_value(Bytes::new(vec![4, 5, 6]));
            assert_eq!(token.token_value(), &Bytes::new(vec![4, 5, 6]));
        }

        #[test]
        fn test_into_token_value() {
            let token = new_token();
            assert_eq!(token.into_token_value(), Bytes::new(vec![1, 2, 3]))
        }

        #[test]
        fn test_loc() {
            let token = new_token();
            assert_eq!(token.loc(), Loc { begin: 1, end: 2 });
        }

        #[test]
        fn test_lex_state_before() {
            let token = new_token();
            assert_eq!(token.lex_state_before(), lex_state(1));
        }

        #[test]
        fn test_lex_state_after() {
            let token = new_token();
            assert_eq!(token.lex_state_after(), lex_state(2));
        }
    }
}

pub use token::Token;
