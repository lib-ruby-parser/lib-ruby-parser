crate::use_native_or_external!(StringPtr);
crate::use_native_or_external!(Maybe);

use crate::debug_level;
use crate::source::token_rewriter::TokenRewriter;
use crate::source::Decoder;

#[repr(C)]
pub(crate) struct InternalParserOptions {
    pub(crate) buffer_name: StringPtr,
    pub(crate) debug: debug_level::Type,
    pub(crate) decoder: Maybe<Decoder>,
    pub(crate) token_rewriter: Maybe<TokenRewriter>,
    pub(crate) record_tokens: bool,
}
