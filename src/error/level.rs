#[derive(Clone)]
pub enum ErrorLevel {
    Warning,
    Error,
    Fatal,
}

impl std::fmt::Debug for ErrorLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Warning => f.write_str("warning"),
            Self::Error => f.write_str("error"),
            Self::Fatal => f.write_str("fatal"),
        }
    }
}
