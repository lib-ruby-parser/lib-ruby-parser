use crate::source::Comment;
use crate::source::DecodedInput;
use crate::source::MagicComment;
use crate::Diagnostic;
use crate::Node;
use crate::Token;

/// Combination of all data that `Parser` can give you
#[derive(Debug)]
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

impl ParserResult {
    pub(crate) fn new(
        ast: Option<Box<Node>>,
        tokens: Vec<Token>,
        diagnostics: Vec<Diagnostic>,
        comments: Vec<Comment>,
        magic_comments: Vec<MagicComment>,
        input: DecodedInput,
    ) -> Self {
        Self {
            ast,
            tokens,
            diagnostics,
            comments,
            magic_comments,
            input,
        }
    }

    /// Returns `ast` attribute
    pub fn ast(&self) -> &Option<Box<Node>> {
        &self.ast
    }
    /// Returns `tokens` attribute
    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
    /// Returns `diagnostics` attribute
    pub fn diagnostics(&self) -> &Vec<Diagnostic> {
        &self.diagnostics
    }
    /// Returns `comments` attribute
    pub fn comments(&self) -> &Vec<Comment> {
        &self.comments
    }
    /// Returns `magic_comments` attribute
    pub fn magic_comments(&self) -> &Vec<MagicComment> {
        &self.magic_comments
    }
    /// Returns `input` attribute
    pub fn input(&self) -> &DecodedInput {
        &self.input
    }
}
