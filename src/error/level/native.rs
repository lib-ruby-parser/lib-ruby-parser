/// Error level of the diagnostic message
#[repr(C)]
#[derive(Clone, PartialEq, Eq)]
pub enum ErrorLevel {
    /// Warning level
    Warning,
    /// Error level
    Error,
}

impl ErrorLevel {
    /// Constructs a warning
    pub fn warning() -> Self {
        Self::Warning
    }

    /// Constructs an error
    pub fn error() -> Self {
        Self::Error
    }

    /// Returns true if `self` is a warning
    pub fn is_warning(&self) -> bool {
        matches!(self, Self::Warning)
    }

    /// Returns true if `self` is an error
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}
