use crate::blobs::{Blob, HasBlob};

/// Byte sequence based on external implementation
#[repr(C)]
pub struct Bytes {
    pub(crate) blob: Blob<Bytes>,
}

use crate::containers::ExternalList;
type ByteList = ExternalList<u8>;

extern "C" {
    fn lib_ruby_parser__external__bytes__new(list_blob: Blob<ByteList>) -> Blob<Bytes>;
    fn lib_ruby_parser__external__bytes__drop(blob: *mut Blob<Bytes>);
    fn lib_ruby_parser__external__bytes__get_raw(blob: *const Blob<Bytes>)
        -> *const Blob<ByteList>;
    fn lib_ruby_parser__external__bytes__set_raw(blob: *mut Blob<Bytes>, list_blob: Blob<ByteList>);
    fn lib_ruby_parser__external__bytes__into_raw(blob: Blob<Bytes>) -> Blob<ByteList>;
    fn lib_ruby_parser__external__bytes__push(blob: *mut Blob<Bytes>, byte: u8);
}

impl Drop for Bytes {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__external__bytes__drop(&mut self.blob) }
    }
}

impl Default for Bytes {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl Eq for Bytes {}

impl PartialEq for Bytes {
    fn eq(&self, other: &Self) -> bool {
        self.as_raw() == other.as_raw()
    }
}

impl std::fmt::Debug for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bytes")
            .field("raw", &self.as_raw())
            .finish()
    }
}

impl Clone for Bytes {
    fn clone(&self) -> Self {
        Self::new(self.as_raw().to_vec())
    }
}

impl Bytes {
    /// Constructs Bytes based on a given vector
    pub fn new(raw: Vec<u8>) -> Self {
        let list: ByteList = raw.into();
        let blob = unsafe { lib_ruby_parser__external__bytes__new(list.into_blob()) };
        Self { blob }
    }

    /// Returns a reference to inner data
    pub fn as_raw(&self) -> &[u8] {
        unsafe {
            (lib_ruby_parser__external__bytes__get_raw(&self.blob) as *const ByteList)
                .as_ref()
                .unwrap()
        }
    }

    /// "Unwraps" self and returns inner data
    pub fn into_raw(self) -> ByteList {
        let list_blob = unsafe { lib_ruby_parser__external__bytes__into_raw(self.blob) };
        std::mem::forget(self);
        ByteList::from_blob(list_blob)
    }

    /// Replaces inner data with given list
    pub fn set_raw(&mut self, raw: ByteList) {
        unsafe { lib_ruby_parser__external__bytes__set_raw(&mut self.blob, raw.into_blob()) }
    }

    /// Appends a byte
    pub fn push(&mut self, byte: u8) {
        unsafe { lib_ruby_parser__external__bytes__push(&mut self.blob, byte) };
    }
}

impl std::ops::Index<usize> for Bytes {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        self.as_raw().index(index)
    }
}
