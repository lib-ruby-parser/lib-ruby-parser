use crate::Token;

/// Enum of what token rewriter should do with a token.
#[derive(Debug)]
pub enum RewriteAction {
    /// Means "drop the token", i.e. don't return it to a parser
    Drop,

    /// Means "keep the token", i.e. return it to a parser
    Keep,
}

/// Enum of what token rewriter should do with the state of the lexer
#[derive(Debug)]
pub enum LexStateAction {
    /// Means "set the state to X"
    Set(i32),

    /// Means "keep the state unchanged"
    Keep,
}

/// A trait that must be implement to perform a token rewriting
pub trait TokenRewriter: std::fmt::Debug {
    /// Invoked for every token that is return from a lexer.
    ///
    /// Returns a triplet of `(<new token>, <change token action>, <change lexer.state action>)`
    fn rewrite_token(
        &mut self,
        token: Box<Token>,
        input: &[u8],
    ) -> (Box<Token>, RewriteAction, LexStateAction);
}
