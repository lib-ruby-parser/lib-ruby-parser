use super::Diagnostic;
use crate::{DiagnosticMessage, ErrorLevel, Loc};

fn new_diagnostic() -> Diagnostic {
    Diagnostic::new(
        ErrorLevel::error(),
        DiagnosticMessage::AliasNthRef {},
        Loc::new(1, 2),
    )
}

#[test]
fn test_new() {
    let diagnostic = new_diagnostic();
    drop(diagnostic)
}

#[test]
fn test_get_level() {
    assert!(new_diagnostic().level().is_error())
}

#[test]
fn test_get_message() {
    assert_eq!(
        new_diagnostic().message(),
        &DiagnosticMessage::AliasNthRef {}
    )
}

#[test]
fn test_get_loc() {
    assert_eq!(new_diagnostic().loc().begin(), 1);
    assert_eq!(new_diagnostic().loc().end(), 2)
}

#[test]
fn test_renders() {
    let source = "line 1\nvery long line 2\n";
    let mut input = crate::source::DecodedInput::named("(test_render)");
    input.update_bytes(Vec::from(source));

    let error = Diagnostic::new(
        ErrorLevel::warning(),
        DiagnosticMessage::FractionAfterNumeric {},
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
