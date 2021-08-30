crate::use_native_or_external!(StringPtr);

use crate::debug_level;
use crate::source::maybe_token_rewriter::MaybeTokenRewriter;
use crate::source::MaybeDecoder;

#[repr(C)]
pub(crate) struct InternalParserOptions {
    pub(crate) buffer_name: StringPtr,
    pub(crate) debug: debug_level::Type,
    pub(crate) decoder: MaybeDecoder,
    pub(crate) token_rewriter: MaybeTokenRewriter,
    pub(crate) record_tokens: bool,
}
