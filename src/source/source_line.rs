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
    /// Returns length of the line
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns true if SourceLine is empty (i.e. has `len = 0`)
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns location of the last non-EOF, non-EOL character
    pub fn line_end(&self) -> usize {
        let mut result = self.end;
        if !self.ends_with_eof {
            result -= 1 // exclude trailing \n
        }
        result
    }
}
