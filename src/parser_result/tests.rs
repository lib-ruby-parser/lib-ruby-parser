use super::ParserResult;
use crate::source::{CommentType, MagicCommentKind, SourceLine};
use crate::{
    nodes::Retry, source::Comment, source::DecodedInput, source::MagicComment, Bytes, Diagnostic,
    DiagnosticMessage, Loc, Node, Token,
};
use crate::{ErrorLevel, LexState};

fn ast() -> Option<Box<Node>> {
    Some(Box::new(Node::Retry(Retry {
        expression_l: Loc { begin: 1, end: 2 },
    })))
}

fn tokens() -> Vec<Token> {
    vec![Token {
        token_type: 280,
        token_value: Bytes::new(vec![97, 98, 99]),
        loc: Loc { begin: 3, end: 4 },
        lex_state_before: LexState { value: 1 },
        lex_state_after: LexState { value: 2 },
    }]
}

fn diagnostics() -> Vec<Diagnostic> {
    vec![Diagnostic {
        level: ErrorLevel::Error,
        message: DiagnosticMessage::AliasNthRef {},
        loc: Loc { begin: 5, end: 6 },
    }]
}

fn comments() -> Vec<Comment> {
    vec![Comment {
        location: Loc { begin: 7, end: 8 },
        kind: CommentType::Inline,
    }]
}
fn magic_comments() -> Vec<MagicComment> {
    vec![MagicComment::new(
        MagicCommentKind::warn_indent(),
        Loc { begin: 9, end: 10 },
        Loc { begin: 11, end: 12 },
    )]
}
fn input() -> DecodedInput {
    let mut input = DecodedInput::named("foo");
    input.bytes = vec![1, 2, 3];
    input.lines = vec![SourceLine::new(1, 2, false)];
    input
}

fn parser_options() -> ParserResult {
    ParserResult::new(
        ast(),
        tokens(),
        diagnostics(),
        comments(),
        magic_comments(),
        input(),
    )
}

#[test]
fn test_new() {
    let parser_options = parser_options();
    drop(parser_options);
}

#[test]
fn test_debug() {
    assert_eq!(
        format!("{:?}", parser_options()),
        "ParserResult { \
ast: Some(Retry(Retry { expression_l: 1...2 })), \
tokens: [[kIN, \"abc\", 3...4]], \
diagnostics: [Diagnostic { level: Error, message: AliasNthRef, loc: 5...6 }], \
comments: [Comment { location: 7...8, kind: Inline }], \
magic_comments: [MagicComment { kind: WarnIndent, key_l: 9...10, value_l: 11...12 }], \
input: DecodedInput { name: \"foo\", lines: [SourceLine { start: 1, end: 2, ends_with_eof: false }], bytes: [1, 2, 3] } \
}"
    )
}

#[test]
fn test_getters() {
    let parser_options = parser_options();

    assert_eq!(parser_options.ast(), &ast());
    assert_eq!(parser_options.tokens(), &tokens());
    assert_eq!(parser_options.diagnostics(), &diagnostics());
    assert_eq!(parser_options.comments(), &comments());
    assert_eq!(parser_options.magic_comments(), &magic_comments());
}
