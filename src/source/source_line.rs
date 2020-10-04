#[derive(Debug, Clone, Default)]
pub struct SourceLine {
    pub start: usize,
    pub end: usize,
}

impl SourceLine {
    pub fn source(&self, source: &Vec<char>) -> Vec<char> {
        source[self.start..self.end].to_owned()
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}
