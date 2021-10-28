use crate::source::Comment;
use crate::source::DecodedInput;
use crate::source::MagicComment;
use crate::Diagnostic;
use crate::Node;
use crate::Token;

/// Combination of all data that `Parser` can give you
#[repr(C)]
pub struct ParserResult {
    /// Abstract Syntax Tree that was constructed from you code.
    /// Contains `None` if the code gives no AST nodes
    pub ast: Option<Box<Node>>,

    /// List of tokens returned by a Lexer and consumed by a Parser.
    /// Empty unless ParserOptions::record_tokens is set to true.
    pub tokens: Vec<Token>,

    /// List of all diagnostics (errors and warings) that have been
    /// recorded during lexing and parsing
    pub diagnostics: Vec<Diagnostic>,

    /// List of comments extracted from the source code.
    pub comments: Vec<Comment>,

    /// List of magic comments extracted from the source code.
    pub magic_comments: Vec<MagicComment>,

    /// Input that was used for parsing.
    ///
    /// Note: this input is not necessary the same byte array that
    /// you passed to Parser::parse. If encoding of the input is
    /// not `UTF-8` or `ASCII-8BIT/BINARY` Parser invokes `decoder`
    /// that usually produces a different sequence of bytes.
    ///
    /// Pass **this** data to `Loc::source`, otherwise you'll get
    /// incorrect source ranges.
    pub input: DecodedInput,
}

impl std::fmt::Debug for ParserResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParserResult")
            .field("ast", &self.ast)
            .field("tokens", &self.tokens)
            .field("diagnostics", &self.diagnostics)
            .field("comments", &self.comments)
            .field("magic_comments", &self.magic_comments)
            .finish()
    }
}
