use std::convert::TryInto;

#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalMaybeLoc;
#[cfg(feature = "compile-with-external-structures")]
type MaybeLoc = ExternalMaybeLoc;
#[cfg(not(feature = "compile-with-external-structures"))]
type MaybeLoc = Option<Loc>;

use crate::source::DecodedInput;

#[cfg(not(feature = "compile-with-external-structures"))]
mod loc {
    /// Representation of any location in the given input
    #[repr(C)]
    pub struct Loc {
        /// Begin of the `Loc` range
        pub begin: usize,
        /// End of the `Loc` range
        pub end: usize,
    }

    impl Loc {
        /// Constructs a new Loc struct
        pub fn new(begin: usize, end: usize) -> Loc {
            Self { begin, end }
        }

        /// Returns `begin` field of the `Loc`
        pub fn begin(&self) -> usize {
            self.begin
        }

        /// Returns `end` field of the `Loc`
        pub fn end(&self) -> usize {
            self.end
        }
    }
}

#[cfg(feature = "compile-with-external-structures")]
mod loc {
    use crate::containers::size::LOC_SIZE;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub(crate) struct LocBlob {
        blob: [u8; LOC_SIZE],
    }

    /// Byte sequence based on external implementation
    #[repr(C)]
    pub struct Loc {
        pub(crate) blob: LocBlob,
    }

    extern "C" {
        fn lib_ruby_parser__internal__containers__loc__make(begin: u64, end: u64) -> LocBlob;
        fn lib_ruby_parser__internal__containers__loc__begin(blob: LocBlob) -> u64;
        fn lib_ruby_parser__internal__containers__loc__end(blob: LocBlob) -> u64;
    }

    impl Loc {
        /// Constructs a new Loc struct
        pub fn new(begin: usize, end: usize) -> Loc {
            let blob = unsafe {
                lib_ruby_parser__internal__containers__loc__make(begin as u64, end as u64)
            };
            Self { blob }
        }

        /// Returns `begin` field of the `Loc`
        pub fn begin(&self) -> usize {
            unsafe { lib_ruby_parser__internal__containers__loc__begin(self.blob) as usize }
        }

        /// Returns `end` field of the `Loc`
        pub fn end(&self) -> usize {
            unsafe { lib_ruby_parser__internal__containers__loc__end(self.blob) as usize }
        }
    }
}

pub use loc::Loc;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use loc::LocBlob;

impl Loc {
    /// Converts location to a range
    pub fn to_range(&self) -> std::ops::Range<usize> {
        self.begin()..self.end()
    }
}

impl Default for Loc {
    fn default() -> Self {
        Self::new(0, 0)
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

    pub(crate) fn maybe_join(&self, other: &MaybeLoc) -> Loc {
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

impl Clone for Loc {
    fn clone(&self) -> Self {
        Self::new(self.begin(), self.end())
    }
}

impl PartialEq for Loc {
    fn eq(&self, other: &Self) -> bool {
        (self.begin() == other.begin()) && (self.end() == other.end())
    }
}

impl Eq for Loc {}

impl Copy for Loc {}
