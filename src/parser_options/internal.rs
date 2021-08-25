#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalStringPtr;
#[cfg(feature = "compile-with-external-structures")]
type StringPtr = ExternalStringPtr;
#[cfg(not(feature = "compile-with-external-structures"))]
type StringPtr = String;

use crate::debug_level;
use crate::source::maybe_token_rewriter::MaybeTokenRewriter;
use crate::source::token_rewriter::TokenRewriter;
use crate::source::MaybeDecoder;

pub(crate) struct InternalParserOptions {
    pub(crate) buffer_name: StringPtr,
    pub(crate) debug: debug_level::Type,
    pub(crate) decoder: MaybeDecoder,
    pub(crate) token_rewriter: MaybeTokenRewriter,
    pub(crate) record_tokens: bool,
}
