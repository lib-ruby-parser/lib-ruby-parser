#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalPtr;
#[cfg(feature = "compile-with-external-structures")]
type Ptr<T> = ExternalPtr<T>;
#[cfg(not(feature = "compile-with-external-structures"))]
type Ptr<T> = Box<T>;

#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalSharedByteList;
#[cfg(feature = "compile-with-external-structures")]
type SharedByteList = ExternalSharedByteList;
#[cfg(not(feature = "compile-with-external-structures"))]
type SharedByteList<'a> = &'a [u8];

use crate::Token;

/// Enum of what token rewriter should do with a token.
#[derive(Debug)]
#[repr(C)]
pub enum RewriteAction {
    /// Means "drop the token", i.e. don't return it to a parser
    Drop,

    /// Means "keep the token", i.e. return it to a parser
    Keep,
}

/// Enum of what token rewriter should do with the state of the lexer
#[derive(Debug)]
#[repr(C)]
pub enum LexStateAction {
    /// Means "set the state to X"
    Set(i32),

    /// Means "keep the state unchanged"
    Keep,
}

/// Output of the token rewriter
#[derive(Debug)]
#[repr(C)]
pub struct TokenRewriterResult {
    /// Rewritten token. Can be input token if no rewriting expected
    pub rewritten_token: Ptr<Token>,

    /// Action to be applied on a token (keep or drop)
    pub token_action: RewriteAction,

    /// Action to be applied on lexer's state (keep as is or change)
    pub lex_state_action: LexStateAction,
}

/// Token rewriter function
pub type TokenRewriterFn = dyn Fn(Ptr<Token>, SharedByteList) -> TokenRewriterResult;

/// Token rewriter struct, can be used to rewrite tokens on the fly
pub struct TokenRewriter {
    f: Option<Box<TokenRewriterFn>>,
}

impl TokenRewriter {
    /// Constructs a rewriter based on a given function
    pub fn new(f: Box<TokenRewriterFn>) -> Self {
        Self { f: Some(f) }
    }

    /// Constructs a no-op token rewriter that has no side effect. Default value.
    pub fn none() -> Self {
        Self { f: None }
    }

    /// Returns an optional reference to a function that rewrite tokens
    pub fn as_option(&self) -> Option<&TokenRewriterFn> {
        if let Some(f) = &self.f {
            Some(&**f)
        } else {
            None
        }
    }
}

impl std::fmt::Debug for TokenRewriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenRewriter")
            .field("f", &self.as_option().map(|_| "function"))
            .finish()
    }
}
