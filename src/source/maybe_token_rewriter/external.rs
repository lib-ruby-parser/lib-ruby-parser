use crate::containers::size::MAYBE_TOKEN_REWRITER_SIZE;
use crate::containers::IntoBlob;
use crate::source::maybe_token_rewriter::MaybeTokenRewriterAPI;
use crate::source::token_rewriter::{TokenRewriter, TokenRewriterBlob};

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct MaybeTokenRewriterBlob {
    blob: [u8; MAYBE_TOKEN_REWRITER_SIZE],
}

/// Custom token_rewriter, a wrapper around a function
#[repr(C)]
pub struct MaybeTokenRewriter {
    pub(crate) blob: MaybeTokenRewriterBlob,
}

extern "C" {
    fn lib_ruby_parser__external__maybe_token_rewriter__new_some(
        blob: TokenRewriterBlob,
    ) -> MaybeTokenRewriterBlob;
    fn lib_ruby_parser__external__maybe_token_rewriter__new_none() -> MaybeTokenRewriterBlob;
    fn lib_ruby_parser__external__maybe_token_rewriter__drop(blob: *mut MaybeTokenRewriterBlob);
    fn lib_ruby_parser__external__maybe_token_rewriter__is_some(
        blob: *const MaybeTokenRewriterBlob,
    ) -> bool;
    fn lib_ruby_parser__external__maybe_token_rewriter__is_none(
        blob: *const MaybeTokenRewriterBlob,
    ) -> bool;
    fn lib_ruby_parser__external__maybe_token_rewriter__as_token_rewriter(
        blob: *const MaybeTokenRewriterBlob,
    ) -> *const TokenRewriterBlob;
    fn lib_ruby_parser__external__maybe_token_rewriter__into_token_rewriter(
        blob: MaybeTokenRewriterBlob,
    ) -> TokenRewriterBlob;
}

impl Drop for MaybeTokenRewriter {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__external__maybe_token_rewriter__drop(&mut self.blob) }
    }
}

impl MaybeTokenRewriter {}

impl MaybeTokenRewriterAPI for MaybeTokenRewriter {
    fn new_some(token_rewriter: TokenRewriter) -> Self {
        let blob = unsafe {
            lib_ruby_parser__external__maybe_token_rewriter__new_some(token_rewriter.into_blob())
        };
        Self { blob }
    }

    fn new_none() -> Self {
        let blob = unsafe { lib_ruby_parser__external__maybe_token_rewriter__new_none() };
        Self { blob }
    }

    fn is_some(&self) -> bool {
        unsafe { lib_ruby_parser__external__maybe_token_rewriter__is_some(&self.blob) }
    }

    fn is_none(&self) -> bool {
        unsafe { lib_ruby_parser__external__maybe_token_rewriter__is_none(&self.blob) }
    }

    fn as_token_rewriter(&self) -> Option<&TokenRewriter> {
        unsafe {
            (lib_ruby_parser__external__maybe_token_rewriter__as_token_rewriter(&self.blob)
                as *const TokenRewriter)
                .as_ref()
        }
    }

    fn as_token_rewriter_mut(&mut self) -> Option<&mut TokenRewriter> {
        unsafe {
            (lib_ruby_parser__external__maybe_token_rewriter__as_token_rewriter(&mut self.blob)
                as *mut TokenRewriter)
                .as_mut()
        }
    }

    fn into_token_rewriter(self) -> TokenRewriter {
        let token_rewriter = TokenRewriter::from_blob(unsafe {
            lib_ruby_parser__external__maybe_token_rewriter__into_token_rewriter(self.blob)
        });
        std::mem::forget(self);
        token_rewriter
    }
}

impl std::fmt::Debug for MaybeTokenRewriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_token_rewriter())
    }
}

impl Default for MaybeTokenRewriter {
    fn default() -> Self {
        Self::new_none()
    }
}
