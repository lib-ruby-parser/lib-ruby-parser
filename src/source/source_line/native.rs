#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
/// Representation of a source line in a source file
pub struct SourceLine {
    /// Start of the line (in bytes)
    pub start: usize,

    /// End of the line (in bytes)
    pub end: usize,

    /// `true` if line ends with EOF char (which is true for the last line in the file)
    pub ends_with_eof: bool,
}

impl SourceLine {
    /// Constructs a SourceLine
    pub fn new(start: usize, end: usize, ends_with_eof: bool) -> Self {
        Self {
            start,
            end,
            ends_with_eof,
        }
    }

    /// Returns start of the line
    pub fn start(&self) -> usize {
        self.start
    }
    /// Sets start of the line
    pub fn set_start(&mut self, start: usize) {
        self.start = start
    }
    /// Returns end of the line
    pub fn end(&self) -> usize {
        self.end
    }
    /// Sets end of the line
    pub fn set_end(&mut self, end: usize) {
        self.end = end
    }
    /// Returns true of line ends with EOF
    pub fn ends_with_eof(&self) -> bool {
        self.ends_with_eof
    }
    /// Sets whether line ends with EOF
    pub fn set_ends_with_eof(&mut self, ends_with_eof: bool) {
        self.ends_with_eof = ends_with_eof
    }
}
