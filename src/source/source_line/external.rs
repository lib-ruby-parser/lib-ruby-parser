use crate::containers::size::SOURCE_LINE_SIZE;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct SourceLineBlob {
    blob: [u8; SOURCE_LINE_SIZE],
}

/// Representation of a source line in a source file
#[repr(C)]
pub struct SourceLine {
    blob: SourceLineBlob,
}

extern "C" {
    fn lib_ruby_parser__internal__containers__source_line__new(
        start: u64,
        end: u64,
        ends_with_eof: bool,
    ) -> SourceLineBlob;

    fn lib_ruby_parser__internal__containers__source_line__get_start(blob: SourceLineBlob) -> u64;
    fn lib_ruby_parser__internal__containers__source_line__get_end(blob: SourceLineBlob) -> u64;
    fn lib_ruby_parser__internal__containers__source_line__get_ends_with_eof(
        blob: SourceLineBlob,
    ) -> bool;

    fn lib_ruby_parser__internal__containers__source_line__set_start(
        blob: SourceLineBlob,
        start: u64,
    ) -> SourceLineBlob;
    fn lib_ruby_parser__internal__containers__source_line__set_end(
        blob: SourceLineBlob,
        end: u64,
    ) -> SourceLineBlob;
    fn lib_ruby_parser__internal__containers__source_line__set_ends_with_eof(
        blob: SourceLineBlob,
        ends_with_eof: bool,
    ) -> SourceLineBlob;
}

impl SourceLine {
    /// Constructs a SourceLine
    pub fn new(start: usize, end: usize, ends_with_eof: bool) -> Self {
        let blob = unsafe {
            lib_ruby_parser__internal__containers__source_line__new(
                start as u64,
                end as u64,
                ends_with_eof,
            )
        };
        Self { blob }
    }

    /// Returns start of the line
    pub fn start(&self) -> usize {
        unsafe { lib_ruby_parser__internal__containers__source_line__get_start(self.blob) as usize }
    }
    /// Sets start of the line
    pub fn end(&self) -> usize {
        unsafe { lib_ruby_parser__internal__containers__source_line__get_end(self.blob) as usize }
    }
    /// Returns end of the line
    pub fn ends_with_eof(&self) -> bool {
        unsafe { lib_ruby_parser__internal__containers__source_line__get_ends_with_eof(self.blob) }
    }

    /// Sets end of the line
    pub fn set_start(&mut self, start: usize) {
        self.blob = unsafe {
            lib_ruby_parser__internal__containers__source_line__set_start(self.blob, start as u64)
        }
    }
    /// Returns true of line ends with EOF
    pub fn set_end(&mut self, end: usize) {
        self.blob = unsafe {
            lib_ruby_parser__internal__containers__source_line__set_end(self.blob, end as u64)
        }
    }
    /// Sets whether line ends with EOF
    pub fn set_ends_with_eof(&mut self, ends_with_eof: bool) {
        self.blob = unsafe {
            lib_ruby_parser__internal__containers__source_line__set_ends_with_eof(
                self.blob,
                ends_with_eof,
            )
        }
    }
}

impl std::fmt::Debug for SourceLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceLine")
            .field("start", &self.start())
            .field("end", &self.end())
            .field("ends_with_eof", &self.ends_with_eof())
            .finish()
    }
}
impl Clone for SourceLine {
    fn clone(&self) -> Self {
        Self::new(self.start(), self.end(), self.ends_with_eof())
    }
}
impl Default for SourceLine {
    fn default() -> Self {
        Self::new(Default::default(), Default::default(), Default::default())
    }
}
impl PartialEq for SourceLine {
    fn eq(&self, other: &Self) -> bool {
        (self.start() == other.start())
            && (self.end() == other.end())
            && (self.ends_with_eof() == other.ends_with_eof())
    }
}
impl Eq for SourceLine {}
