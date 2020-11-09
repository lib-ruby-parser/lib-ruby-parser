use crate::source::CustomDecoder;

pub struct ParserOptions {
    pub buffer_name: String,
    pub debug: bool,
    pub decoder: Option<CustomDecoder>,
}

const DEFAULT_BUFFER_NAME: &str = "(eval)";

impl Default for ParserOptions {
    fn default() -> Self {
        Self {
            buffer_name: DEFAULT_BUFFER_NAME.to_owned(),
            debug: false,
            decoder: None,
        }
    }
}
