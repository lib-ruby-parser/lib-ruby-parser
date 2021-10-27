/// Error level of the diagnostic message
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
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
        .to_string()
    }
}
