use super::Loc;
use crate::source::DecodedInput;
use std::convert::TryInto;

crate::use_native_or_external!(Maybe);

impl Loc {
    /// Converts location to a range
    pub fn to_range(&self) -> std::ops::Range<usize> {
        self.begin()..self.end()
    }
}

impl Loc {
    /// Returns size of the `Loc` (i.e. `end - begin`)
    pub fn size(&self) -> usize {
        self.end() - self.begin()
    }

    /// Returns a new `Loc` with given `begin` and current `end`
    pub fn with_begin(&self, begin: usize) -> Loc {
        Self::new(begin, self.end())
    }

    /// Returns a new `Loc` with given `end` and current `begin`
    pub fn with_end(&self, end: usize) -> Loc {
        Self::new(self.begin(), end)
    }

    /// Adds given `delta` to `begin`
    pub fn adjust_begin(&self, delta: i32) -> Loc {
        let begin: i32 = self
            .begin()
            .try_into()
            .expect("failed to convert location to i32 (is it too big?)");
        let begin: usize = (begin + delta)
            .try_into()
            .expect("failed to convert location to usize (is it negative?)");
        Self::new(begin, self.end())
    }

    /// Adds given `delta` to `end`
    pub fn adjust_end(&self, d: i32) -> Loc {
        let end: i32 = self
            .end()
            .try_into()
            .expect("failed to convert location to i32 (is it too big?)");
        let end: usize = (end + d)
            .try_into()
            .expect("failed to convert location to usize (is it negative?)");
        Self::new(self.begin(), end)
    }

    /// Returns a new `Loc` with the same `begin`, but adjusted `end`,
    /// so that its size is equal to given `new_size`
    pub fn resize(&self, new_size: usize) -> Loc {
        self.with_end(self.begin() + new_size)
    }

    /// Joins two `Loc`s by choosing `min(begin)` + `max(end)`
    pub fn join(&self, other: &Self) -> Loc {
        Self::new(
            std::cmp::min(self.begin(), other.begin()),
            std::cmp::max(self.end(), other.end()),
        )
    }

    pub(crate) fn maybe_join(&self, other: &Maybe<Loc>) -> Loc {
        match other.as_ref() {
            Some(other) => self.join(other),
            None => self.clone(),
        }
    }

    /// Returns true if `Loc` is empty (i.e. `begin` == `end`)
    pub fn is_empty(&self) -> bool {
        self.begin() == self.end()
    }

    pub(crate) fn begin_line_col(&self, input: &DecodedInput) -> Option<(usize, usize)> {
        input.line_col_for_pos(self.begin())
    }

    #[allow(dead_code)]
    pub(crate) fn end_line_col(&self, input: &DecodedInput) -> Option<(usize, usize)> {
        input.line_col_for_pos(self.end())
    }

    pub(crate) fn expand_to_line(&self, input: &DecodedInput) -> Option<(usize, Loc)> {
        let (begin_line, _) = self.begin_line_col(input)?;
        let line_no = begin_line;
        let line = input.line_at(line_no);
        Some((line_no, Self::new(line.start(), line.line_end())))
    }

    /// Returns source code of the current `Loc` on a given `Input`
    pub fn source(&self, input: &DecodedInput) -> Option<String> {
        let bytes = input.substr_at(self.begin(), self.end())?;
        Some(String::from_utf8_lossy(bytes).into_owned())
    }

    pub(crate) fn print(&self, name: &str) {
        println!(
            "{}{} {}",
            " ".repeat(self.begin()),
            "~".repeat(self.size()),
            name
        )
    }
}

impl std::fmt::Debug for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}...{}", self.begin(), self.end()))
    }
}
