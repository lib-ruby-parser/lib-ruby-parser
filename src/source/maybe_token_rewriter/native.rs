use crate::source::maybe_token_rewriter::MaybeTokenRewriterAPI;
use crate::source::token_rewriter::TokenRewriter;

/// Native MaybeTokenRewriter type.
pub type MaybeTokenRewriter = Option<TokenRewriter>;

impl MaybeTokenRewriterAPI for MaybeTokenRewriter {
    fn new_some(token_rewriter: TokenRewriter) -> Self {
        Some(token_rewriter)
    }

    fn new_none() -> Self {
        None
    }

    fn is_some(&self) -> bool {
        matches!(self, Some(_))
    }

    fn is_none(&self) -> bool {
        matches!(self, None)
    }

    fn as_token_rewriter(&self) -> Option<&TokenRewriter> {
        self.as_ref()
    }

    fn as_token_rewriter_mut(&mut self) -> Option<&mut TokenRewriter> {
        self.as_mut()
    }

    fn into_token_rewriter(self) -> TokenRewriter {
        self.unwrap()
    }
}
