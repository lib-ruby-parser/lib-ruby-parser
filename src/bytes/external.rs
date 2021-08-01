use crate::containers::size::BYTES_SIZE;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct BytesBlob {
    blob: [u8; BYTES_SIZE],
}

/// Byte sequence based on external implementation
#[repr(C)]
pub struct Bytes {
    pub(crate) blob: BytesBlob,
}

use crate::containers::list::external::{List, ListBlob};

extern "C" {
    fn lib_ruby_parser__internal__containers__bytes__make_from_list_blob(
        list_blob: ListBlob,
    ) -> BytesBlob;
    fn lib_ruby_parser__internal__containers__bytes__free(bytes_blob: BytesBlob);
    fn lib_ruby_parser__internal__containers__bytes__make() -> BytesBlob;
    fn lib_ruby_parser__internal__containers__bytes__to_list_blob(
        bytes_blob: BytesBlob,
    ) -> ListBlob;
}

impl Drop for Bytes {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__internal__containers__bytes__free(self.blob) }
        self.blob = unsafe { lib_ruby_parser__internal__containers__bytes__make() };
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
        let list_blob: ListBlob = list.into();
        let bytes_blob =
            unsafe { lib_ruby_parser__internal__containers__bytes__make_from_list_blob(list_blob) };
        Self { blob: bytes_blob }
    }

    /// Returns a reference to inner data
    pub fn as_raw(&self) -> &[u8] {
        let list_blob =
            unsafe { lib_ruby_parser__internal__containers__bytes__to_list_blob(self.blob) };
        let list: List<u8> = list_blob.into();
        let slice = unsafe { std::slice::from_raw_parts(list.as_ptr(), list.len()) };
        std::mem::forget(list);
        slice
    }

    /// "Unwraps" self and returns inner data
    pub fn into_raw(mut self) -> List<u8> {
        let list_blob =
            unsafe { lib_ruby_parser__internal__containers__bytes__to_list_blob(self.blob) };
        self.blob = unsafe { lib_ruby_parser__internal__containers__bytes__make() };
        list_blob.into()
    }

    /// Replaces inner data with given list
    pub fn set_raw(&mut self, raw: List<u8>) {
        let list_blob =
            unsafe { lib_ruby_parser__internal__containers__bytes__to_list_blob(self.blob) };
        let list: List<u8> = list_blob.into();
        drop(list);

        let list_blob: ListBlob = raw.into();
        self.blob =
            unsafe { lib_ruby_parser__internal__containers__bytes__make_from_list_blob(list_blob) };
    }

    /// Appends a byte
    pub fn push(&mut self, byte: u8) {
        let bytes_blob = self.blob;
        let list_blob =
            unsafe { lib_ruby_parser__internal__containers__bytes__to_list_blob(bytes_blob) };
        let mut list: List<u8> = list_blob.into();
        list.push(byte);
        let list_blob: ListBlob = list.into();
        let bytes_blob =
            unsafe { lib_ruby_parser__internal__containers__bytes__make_from_list_blob(list_blob) };
        self.blob = bytes_blob;
    }
}

impl Bytes {
    pub(crate) fn into_blob(mut self) -> BytesBlob {
        let result = self.blob;
        self.blob = unsafe { lib_ruby_parser__internal__containers__bytes__make() };
        result
    }
}

impl std::ops::Index<usize> for Bytes {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        self.as_raw().index(index)
    }
}
