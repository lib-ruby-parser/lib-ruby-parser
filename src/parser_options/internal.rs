use crate::source::token_rewriter::TokenRewriter;
use crate::source::Decoder;

#[repr(C)]
pub(crate) struct InternalParserOptions {
    pub(crate) buffer_name: String,
    pub(crate) decoder: Option<Decoder>,
    pub(crate) token_rewriter: Option<TokenRewriter>,
    pub(crate) record_tokens: bool,
}
