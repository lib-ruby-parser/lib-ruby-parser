use std::convert::TryInto;
use crate::source::buffer::*;
use crate::source::FileLoc;

#[derive(Clone, PartialEq)]
pub struct Range {
    pub begin_pos: usize,
    pub end_pos: usize
}

impl Range {
    pub fn new(begin_pos: usize, end_pos: usize) -> Self {
        Self { begin_pos, end_pos }
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

    pub fn to_a(&self) -> &[usize] {
        unimplemented!()
    }

    pub fn to_range(&self) -> std::ops::Range<usize> {
        self.begin_pos..self.end_pos
    }

    pub fn with(&self, begin_pos: usize, end_pos: usize) -> Self {
        Self { begin_pos, end_pos }
    }

    pub fn adjust(&self, begin_pos_d: i32, end_pos_d: i32) -> Self {
        let begin_pos: i32 = self.begin_pos.try_into().unwrap();
        let end_pos: i32 = self.end_pos.try_into().unwrap();

        let begin_pos: usize = (begin_pos + begin_pos_d).try_into().unwrap();
        let end_pos: usize = (end_pos + end_pos_d).try_into().unwrap();

        Self { begin_pos, end_pos }
    }

    pub fn resize(&self, new_size: usize) -> Self {
        self.with(self.begin_pos, self.begin_pos + new_size)
    }

    pub fn join(&self, other: &Self) -> Self {
        Self {
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

    pub fn to_locs(&self, buffer: &Buffer) -> Option<(FileLoc, FileLoc)> {
        let begin_loc = FileLoc::from_pos(self.begin_pos, &buffer)?;
        let end_loc   = FileLoc::from_pos(self.end_pos, &buffer)?;

        Some((begin_loc, end_loc))
    }
}

impl std::fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}..{}", self.begin_pos, self.end_pos))
    }
}
