use lib_ruby_parser::{
    source::buffer::*, source::Range, Diagnostic, DiagnosticMessage, ErrorLevel,
};

#[test]
fn it_renders() {
    let source = "line 1\nvery long line 2\n";
    let buffer = Buffer::new("(test_render)", source.as_bytes().to_vec(), None);

    let error = Diagnostic::new(
        ErrorLevel::Warning,
        DiagnosticMessage::FractionAfterNumeric,
        Range::new(8, 12),
    );

    assert_eq!(
        error
            .render(&buffer.input)
            .expect("failed to render diagnostic"),
        vec![
            "(test_render):2:1: warning: unexpected fraction part after numeric literal",
            "(test_render):2: very long line 2",
            "(test_render):2:  ^~~~"
        ]
        .join("\n")
    );
}
