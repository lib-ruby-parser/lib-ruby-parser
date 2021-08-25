use crate::source::token_rewriter::TokenRewriter;

/// Trait with common MaybeTokenRewriter APIs
pub trait MaybeTokenRewriterAPI {
    /// Constructs `Some` variant
    fn new_some(token_rewriter: TokenRewriter) -> Self
    where
        Self: Sized;

    /// Constructs `None` variant
    fn new_none() -> Self
    where
        Self: Sized;

    /// Returns true if `self` is `Some`
    fn is_some(&self) -> bool;

    /// Returns true if `self` is `None`
    fn is_none(&self) -> bool;

    /// Casts &self to Option<&TokenRewriter>
    fn as_token_rewriter(&self) -> Option<&TokenRewriter>;

    /// Casts self to TokenRewriter. Panics if self is `None`
    fn into_token_rewriter(self) -> TokenRewriter;
}
