use super::ParserOptions;

const DEFAULT_BUFFER_NAME: &str = "(eval)";

impl Default for ParserOptions {
    fn default() -> Self {
        Self::new(DEFAULT_BUFFER_NAME.to_string().into(), None, None, true)
    }
}
