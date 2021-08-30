use crate::blobs::BytesBlob;
use crate::blobs::LocBlob;
use crate::blobs::TokenBlob;
use crate::containers::IntoBlob;
use crate::{Bytes, LexState, Loc};

/// Byte sequence based on external implementation
#[repr(C)]
pub struct Token {
    pub(crate) blob: TokenBlob,
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
        unsafe { lib_ruby_parser__external__token__drop(&mut self.blob) }
    }
}

extern "C" {
    fn lib_ruby_parser__external__token__new(
        token_type: i32,
        token_value: BytesBlob,
        loc: LocBlob,
        lex_state_before: i32,
        lex_state_after: i32,
    ) -> TokenBlob;
    fn lib_ruby_parser__external__token__drop(blob: *mut TokenBlob);
    fn lib_ruby_parser__external__token__get_token_type(blob: *const TokenBlob) -> i32;
    fn lib_ruby_parser__external__token__get_token_value(
        blob: *const TokenBlob,
    ) -> *const BytesBlob;
    fn lib_ruby_parser__external__token__set_token_value(
        blob: *mut TokenBlob,
        bytes_blob: BytesBlob,
    );
    fn lib_ruby_parser__external__token__into_token_value(blob: TokenBlob) -> BytesBlob;
    fn lib_ruby_parser__external__token__get_loc(blob: *const TokenBlob) -> *const LocBlob;
    fn lib_ruby_parser__external__token__get_lex_state_before(blob: *const TokenBlob) -> i32;
    fn lib_ruby_parser__external__token__get_lex_state_after(blob: *const TokenBlob) -> i32;
}

impl Token {
    /// Constructor
    pub fn new(
        token_type: i32,
        token_value: Bytes,
        loc: Loc,
        lex_state_before: LexState,
        lex_state_after: LexState,
    ) -> Self {
        let blob = unsafe {
            lib_ruby_parser__external__token__new(
                token_type,
                token_value.into_blob(),
                loc.into_blob(),
                lex_state_before.get(),
                lex_state_after.get(),
            )
        };
        Self { blob }
    }

    /// Returns type of the token
    pub fn token_type(&self) -> i32 {
        unsafe { lib_ruby_parser__external__token__get_token_type(&self.blob) }
    }

    /// Returns type of the token
    pub fn token_value(&self) -> &Bytes {
        unsafe {
            (lib_ruby_parser__external__token__get_token_value(&self.blob) as *const Bytes)
                .as_ref()
                .unwrap()
        }
    }

    /// Sets token value
    pub fn set_token_value(&mut self, token_value: Bytes) {
        unsafe {
            lib_ruby_parser__external__token__set_token_value(
                &mut self.blob,
                token_value.into_blob(),
            )
        }
    }

    /// Consumes self, returns owned values of the token
    pub fn into_token_value(self) -> Bytes {
        let bytes_blob = unsafe { lib_ruby_parser__external__token__into_token_value(self.blob) };
        std::mem::forget(self);
        Bytes { blob: bytes_blob }
    }

    /// Returns location of the token
    pub fn loc(&self) -> &Loc {
        unsafe {
            (lib_ruby_parser__external__token__get_loc(&self.blob) as *const Loc)
                .as_ref()
                .unwrap()
        }
    }

    /// Returns lex state **before** reading the token
    pub fn lex_state_before(&self) -> LexState {
        let value = unsafe { lib_ruby_parser__external__token__get_lex_state_before(&self.blob) };
        LexState { value }
    }

    /// Returns lex state **after** reading the token
    pub fn lex_state_after(&self) -> LexState {
        let value = unsafe { lib_ruby_parser__external__token__get_lex_state_after(&self.blob) };
        LexState { value }
    }
}
