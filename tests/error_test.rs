use lib_ruby_parser::{
    containers::List, source::Input, Diagnostic, DiagnosticMessage, ErrorLevel, Loc,
};

#[test]
fn it_renders() {
    let source = "line 1\nvery long line 2\n";
    let mut input = Input::new("(test_render)", None);
    input.set_bytes(List::from(source));

    let error = Diagnostic::new(
        ErrorLevel::Warning,
        DiagnosticMessage::FractionAfterNumeric,
        Loc::new(8, 12),
    );

    assert_eq!(
        error.render(&input).expect("failed to render diagnostic"),
        vec![
            "(test_render):2:1: warning: unexpected fraction part after numeric literal",
            "(test_render):2: very long line 2",
            "(test_render):2:  ^~~~"
        ]
        .join("\n")
    );
}
