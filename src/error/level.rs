#[derive(Clone, PartialEq)]
/// Error level of the diagnostic message
pub enum ErrorLevel {
    /// Warning level
    Warning,
    /// Error level
    Error,
}

impl ToString for ErrorLevel {
    fn to_string(&self) -> String {
        match self {
            Self::Warning => "warning",
            Self::Error => "error",
        }
        .into()
    }
}

impl std::fmt::Debug for ErrorLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}
