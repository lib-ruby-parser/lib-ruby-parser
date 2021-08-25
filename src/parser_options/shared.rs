use super::ParserOptions;
use crate::debug_level;
use crate::source::maybe_token_rewriter::{MaybeTokenRewriter, MaybeTokenRewriterAPI};
use crate::source::{MaybeDecoder, MaybeDecoderAPI};

const DEFAULT_BUFFER_NAME: &str = "(eval)";

impl Default for ParserOptions {
    fn default() -> Self {
        Self::new(
            DEFAULT_BUFFER_NAME.to_string().into(),
            debug_level::NONE,
            MaybeDecoder::new_none(),
            MaybeTokenRewriter::new_none(),
            true,
        )
    }
}
