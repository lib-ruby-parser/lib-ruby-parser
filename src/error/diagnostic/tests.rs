use super::Diagnostic;
use crate::{DiagnosticMessage, ErrorLevel, Loc};

fn make_diagnostic() -> Diagnostic {
    Diagnostic::new(
        ErrorLevel::error(),
        DiagnosticMessage::new_alias_nth_ref(),
        Loc::new(1, 2),
    )
}

#[test]
fn test_new() {
    let diagnostic = make_diagnostic();
    drop(diagnostic)
}

#[test]
fn test_get_level() {
    assert!(make_diagnostic().level().is_error())
}

#[test]
fn test_get_message() {
    assert!(make_diagnostic().message().is_alias_nth_ref())
}

#[test]
fn test_get_loc() {
    assert_eq!(make_diagnostic().loc().begin(), 1);
    assert_eq!(make_diagnostic().loc().end(), 2)
}
