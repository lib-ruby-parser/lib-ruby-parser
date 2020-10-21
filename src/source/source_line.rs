#[derive(Debug, Clone, Default)]
pub struct SourceLine {
    pub start: usize,
    pub end: usize,
}

impl SourceLine {
    pub fn source(&self, source: &Vec<u8>) -> String {
        String::from_utf8_lossy(&source[self.start..self.end]).into_owned()
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}
