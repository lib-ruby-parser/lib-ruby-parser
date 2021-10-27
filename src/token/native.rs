use crate::{Bytes, LexState, Loc};

/// A token that is emitted by a lexer and consumed by a parser
#[derive(Clone, PartialEq, Eq)]
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

    /// Lex state **before** reading the token
    pub lex_state_before: LexState,

    /// Lex state **after** reading the token
    pub lex_state_after: LexState,
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
        Self {
            token_type,
            token_value,
            loc,
            lex_state_before,
            lex_state_after,
        }
    }

    /// Returns type of the token
    pub fn token_type(&self) -> i32 {
        self.token_type
    }

    /// Returns type of the token
    pub fn token_value(&self) -> &Bytes {
        &self.token_value
    }

    /// Sets token value
    pub fn set_token_value(&mut self, token_value: Bytes) {
        self.token_value = token_value
    }

    /// Consumes self, returns owned values of the token
    pub fn into_token_value(self) -> Bytes {
        self.token_value
    }

    /// Returns location of the token
    pub fn loc(&self) -> &Loc {
        &self.loc
    }

    /// Returns lex state **before** reading the token
    pub fn lex_state_before(&self) -> LexState {
        self.lex_state_before
    }

    /// Returns lex state **after** reading the token
    pub fn lex_state_after(&self) -> LexState {
        self.lex_state_after
    }
}
