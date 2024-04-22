/// Error level of the diagnostic message
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorLevel {
    /// Warning level
    Warning,
    /// Error level
    Error,
}

impl core::fmt::Display for ErrorLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
        }
    }
}
