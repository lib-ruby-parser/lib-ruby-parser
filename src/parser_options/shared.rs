use super::ParserOptions;
use crate::debug_level;
use crate::source::MaybeDecoder;
use crate::source::MaybeDecoderAPI;

const DEFAULT_BUFFER_NAME: &str = "(eval)";

impl Default for ParserOptions {
    fn default() -> Self {
        Self::new(
            DEFAULT_BUFFER_NAME.to_string().into(),
            debug_level::NONE,
            MaybeDecoder::new_none(),
            None,
            true,
        )
    }
}
