/// An error that is returned when attempting to build
/// a finder::Item from a string that can't be recognized
#[derive(Debug)]
pub struct PatternError {
    /// Pattern that wasn't recognozed
    pub pattern: String,
}

impl std::fmt::Display for PatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PatternError: unsupported pattern {}", self.pattern)
    }
}

impl std::error::Error for PatternError {}
