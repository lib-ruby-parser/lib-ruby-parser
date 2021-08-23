#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalStringPtr;
#[cfg(feature = "compile-with-external-structures")]
type StringPtr = ExternalStringPtr;
#[cfg(not(feature = "compile-with-external-structures"))]
type StringPtr = String;

use crate::debug_level;
use crate::source::CustomDecoder;
use crate::token_rewriter::TokenRewriter;

pub(crate) struct InternalParserOptions {
    pub(crate) buffer_name: StringPtr,
    pub(crate) debug: debug_level::Type,
    pub(crate) decoder: CustomDecoder,
    pub(crate) token_rewriter: TokenRewriter,
    pub(crate) record_tokens: bool,
}
