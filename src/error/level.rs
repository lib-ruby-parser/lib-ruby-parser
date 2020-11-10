#[derive(Clone, PartialEq)]
/// Error level of the diagnostic message
pub enum ErrorLevel {
    Warning,
    Error,
}

impl std::fmt::Debug for ErrorLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Warning => f.write_str("warning"),
            Self::Error => f.write_str("error"),
        }
    }
}
