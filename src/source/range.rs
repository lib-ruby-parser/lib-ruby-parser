use crate::source::buffer::Input;
use std::convert::TryInto;
use std::rc::Rc;

#[derive(Clone)]
pub struct Range {
    pub begin_pos: usize,
    pub end_pos: usize,
    pub input: Rc<Input>,
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.begin_pos == other.begin_pos && self.end_pos == other.end_pos
    }
}

impl Range {
    pub fn new(begin_pos: usize, end_pos: usize, input: Rc<Input>) -> Self {
        debug_assert!(end_pos >= begin_pos);
        Self {
            begin_pos,
            end_pos,
            input,
        }
    }

    pub fn begin(&self) -> Self {
        self.with_begin(self.begin_pos).with_end(self.begin_pos)
    }

    pub fn end(&self) -> Self {
        self.with_begin(self.end_pos).with_end(self.end_pos)
    }

    pub fn size(&self) -> usize {
        self.end_pos - self.begin_pos
    }

    pub fn to_a(&self) -> &[usize] {
        unimplemented!()
    }

    pub fn to_range(&self) -> std::ops::Range<usize> {
        self.begin_pos..self.end_pos
    }

    pub fn with_begin(&self, begin_pos: usize) -> Self {
        Self::new(begin_pos, self.end_pos, Rc::clone(&self.input))
    }

    pub fn with_end(&self, end_pos: usize) -> Self {
        Self::new(self.begin_pos, end_pos, Rc::clone(&self.input))
    }

    pub fn with(&self, begin_pos: usize, end_pos: usize) -> Self {
        Self::new(begin_pos, end_pos, Rc::clone(&self.input))
    }

    pub fn adjust_begin(&self, d: i32) -> Self {
        let begin_pos: i32 = self.begin_pos.try_into().unwrap();
        let begin_pos: usize = (begin_pos + d).try_into().unwrap();
        Self::new(begin_pos, self.end_pos, Rc::clone(&self.input))
    }

    pub fn adjust_end(&self, d: i32) -> Self {
        let end_pos: i32 = self.end_pos.try_into().unwrap();
        let end_pos: usize = (end_pos + d).try_into().unwrap();
        Self::new(self.begin_pos, end_pos, Rc::clone(&self.input))
    }

    pub fn resize(&self, new_size: usize) -> Self {
        self.with_end(self.begin_pos + new_size)
    }

    pub fn join(&self, other: &Self) -> Self {
        Self::new(
            std::cmp::min(self.begin_pos, other.begin_pos),
            std::cmp::max(self.end_pos, other.end_pos),
            Rc::clone(&self.input),
        )
    }

    pub(crate) fn maybe_join(&self, other: &Option<Self>) -> Self {
        match other {
            Some(other) => self.join(other),
            None => self.clone(),
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

    pub fn begin_line_col(&self) -> Option<(usize, usize)> {
        self.input.line_col_for_pos(self.begin_pos)
    }

    pub fn end_line_col(&self) -> Option<(usize, usize)> {
        self.input.line_col_for_pos(self.end_pos)
    }

    pub fn expand_to_line(&self) -> Option<(usize, Self)> {
        println!(
            "self.begin_line_col() = {:?}, self.end_line_col() = {:?}",
            self.begin_line_col(),
            self.end_line_col()
        );
        let (begin_line, _) = self.begin_line_col()?;
        let (end_line, _) = self.end_line_col()?;
        if begin_line != end_line {
            unreachable!("multi-line error")
        }
        let line_no = begin_line;
        let line = &self.input.lines[line_no];
        Some((line_no, self.with(line.start, line.end)))
    }

    pub fn source(&self) -> Option<String> {
        let bytes = self.input.substr_at(self.begin_pos, self.end_pos)?;
        Some(String::from_utf8_lossy(bytes).into_owned())
    }

    pub fn print(&self, name: &str) {
        println!(
            "{}{} {}",
            " ".repeat(self.begin_pos),
            "~".repeat(self.size()),
            name
        )
    }
}

impl std::fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}..{}", self.begin_pos, self.end_pos))
    }
}
