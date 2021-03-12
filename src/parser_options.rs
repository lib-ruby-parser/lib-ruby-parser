use crate::debug_level;
use crate::source::CustomDecoder;
use crate::token_rewriter::TokenRewriter;

/// Configuration of the parser
#[derive(Debug)]
pub struct ParserOptions {
    /// Name of the buffer. Used in all diagnostic messages
    pub buffer_name: String,

    /// Controls which debug information is printed during parsing
    ///
    /// Can be:
    ///
    /// + lib_ruby_parser::debug_level::None
    /// + lib_ruby_parser::debug_level::Parser
    /// + lib_ruby_parser::debug_level::Lexer
    /// + lib_ruby_parser::debug_level::Buffer
    /// + or a combination of them (like `Lexer | Buffer`, these value is just a bitmask)
    pub debug: debug_level::Type,

    /// Custom decoder that can be used if the source is encoded
    /// in unknown encoding. Only UTF-8 and ASCII-8BIT/BINARY are
    /// supported out of the box.
    ///
    /// # Example
    /// ```rust
    /// use lib_ruby_parser::source::{InputError, CustomDecoder, RustFnBasedCustomDecoder};
    /// use lib_ruby_parser::{Parser, ParserOptions, ParserResult, debug_level};
    ///
    /// fn decode(encoding: &str, input: &[u8]) -> Result<Vec<u8>, InputError> {
    ///     if "US-ASCII" == encoding.to_uppercase() {
    ///         // reencode and return Ok(result)
    ///         return Ok(b"# encoding: us-ascii\ndecoded".to_vec());
    ///     }
    ///     Err(InputError::DecodingError(
    ///         "only us-ascii is supported".to_string(),
    ///     ))
    /// }
    ///
    /// // Or
    /// let decode_closure = |encoding: &str, input: &[u8]| -> Result<Vec<u8>, InputError> {
    ///     if "US-ASCII" == encoding.to_uppercase() {
    ///         // reencode and return Ok(result)
    ///         return Ok(b"# encoding: us-ascii\ndecoded".to_vec());
    ///     }
    ///     Err(InputError::DecodingError(
    ///         "only us-ascii is supported".to_string(),
    ///     ))
    /// };
    ///
    /// let decoder = RustFnBasedCustomDecoder::new(Box::new(decode_closure));
    /// let options = ParserOptions { decoder: Some(Box::new(decoder)), debug: debug_level::PARSER, ..Default::default() };
    /// let mut parser = Parser::new(b"# encoding: us-ascii\n3 + 3", options);
    /// let ParserResult { ast, input, .. } = parser.do_parse();
    ///
    /// assert_eq!(ast.unwrap().expression().source(&input).unwrap(), "decoded".to_string())
    /// ```
    pub decoder: Option<Box<dyn CustomDecoder>>,

    /// Optional token rewriter, see TokenRewriter API
    pub token_rewriter: Option<Box<dyn TokenRewriter>>,

    /// When set to true Parser records tokens.
    /// When set to false `ParserResult.tokens` is guaranteed to be empty.
    /// If you don't need tokens better set it to false to speed up parsing.
    pub record_tokens: bool,
}

const DEFAULT_BUFFER_NAME: &str = "(eval)";

impl Default for ParserOptions {
    fn default() -> Self {
        Self {
            buffer_name: DEFAULT_BUFFER_NAME.to_string(),
            debug: debug_level::NONE,
            decoder: None,
            token_rewriter: None,
            record_tokens: true,
        }
    }
}
