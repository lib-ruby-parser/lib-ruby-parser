use crate::source::Decoder;

/// Configuration of the parser
#[derive(Debug)]
#[repr(C)]
pub struct ParserOptions<'b> {
    /// Name of the buffer. Used in all diagnostic messages
    pub buffer_name: &'b str,

    /// Custom decoder that can be used if the source is encoded
    /// in unknown encoding. Only UTF-8 and ASCII-8BIT/BINARY are
    /// supported out of the box.
    ///
    /// # Example
    /// ```rust
    /// use lib_ruby_parser::source::{Decoder, DecoderResult, InputError};
    /// use lib_ruby_parser::{Parser, ParserOptions, ParserResult, LocExt};
    ///
    /// fn decode(encoding: String, input: Vec<u8>) -> DecoderResult {
    ///     if "US-ASCII" == encoding.to_uppercase() {
    ///         // reencode and return Ok(result)
    ///         return DecoderResult::Ok(b"# encoding: us-ascii\ndecoded".to_vec().into());
    ///     }
    ///     DecoderResult::Err(InputError::DecodingError(
    ///         "only us-ascii is supported".into(),
    ///     ))
    /// }
    ///
    /// let decoder = Decoder::new(Box::new(decode));
    /// let options = ParserOptions {
    ///     decoder: Some(decoder),
    ///     ..Default::default()
    /// };
    /// let parser = Parser::new(b"# encoding: us-ascii\n3 + 3".to_vec(), options);
    /// let ParserResult { ast, input, .. } = parser.do_parse();
    ///
    /// assert_eq!(
    ///     ast.unwrap().expression().source(&input).unwrap(),
    ///     "decoded".to_string()
    /// )
    /// ```
    pub decoder: Option<Decoder<'b>>,

    /// When set to true Parser records tokens.
    /// When set to false `ParserResult.tokens` is guaranteed to be empty.
    /// If you don't need tokens better set it to false to speed up parsing.
    pub record_tokens: bool,
}

const DEFAULT_BUFFER_NAME: &str = "(eval)";

impl Default for ParserOptions<'_> {
    fn default() -> Self {
        Self {
            buffer_name: DEFAULT_BUFFER_NAME,
            decoder: None,
            record_tokens: true,
        }
    }
}
