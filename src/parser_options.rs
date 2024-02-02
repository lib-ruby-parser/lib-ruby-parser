use crate::source::token_rewriter::TokenRewriter;
use crate::source::Decoder;

/// Configuration of the parser
#[derive(Debug)]
#[repr(C)]
pub struct ParserOptions {
    /// Name of the buffer. Used in all diagnostic messages
    pub buffer_name: String,

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
    pub decoder: Option<Decoder>,

    /// Optional token rewriter, see TokenRewriter API
    ///
    /// # Example
    /// ```
    /// use lib_ruby_parser::{
    ///     nodes::*,
    ///     source::token_rewriter::*,
    ///     Bytes, Node, Parser, ParserOptions, ParserResult, Token,
    /// };
    /// fn rewrite_foo_to_bar(mut token: Box<Token>, input: &[u8]) -> TokenRewriterResult {
    ///     // simply rewrite all tokens "foo" to "bar"
    ///     if token.to_string_lossy() == "foo" {
    ///         token.token_value = Bytes::new(b"bar".to_vec());
    ///     }
    ///
    ///     // return token + keep it + keep lexer's state
    ///     TokenRewriterResult {
    ///         rewritten_token: token,
    ///         token_action: RewriteAction::Keep,
    ///         lex_state_action: LexStateAction::Keep,
    ///     }
    /// }
    /// let token_rewriter = TokenRewriter::new(Box::new(rewrite_foo_to_bar));
    /// let options = ParserOptions {
    ///     token_rewriter: Some(token_rewriter),
    ///     ..Default::default()
    /// };
    /// let ParserResult { ast, .. } = Parser::new(b"foo = 1".to_vec(), options).do_parse();
    ///
    /// let ast = ast.unwrap();
    ///
    /// let lvar_name = match &*ast {
    ///     Node::Lvasgn(Lvasgn { name, .. }) => name,
    ///     other => panic!("expected lvasgn node, got {:?}", other),
    /// };
    /// assert_eq!(*lvar_name, String::from("bar"));
    /// ```
    pub token_rewriter: Option<TokenRewriter>,

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
            decoder: None,
            token_rewriter: None,
            record_tokens: true,
        }
    }
}
