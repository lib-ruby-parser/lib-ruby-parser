use super::ParserOptions;
use crate::debug_level;
use crate::source::Decoder;
use crate::token_rewriter::TokenRewriter;

const DEFAULT_BUFFER_NAME: &str = "(eval)";

impl Default for ParserOptions {
    fn default() -> Self {
        Self::new(
            DEFAULT_BUFFER_NAME.to_string().into(),
            debug_level::NONE,
            Decoder::none(),
            TokenRewriter::none(),
            true,
        )
    }
}
