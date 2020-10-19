use crate::source::buffer::*;
use crate::source::FileLoc;
use std::convert::TryInto;

#[derive(Clone, PartialEq)]
pub struct Range {
    begin_pos: usize,
    end_pos: usize,
}

impl Range {
    pub fn new(begin_pos: usize, end_pos: usize) -> Self {
        debug_assert!(end_pos >= begin_pos);
        Self { begin_pos, end_pos }
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
        Self::new(begin_pos, self.end_pos)
    }

    pub fn with_end(&self, end_pos: usize) -> Self {
        Self::new(self.begin_pos, end_pos)
    }

    pub fn adjust_begin(&self, d: i32) -> Self {
        let begin_pos: i32 = self.begin_pos.try_into().unwrap();
        let begin_pos: usize = (begin_pos + d).try_into().unwrap();
        Self::new(begin_pos, self.end_pos)
    }

    pub fn adjust_end(&self, d: i32) -> Self {
        let end_pos: i32 = self.end_pos.try_into().unwrap();
        let end_pos: usize = (end_pos + d).try_into().unwrap();
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

    pub fn begin_loc(&self, buffer: &Buffer) -> Option<FileLoc> {
        FileLoc::from_pos(self.begin_pos, &buffer)
    }

    pub fn end_loc(&self, buffer: &Buffer) -> Option<FileLoc> {
        FileLoc::from_pos(self.end_pos, &buffer)
    }

    pub fn to_locs(&self, buffer: &Buffer) -> Option<(FileLoc, FileLoc)> {
        Some((self.begin_loc(buffer)?, self.end_loc(buffer)?))
    }

    pub fn source(&self, buffer: &Buffer) -> Option<String> {
        buffer
            .substr_at(self.begin_pos, self.end_pos)
            .map(|e| e.to_owned())
    }

    pub fn begin_pos(&self) -> usize {
        self.begin_pos
    }

    pub fn end_pos(&self) -> usize {
        self.end_pos
    }
}

impl std::fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}..{}", self.begin_pos, self.end_pos))
    }
}
