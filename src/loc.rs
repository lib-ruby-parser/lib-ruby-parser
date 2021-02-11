use crate::source::buffer::Input;
use crate::Loc;
use std::convert::TryInto;

impl Loc {
    pub fn new(begin: usize, end: usize) -> Self {
        Self { begin, end }
    }

    pub fn validate(&self, input: &Input) {
        if cfg!(debug_assertions) {
            if input.bytes.last().unwrap() != &100 {
                panic!("creating a wrong loc")
            }
            debug_assert!(
                (self.begin < self.end) || (self.begin == self.end && self.end == input.len()),
                "begin = {}, end = {}, input.len() = {}",
                self.begin,
                self.end,
                input.len()
            );
            debug_assert!(
                self.end <= input.len(),
                "end = {}, len = {}",
                self.end,
                input.len()
            );
        }
    }

    pub fn begin(&self) -> Self {
        self.with_begin(self.begin).with_end(self.begin)
    }

    pub fn end(&self) -> Self {
        self.with_begin(self.end).with_end(self.end)
    }

    pub fn size(&self) -> usize {
        self.end - self.begin
    }

    pub fn with_begin(&self, begin: usize) -> Self {
        Self::new(begin, self.end)
    }

    pub fn with_end(&self, end: usize) -> Self {
        Self::new(self.begin, end)
    }

    pub fn with(&self, begin: usize, end: usize) -> Self {
        Self::new(begin, end)
    }

    pub fn adjust_begin(&self, d: i32) -> Self {
        let begin: i32 = self
            .begin
            .try_into()
            .expect("failed to convert location to i32 (is it too big?)");
        let begin: usize = (begin + d)
            .try_into()
            .expect("failed to convert location to usize (is it negative?)");
        Self::new(begin, self.end)
    }

    pub fn adjust_end(&self, d: i32) -> Self {
        let end: i32 = self
            .end
            .try_into()
            .expect("failed to convert location to i32 (is it too big?)");
        let end: usize = (end + d)
            .try_into()
            .expect("failed to convert location to usize (is it negative?)");
        Self::new(self.begin, end)
    }

    pub fn resize(&self, new_size: usize) -> Self {
        self.with_end(self.begin + new_size)
    }

    pub fn join(&self, other: &Self) -> Self {
        Self::new(
            std::cmp::min(self.begin, other.begin),
            std::cmp::max(self.end, other.end),
        )
    }

    pub(crate) fn maybe_join(&self, other: &Option<Self>) -> Self {
        match other {
            Some(other) => self.join(other),
            None => self.clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.begin == self.end
    }

    pub fn begin_line_col(&self, input: &Input) -> Option<(usize, usize)> {
        input.line_col_for_pos(self.begin)
    }

    pub fn end_line_col(&self, input: &Input) -> Option<(usize, usize)> {
        input.line_col_for_pos(self.end)
    }

    pub fn expand_to_line(&self, input: &Input) -> Option<(usize, Self)> {
        let (begin_line, _) = self.begin_line_col(input)?;
        let line_no = begin_line;
        let line = &input.lines[line_no];
        Some((line_no, self.with(line.start, line.line_end())))
    }

    pub fn source(&self, input: &Input) -> Option<String> {
        let bytes = input.substr_at(self.begin, self.end)?;
        Some(String::from_utf8_lossy(bytes).into_owned())
    }

    pub(crate) fn print(&self, name: &str) {
        println!(
            "{}{} {}",
            " ".repeat(self.begin),
            "~".repeat(self.size()),
            name
        )
    }
}

impl std::fmt::Debug for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}...{}", self.begin, self.end))
    }
}
