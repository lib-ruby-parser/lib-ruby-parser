use super::ParserResult;
use crate::source::{CommentType, MagicCommentKind, SourceLine};
use crate::{
    source::Comment, source::DecodedInput, source::MagicComment, Bytes, Diagnostic,
    DiagnosticMessage, Loc, Node, Token,
};
use crate::{ErrorLevel, LexState};

crate::use_native_or_external!(MaybePtr);
crate::use_native_or_external!(List);

fn ast() -> MaybePtr<Node> {
    MaybePtr::some(Node::new_retry(Loc::new(1, 2)))
}

fn tokens() -> List<Token> {
    list![Token::new(
        280,
        Bytes::new(list![97, 98, 99]),
        Loc::new(3, 4),
        LexState { value: 1 },
        LexState { value: 2 },
    )]
}

fn diagnostics() -> List<Diagnostic> {
    list![Diagnostic::new(
        ErrorLevel::error(),
        DiagnosticMessage::new_alias_nth_ref(),
        Loc::new(5, 6),
    )]
}

fn comments() -> List<Comment> {
    list![Comment::make(Loc::new(7, 8), CommentType::inline())]
}
fn magic_comments() -> List<MagicComment> {
    list![MagicComment::new(
        MagicCommentKind::warn_indent(),
        Loc::new(9, 10),
        Loc::new(11, 12),
    )]
}
fn input() -> DecodedInput {
    let mut input = DecodedInput::named("foo");
    input.set_bytes(list![1, 2, 3]);
    input.set_lines(list![SourceLine::new(1, 2, false)]);
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
diagnostics: [Diagnostic { level: error, message: AliasNthRef(AliasNthRef), loc: 5...6 }], \
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
