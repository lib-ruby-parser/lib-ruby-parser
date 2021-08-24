use super::InternalTokenRewriterResult;
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

impl RewriteAction {
    pub(crate) fn is_drop(&self) -> bool {
        matches!(self, Self::Drop)
    }

    pub(crate) fn is_keep(&self) -> bool {
        matches!(self, Self::Keep)
    }
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

impl LexStateAction {
    pub(crate) fn is_set(&self) -> bool {
        matches!(self, Self::Set(_))
    }

    pub(crate) fn is_keep(&self) -> bool {
        matches!(self, Self::Keep)
    }

    pub(crate) fn next_state(&self) -> i32 {
        match self {
            Self::Set(state) => *state,
            Self::Keep => panic!("Wrong variant of LexStateAction"),
        }
    }
}

/// Output of the token rewriter
#[derive(Debug)]
#[repr(C)]
pub struct TokenRewriterResult {
    /// Rewritten token. Can be input token if no rewriting expected
    pub rewritten_token: Box<Token>,

    /// Action to be applied on a token (keep or drop)
    pub token_action: RewriteAction,

    /// Action to be applied on lexer's state (keep as is or change)
    pub lex_state_action: LexStateAction,
}

impl TokenRewriterResult {
    pub(crate) fn into_internal(self) -> InternalTokenRewriterResult {
        let Self {
            rewritten_token,
            token_action,
            lex_state_action,
        } = self;
        InternalTokenRewriterResult {
            rewritten_token,
            token_action,
            lex_state_action,
        }
    }
}

/// Token rewriter function
pub type TokenRewriterFn = dyn Fn(Box<Token>, &[u8]) -> TokenRewriterResult;

/// Token rewriter struct, can be used to rewrite tokens on the fly
pub struct TokenRewriter {
    f: Box<TokenRewriterFn>,
}

impl TokenRewriter {
    /// Constructs a rewriter based on a given function
    pub fn new(f: Box<TokenRewriterFn>) -> Self {
        Self { f }
    }

    pub(crate) fn call(&self, token: Box<Token>, input: &[u8]) -> TokenRewriterResult {
        let f = &*self.f;
        f(token, input)
    }
}
