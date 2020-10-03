use crate::Buffer;

#[derive(Clone)]
pub struct FileLoc {
    filename: String,
    line: usize,
    col: usize
}

impl std::fmt::Debug for FileLoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}:{}", self.filename, self.line, self.col))
    }
}

impl FileLoc {
    pub fn from_pos(pos: usize, buffer: &Buffer) -> Option<Self> {
        let filename = buffer.name.clone();
        let (line, col) = buffer.line_col_for_pos(pos)?;
        Some(Self { filename, line, col })
    }
}
