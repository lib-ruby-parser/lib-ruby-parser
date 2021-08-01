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
        let blob =
            unsafe { lib_ruby_parser__internal__containers__loc__make(begin as u64, end as u64) };
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

impl Copy for Loc {}
