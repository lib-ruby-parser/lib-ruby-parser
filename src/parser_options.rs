pub struct ParserOptions<'a> {
    pub buffer_name: &'a str,
    pub debug: bool,
    pub known_encoding: Option<String>,
}

const DEFAULT_BUFFER_NAME: &'static str = "(eval)";

impl<'a> Default for ParserOptions<'a> {
    fn default() -> Self {
        Self {
            buffer_name: DEFAULT_BUFFER_NAME,
            debug: false,
            known_encoding: None,
        }
    }
}
