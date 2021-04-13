use crate::{
    containers::{
        maybe_ptr::{AsOption, MaybePtrNone, MaybePtrSome},
        MaybePtr, Ptr, SharedList,
    },
    Token,
};

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

/// Output of the token rewriter
pub type TokenRewriterResult = (Ptr<Token>, RewriteAction, LexStateAction);

/// Token rewriter function
pub type TokenRewriterFn = fn(Ptr<Token>, SharedList<u8>) -> TokenRewriterResult;

/// Token rewriter struct, can be used to rewrite tokens on the fly
#[repr(C)]
pub struct TokenRewriter {
    f: MaybePtr<TokenRewriterFn>,
}

impl TokenRewriter {
    /// Constructs a rewriter based on a given function
    pub fn new(f: TokenRewriterFn) -> Self {
        Self {
            f: MaybePtr::some(f),
        }
    }

    /// Constructs a no-op token rewriter that has no side effect. Default value.
    pub fn none() -> Self {
        Self {
            f: MaybePtr::none(),
        }
    }

    /// Returns an optional reference to a function that rewrite tokens
    pub fn as_option(&self) -> Option<&TokenRewriterFn> {
        self.f.as_option()
    }
}

impl std::fmt::Debug for TokenRewriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenRewriter")
            .field("f", &self.f.as_option().map(|_| "function"))
            .finish()
    }
}
