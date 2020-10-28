use ruby_parser::{source::buffer::*, source::Range, Diagnostic, DiagnosticMessage, ErrorLevel};
use std::rc::Rc;

#[test]
fn it_renders() {
    let source = "line 1\nvery long line 2";
    let buffer = Buffer::new("(test_render)", source.as_bytes().to_vec(), None).unwrap();

    let error = Diagnostic::new(
        ErrorLevel::Warning,
        DiagnosticMessage::FractionAfterNumeric,
        Range::new(8, 12, Rc::clone(&buffer.input)),
    );

    assert_eq!(
        error.render().unwrap(),
        vec![
            "(test_render):2:1: warning: unexpected fraction part after numeric literal",
            "(test_render):2: very long line 2",
            "(test_render):2: ^~~"
        ]
        .join("\n")
    );
}
