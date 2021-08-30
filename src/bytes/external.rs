use crate::blobs::BytesBlob;
use crate::containers::IntoBlob;

/// Byte sequence based on external implementation
#[repr(C)]
pub struct Bytes {
    pub(crate) blob: BytesBlob,
}

use crate::blobs::ListBlob;
use crate::containers::list::external::List;

extern "C" {
    fn lib_ruby_parser__external__bytes__new(list_blob: ListBlob) -> BytesBlob;
    fn lib_ruby_parser__external__bytes__drop(blob: *mut BytesBlob);
    fn lib_ruby_parser__external__bytes__get_raw(blob: *const BytesBlob) -> *const ListBlob;
    fn lib_ruby_parser__external__bytes__set_raw(blob: *mut BytesBlob, list_blob: ListBlob);
    fn lib_ruby_parser__external__bytes__into_raw(blob: BytesBlob) -> ListBlob;
    fn lib_ruby_parser__external__bytes__push(blob: *mut BytesBlob, byte: u8);
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
        let list: List<u8> = raw.into();
        let blob = unsafe { lib_ruby_parser__external__bytes__new(list.into_blob()) };
        Self { blob }
    }

    /// Returns a reference to inner data
    pub fn as_raw(&self) -> &[u8] {
        unsafe {
            (lib_ruby_parser__external__bytes__get_raw(&self.blob) as *const List<u8>)
                .as_ref()
                .unwrap()
        }
    }

    /// "Unwraps" self and returns inner data
    pub fn into_raw(self) -> List<u8> {
        let list_blob = unsafe { lib_ruby_parser__external__bytes__into_raw(self.blob) };
        std::mem::forget(self);
        List::<u8>::from_blob(list_blob)
    }

    /// Replaces inner data with given list
    pub fn set_raw(&mut self, raw: List<u8>) {
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
