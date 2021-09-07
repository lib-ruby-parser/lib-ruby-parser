use crate::blobs::Blob;

/// Byte sequence based on external implementation
#[repr(C)]
pub struct Loc {
    pub(crate) blob: Blob<Loc>,
}

extern "C" {
    fn lib_ruby_parser__external__loc__new(begin: u64, end: u64) -> Blob<Loc>;
    fn lib_ruby_parser__external__loc__drop(blob: *mut Blob<Loc>);
    fn lib_ruby_parser__external__loc__get_begin(blob: *const Blob<Loc>) -> u64;
    fn lib_ruby_parser__external__loc__get_end(blob: *const Blob<Loc>) -> u64;
}

impl Loc {
    /// Constructs a new Loc struct
    pub fn new(begin: usize, end: usize) -> Loc {
        let blob = unsafe { lib_ruby_parser__external__loc__new(begin as u64, end as u64) };
        Self { blob }
    }

    /// Returns `begin` field of the `Loc`
    pub fn begin(&self) -> usize {
        unsafe { lib_ruby_parser__external__loc__get_begin(&self.blob) as usize }
    }

    /// Returns `end` field of the `Loc`
    pub fn end(&self) -> usize {
        unsafe { lib_ruby_parser__external__loc__get_end(&self.blob) as usize }
    }
}

impl Default for Loc {
    fn default() -> Self {
        Self::new(0, 0)
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

impl Drop for Loc {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__external__loc__drop(&mut self.blob) };
    }
}
