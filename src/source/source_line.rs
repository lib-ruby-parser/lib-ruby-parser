#[derive(Debug, Clone, Default)]
pub struct SourceLine {
    pub start: usize,
    pub end: usize,
    pub ends_with_eof: bool,
}

impl SourceLine {
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn line_end(&self) -> usize {
        let mut result = self.end;
        if !self.ends_with_eof {
            result -= 1 // exclude trailing \n
        }
        result
    }
}
