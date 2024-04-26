use lib_ruby_parser_ast::SingleLinkedIntrusiveList;

use crate::source::Comment;
use crate::source::DecodedInput;
use crate::source::MagicComment;
use crate::Diagnostic;
use crate::Node;
use crate::Token;

/// Combination of all data that `Parser` can give you
#[repr(C)]
pub struct ParserResult<'b> {
    /// Abstract Syntax Tree that was constructed from you code.
    /// Contains `None` if the code gives no AST nodes
    pub ast: Option<&'b Node<'b>>,

    /// List of tokens returned by a Lexer and consumed by a Parser.
    /// Empty unless ParserOptions::record_tokens is set to true.
    pub tokens: &'b SingleLinkedIntrusiveList<'b, Token<'b>>,

    /// List of all diagnostics (errors and warnings) that have been
    /// recorded during lexing and parsing
    pub diagnostics: &'b SingleLinkedIntrusiveList<'b, Diagnostic<'b>>,

    /// List of comments extracted from the source code.
    pub comments: &'b SingleLinkedIntrusiveList<'b, Comment>,

    /// List of magic comments extracted from the source code.
    pub magic_comments: &'b SingleLinkedIntrusiveList<'b, MagicComment>,

    /// Input that was used for parsing.
    ///
    /// Note: this input is not necessary the same byte array that
    /// you passed to Parser::parse. If encoding of the input is
    /// not `UTF-8` or `ASCII-8BIT/BINARY` Parser invokes `decoder`
    /// that usually produces a different sequence of bytes.
    ///
    /// Pass **this** data to `Loc::source`, otherwise you'll get
    /// incorrect source ranges.
    pub input: DecodedInput<'b>,
}

impl core::fmt::Debug for ParserResult<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ParserResult")
            .field("ast", &self.ast)
            .field("tokens", &self.tokens)
            .field("diagnostics", &self.diagnostics)
            .field("comments", &self.comments)
            .field("magic_comments", &self.magic_comments)
            .finish()
    }
}

#[test]
fn test_fmt() {
    let mut mem = [0; 100];
    let blob = lib_ruby_parser_ast::Blob::from(&mut mem);

    let mut tmp = [0; 100];
    let formatted = lib_ruby_parser_ast::write_to(
        &mut tmp,
        format_args!(
            "{:?}",
            ParserResult {
                ast: None,
                tokens: SingleLinkedIntrusiveList::new(&blob),
                diagnostics: SingleLinkedIntrusiveList::new(&blob),
                comments: SingleLinkedIntrusiveList::new(&blob),
                magic_comments: SingleLinkedIntrusiveList::new(&blob),
                input: DecodedInput::new("foo", b"", &blob)
            }
        ),
    )
    .unwrap();

    assert_eq!(
        formatted,
        // All fields except `input`
        "ParserResult { ast: None, tokens: [], diagnostics: [], comments: [], magic_comments: [] }"
    )
}
