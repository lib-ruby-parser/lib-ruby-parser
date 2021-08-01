use super::SourceLine;

impl SourceLine {
    /// Returns length of the line
    pub fn len(&self) -> usize {
        self.end() - self.start()
    }

    /// Returns location of the last non-EOF, non-EOL character
    pub fn line_end(&self) -> usize {
        let mut result = self.end();
        if !self.ends_with_eof() {
            result -= 1 // exclude trailing \n
        }
        result
    }
}
