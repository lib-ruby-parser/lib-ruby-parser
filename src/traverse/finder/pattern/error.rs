#[derive(Debug)]
pub struct PatternError {
    pub pattern: String,
}

impl std::fmt::Display for PatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PatternError: unsupported pattern {}", self.pattern)
    }
}

impl std::error::Error for PatternError {}
