use super::InternalTokenRewriterResult;
use crate::containers::size::{
    LEX_STATE_ACTION_SIZE, REWRITE_ACTION_SIZE, TOKEN_REWRITER_RESULT_SIZE,
};
use crate::containers::{ExternalPtr as Ptr, ExternalSharedByteList as SharedByteList};
use crate::Token;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct RewriteActionBlob {
    blob: [u8; REWRITE_ACTION_SIZE],
}

/// Enum of what token rewriter should do with a token.
#[repr(C)]
pub struct RewriteAction {
    pub(crate) blob: RewriteActionBlob,
}

extern "C" {
    fn lib_ruby_parser__internal__containers__token_rewriter__rewrite_action__is_drop(
        blob: *const RewriteActionBlob,
    ) -> bool;
    fn lib_ruby_parser__internal__containers__token_rewriter__rewrite_action__is_keep(
        blob: *const RewriteActionBlob,
    ) -> bool;
    fn lib_ruby_parser__internal__containers__token_rewriter__rewrite_action__drop(
        blob: *mut RewriteActionBlob,
    );
}

impl Drop for RewriteAction {
    fn drop(&mut self) {
        unsafe {
            lib_ruby_parser__internal__containers__token_rewriter__rewrite_action__drop(
                &mut self.blob,
            )
        }
    }
}

impl RewriteAction {
    pub(crate) fn is_drop(&self) -> bool {
        unsafe {
            lib_ruby_parser__internal__containers__token_rewriter__rewrite_action__is_drop(
                &self.blob,
            )
        }
    }

    pub(crate) fn is_keep(&self) -> bool {
        unsafe {
            lib_ruby_parser__internal__containers__token_rewriter__rewrite_action__is_keep(
                &self.blob,
            )
        }
    }
}

impl std::fmt::Debug for RewriteAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_drop() {
            write!(f, "Drop")
        } else if self.is_keep() {
            write!(f, "Keep")
        } else {
            panic!("Unknown RewriteAction variant")
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct LexStateActionBlob {
    blob: [u8; LEX_STATE_ACTION_SIZE],
}

/// Enum of what token rewriter should do with the state of the lexer
#[repr(C)]
pub struct LexStateAction {
    pub(crate) blob: LexStateActionBlob,
}

extern "C" {
    fn lib_ruby_parser__internal__containers__token_rewriter__lex_state_action__is_set(
        blob: *const LexStateActionBlob,
    ) -> bool;
    fn lib_ruby_parser__internal__containers__token_rewriter__lex_state_action__is_keep(
        blob: *const LexStateActionBlob,
    ) -> bool;
    fn lib_ruby_parser__internal__containers__token_rewriter__lex_state_action__drop(
        blob: *mut LexStateActionBlob,
    );
    fn lib_ruby_parser__internal__containers__token_rewriter__lex_state_action__get_next_state(
        blob: *const LexStateActionBlob,
    ) -> i32;
}

impl Drop for LexStateAction {
    fn drop(&mut self) {
        unsafe {
            lib_ruby_parser__internal__containers__token_rewriter__lex_state_action__drop(
                &mut self.blob,
            )
        }
    }
}

impl LexStateAction {
    pub(crate) fn is_set(&self) -> bool {
        unsafe {
            lib_ruby_parser__internal__containers__token_rewriter__lex_state_action__is_set(
                &self.blob,
            )
        }
    }

    pub(crate) fn is_keep(&self) -> bool {
        unsafe {
            lib_ruby_parser__internal__containers__token_rewriter__lex_state_action__is_keep(
                &self.blob,
            )
        }
    }

    pub(crate) fn next_state(&self) -> i32 {
        unsafe {
            lib_ruby_parser__internal__containers__token_rewriter__lex_state_action__get_next_state(
                &self.blob,
            )
        }
    }
}

impl std::fmt::Debug for LexStateAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_keep() {
            write!(f, "Keep")
        } else if self.is_set() {
            write!(f, "Set({})", self.next_state())
        } else {
            panic!("Unknown LexStateAction variant")
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct TokenRewriterResultBlob {
    blob: [u8; TOKEN_REWRITER_RESULT_SIZE],
}

/// Output of the token rewriter
#[repr(C)]
pub struct TokenRewriterResult {
    pub(crate) blob: TokenRewriterResultBlob,
}

extern "C" {
    fn lib_ruby_parser__internal__containers__token_rewriter__into_internal(
        blob: TokenRewriterResultBlob,
    ) -> InternalTokenRewriterResult;
    fn lib_ruby_parser__internal__containers__token_rewriter__drop(
        blob: *mut TokenRewriterResultBlob,
    );
}

impl Drop for TokenRewriterResult {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__internal__containers__token_rewriter__drop(&mut self.blob) }
    }
}

impl TokenRewriterResult {
    pub(crate) fn into_internal(self) -> InternalTokenRewriterResult {
        let internal = unsafe {
            lib_ruby_parser__internal__containers__token_rewriter__into_internal(self.blob)
        };
        std::mem::forget(self);
        internal
    }
}

impl std::fmt::Debug for TokenRewriterResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenRewriterResult").finish()
    }
}

/// Token rewriter function
pub type TokenRewriterFn = dyn Fn(Ptr<Token>, SharedByteList) -> TokenRewriterResult;

/// Token rewriter struct, can be used to rewrite tokens on the fly
pub struct TokenRewriter {
    f: Box<TokenRewriterFn>,
}

impl TokenRewriter {
    /// Constructs a rewriter based on a given function
    pub fn new(f: Box<TokenRewriterFn>) -> Self {
        Self { f }
    }

    pub(crate) fn call(&self, token: Ptr<Token>, input: SharedByteList) -> TokenRewriterResult {
        let f = &*self.f;
        f(token, input)
    }
}
