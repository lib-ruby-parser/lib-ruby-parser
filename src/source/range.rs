use crate::source::buffer::Input;
use std::convert::TryInto;

#[derive(Clone)]
pub struct Range {
    pub begin_pos: usize,
    pub end_pos: usize,
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.begin_pos == other.begin_pos && self.end_pos == other.end_pos
    }
}

impl Range {
    pub fn new(begin_pos: usize, end_pos: usize) -> Self {
        Self { begin_pos, end_pos }
    }

    pub fn validate(&self, input: &Input) {
        if cfg!(debug_assertions) {
            if input.bytes.last().unwrap() != &100 {
                panic!("creating a wrong range")
            }
            debug_assert!(
                (self.begin_pos < self.end_pos)
                    || (self.begin_pos == self.end_pos && self.end_pos == input.len()),
                "begin_pos = {}, end_pos = {}, input.len() = {}",
                self.begin_pos,
                self.end_pos,
                input.len()
            );
            debug_assert!(
                self.end_pos <= input.len(),
                "end_pos = {}, len = {}",
                self.end_pos,
                input.len()
            );
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

    pub fn to_range(&self) -> std::ops::Range<usize> {
        self.begin_pos..self.end_pos
    }

    pub fn with_begin(&self, begin_pos: usize) -> Self {
        Self::new(begin_pos, self.end_pos)
    }

    pub fn with_end(&self, end_pos: usize) -> Self {
        Self::new(self.begin_pos, end_pos)
    }

    pub fn with(&self, begin_pos: usize, end_pos: usize) -> Self {
        Self::new(begin_pos, end_pos)
    }

    pub fn adjust_begin(&self, d: i32) -> Self {
        let begin_pos: i32 = self
            .begin_pos
            .try_into()
            .expect("failed to convert location to i32 (is it too big?)");
        let begin_pos: usize = (begin_pos + d)
            .try_into()
            .expect("failed to convert location to usize (is it negative?)");
        Self::new(begin_pos, self.end_pos)
    }

    pub fn adjust_end(&self, d: i32) -> Self {
        let end_pos: i32 = self
            .end_pos
            .try_into()
            .expect("failed to convert location to i32 (is it too big?)");
        let end_pos: usize = (end_pos + d)
            .try_into()
            .expect("failed to convert location to usize (is it negative?)");
        Self::new(self.begin_pos, end_pos)
    }

    pub fn resize(&self, new_size: usize) -> Self {
        self.with_end(self.begin_pos + new_size)
    }

    pub fn join(&self, other: &Self) -> Self {
        Self::new(
            std::cmp::min(self.begin_pos, other.begin_pos),
            std::cmp::max(self.end_pos, other.end_pos),
        )
    }

    pub(crate) fn maybe_join(&self, other: &Option<Self>) -> Self {
        match other {
            Some(other) => self.join(other),
            None => self.clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.begin_pos == self.end_pos
    }

    pub fn begin_line_col(&self, input: &Input) -> Option<(usize, usize)> {
        input.line_col_for_pos(self.begin_pos)
    }

    pub fn end_line_col(&self, input: &Input) -> Option<(usize, usize)> {
        input.line_col_for_pos(self.end_pos)
    }

    pub fn expand_to_line(&self, input: &Input) -> Option<(usize, Self)> {
        let (begin_line, _) = self.begin_line_col(input)?;
        let line_no = begin_line;
        let line = &input.lines[line_no];
        Some((line_no, self.with(line.start, line.line_end())))
    }

    pub fn source(&self, input: &Input) -> Option<String> {
        let bytes = input.substr_at(self.begin_pos, self.end_pos)?;
        Some(String::from_utf8_lossy(bytes).into_owned())
    }

    pub(crate) fn print(&self, name: &str) {
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
        f.write_str(&format!("{}...{}", self.begin_pos, self.end_pos))
    }
}
