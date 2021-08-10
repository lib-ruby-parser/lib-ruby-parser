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

    fn lib_ruby_parser__internal__containers__source_line__get_start(
        blob: *const SourceLineBlob,
    ) -> u64;
    fn lib_ruby_parser__internal__containers__source_line__get_end(
        blob: *const SourceLineBlob,
    ) -> u64;
    fn lib_ruby_parser__internal__containers__source_line__get_ends_with_eof(
        blob: *const SourceLineBlob,
    ) -> bool;

    fn lib_ruby_parser__internal__containers__source_line__set_start(
        blob: *mut SourceLineBlob,
        start: u64,
    );
    fn lib_ruby_parser__internal__containers__source_line__set_end(
        blob: *mut SourceLineBlob,
        end: u64,
    );
    fn lib_ruby_parser__internal__containers__source_line__set_ends_with_eof(
        blob: *mut SourceLineBlob,
        ends_with_eof: bool,
    );

    fn lib_ruby_parser__internal__containers__source_line__drop(blob: *mut SourceLineBlob);
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

    /// Returns `start` attribute
    pub fn start(&self) -> usize {
        unsafe {
            lib_ruby_parser__internal__containers__source_line__get_start(&self.blob) as usize
        }
    }
    /// Returns `end` attribute
    pub fn end(&self) -> usize {
        unsafe { lib_ruby_parser__internal__containers__source_line__get_end(&self.blob) as usize }
    }
    /// Returns `ends_with_eof` attribute
    pub fn ends_with_eof(&self) -> bool {
        unsafe { lib_ruby_parser__internal__containers__source_line__get_ends_with_eof(&self.blob) }
    }

    /// Sets `start` attribute to given value
    pub fn set_start(&mut self, start: usize) {
        unsafe {
            lib_ruby_parser__internal__containers__source_line__set_start(
                &mut self.blob,
                start as u64,
            )
        }
    }
    /// Sets `end` attribute to given value
    pub fn set_end(&mut self, end: usize) {
        unsafe {
            lib_ruby_parser__internal__containers__source_line__set_end(&mut self.blob, end as u64)
        }
    }
    /// Sets `ends_with_eof` attribute to given value
    pub fn set_ends_with_eof(&mut self, ends_with_eof: bool) {
        unsafe {
            lib_ruby_parser__internal__containers__source_line__set_ends_with_eof(
                &mut self.blob,
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

impl Drop for SourceLine {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__internal__containers__source_line__drop(&mut self.blob) };
    }
}
