crate::use_native_or_external!(Maybe);

use super::ParserOptions;

const DEFAULT_BUFFER_NAME: &str = "(eval)";

impl Default for ParserOptions {
    fn default() -> Self {
        Self::new(
            DEFAULT_BUFFER_NAME.to_string().into(),
            Maybe::none(),
            Maybe::none(),
            true,
        )
    }
}
