use crate::source::CustomDecoder;

/// Configuration of the parser
pub struct ParserOptions {
    /// Name of the buffer. Used in all diagnostic messages
    pub buffer_name: String,

    /// Controls whether the parser should run in debug mode
    ///
    /// Debug mode forces parser/lexer to print additional information
    /// while running (like bison actions)
    pub debug: bool,

    /// Custom decoder that can be used if the source is encoded
    /// in unknown encoding. Only UTF-8 and ASCII-8BIT/BINARY are
    /// supported out of the box.
    ///
    /// # Example
    /// ```rust
    /// use lib_ruby_parser::source::{InputError, RecognizedEncoding};
    /// use lib_ruby_parser::{Parser, ParserOptions};
    ///
    /// fn decoder(encoding: RecognizedEncoding, input: Vec<u8>) -> Result<Vec<u8>, InputError> {
    ///     if let RecognizedEncoding::US_ASCII = encoding {
    ///         // reencode and return Ok(result)
    ///         return Ok(b"decoded".to_vec());
    ///     }
    ///     Err(InputError::DecodingError(
    ///         "only us-ascii is supported".to_owned(),
    ///     ))
    /// }
    ///
    /// let options = ParserOptions { decoder: Some(Box::new(decoder)), ..Default::default() };
    /// let mut parser = Parser::new(b"# encoding: us-ascii\n3 + 3", options).unwrap();
    /// let ast = parser.do_parse().ast.unwrap();
    ///
    /// assert_eq!(ast.expression().source().unwrap(), "decoded".to_owned())
    /// ```
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
