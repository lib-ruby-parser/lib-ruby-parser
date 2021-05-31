use lib_ruby_parser::{source::DecodedInput, Diagnostic, DiagnosticMessage, ErrorLevel, Loc};

#[cfg(feature = "compile-with-external-structures")]
use lib_ruby_parser::containers::ExternalList;
#[cfg(feature = "compile-with-external-structures")]
type List<T> = ExternalList<T>;
#[cfg(not(feature = "compile-with-external-structures"))]
type List<T> = Vec<T>;

#[test]
fn it_renders() {
    let source = "line 1\nvery long line 2\n";
    let mut input = DecodedInput::new("(test_render)");
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
