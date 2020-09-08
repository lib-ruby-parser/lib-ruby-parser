use crate::source::Buffer;

#[derive(Debug)]
pub struct Range<'a> {
    pub source_buffer: &'a Buffer,
    pub begin_pos: usize,
    pub end_pos: usize
}

impl<'a> Range<'a> {
    pub fn new(source_buffer: &'a Buffer, begin_pos: usize, end_pos: usize) -> Self {
        Self { source_buffer, begin_pos, end_pos }
    }

    pub fn begin(&self) -> Self {
        self.with(self.begin_pos, self.begin_pos)
    }

    pub fn end(&self) -> Self {
        self.with(self.end_pos, self.end_pos)
    }

    pub fn size(&self) -> usize {
        self.end_pos - self.begin_pos
    }

    pub fn line(&self) -> Option<usize> {
        self.source_buffer.line_for_position(self.begin_pos)
    }
    pub fn first_line(&self) -> Option<usize> { self.line() }

    pub fn column(&self) -> Option<usize> {
        self.source_buffer.column_for_position(self.begin_pos)
    }

    pub fn last_line(&self) -> Option<usize> {
        self.source_buffer.line_for_position(self.end_pos)
    }

    pub fn last_column(&self) -> Option<usize> {
        self.source_buffer.column_for_position(self.end_pos)
    }

    pub fn column_range(&self) -> Option<std::ops::Range<usize>> {
        if self.begin().line() != self.end().line() {
            return None
        }

        match (self.begin().column(), self.end().column()) {
            (Some(begin_col), Some(end_col)) => Some(begin_col..end_col),
            (_, _) => None
        }
    }

    pub fn source_line(&self) -> Option<&String> {
        if let Some(line) = self.line() {
            Some(self.source_buffer.source_line(line))
        } else {
            None
        }
    }

    pub fn source(&self) -> Option<String> {
        self.source_buffer.slice(self.begin_pos..self.end_pos).map(|chars| chars.to_vec().iter().collect::<String>() )
    }

    pub fn is(&self, what: &[&'static str]) -> bool {
        if let Some(source) = self.source() {
            what.contains(&&source[..])
        } else {
            false
        }
    }

    pub fn to_a(&self) -> &[usize] {
        unimplemented!()
    }

    pub fn to_range(&self) -> std::ops::Range<usize> {
        self.begin_pos..self.end_pos
    }

    pub fn to_s(&self) -> String {
        if let Some((line, column)) = self.source_buffer.decompose_position(self.begin_pos) {
            format!("{}:{}:{}", self.source_buffer.name(), line, column)
        } else {
            "<invalid line:col>".into()
        }
    }

    pub fn with(&self, begin_pos: usize, end_pos: usize) -> Self {
        Self { source_buffer: self.source_buffer.clone(), begin_pos, end_pos }
    }

    pub fn adjust(&self, begin_pos: usize, end_pos: usize) -> Self {
        Self {
            source_buffer: self.source_buffer.clone(),
            begin_pos: self.begin_pos + begin_pos,
            end_pos: self.end_pos + end_pos
        }
    }

    pub fn resize(&self, new_size: usize) -> Self {
        self.with(self.begin_pos, self.begin_pos + new_size)
    }

    pub fn join(&self, other: &Self) -> Self {
        Self {
            source_buffer: self.source_buffer.clone(),
            begin_pos: std::cmp::min(self.begin_pos, other.begin_pos),
            end_pos: std::cmp::max(self.end_pos, other.end_pos)
        }
    }

    pub fn intersect(&self, _other: &Self) -> Self {
        unimplemented!()
    }

    pub fn overlaps(&self, _other: &Self) -> bool {
        unimplemented!()
    }

    pub fn contains(&self, _other: &Self) -> bool {
        unimplemented!()
    }

    pub fn contained(&self, _other: &Self) -> bool {
        unimplemented!()
    }

    pub fn crossing(&self, _other: &Self) -> bool {
        unimplemented!()
    }

    pub fn is_empty(&self) -> bool {
        self.begin_pos == self.end_pos
    }

    pub fn cmp(&self, _other: &Self) -> i8 {
        unimplemented!()
    }

    pub fn inspect(&self) -> String {
        format!("#<Parser::Source::Range {} {}...{}>", self.source_buffer.name(), self.begin_pos, self.end_pos)
    }
}
