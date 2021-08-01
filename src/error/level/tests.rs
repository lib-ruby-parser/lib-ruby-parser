use super::ErrorLevel;

#[test]
fn test_error() {
    let error = ErrorLevel::error();
    drop(error);
}

#[test]
fn test_warning() {
    let warning = ErrorLevel::warning();
    drop(warning);
}

#[test]
fn test_is_error() {
    assert!(ErrorLevel::error().is_error());
    assert!(!ErrorLevel::warning().is_error());
}

#[test]
fn test_is_warning() {
    assert!(ErrorLevel::warning().is_warning());
    assert!(!ErrorLevel::error().is_warning());
}
