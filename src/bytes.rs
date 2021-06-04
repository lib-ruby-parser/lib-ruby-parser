#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalList;
#[cfg(feature = "compile-with-external-structures")]
type List<T> = ExternalList<T>;
#[cfg(not(feature = "compile-with-external-structures"))]
type List<T> = Vec<T>;

/// Trait with common methods of Bytes (Rust- or external-based)
pub trait BytesTrait:
    std::ops::Index<usize> + Default + std::fmt::Debug + Clone + PartialEq + Eq
{
    /// Constructs Bytes based on a given vector
    fn new(raw: Vec<u8>) -> Self;

    /// Returns a reference to inner data
    fn as_raw(&self) -> &[u8];
    /// "Unwraps" self and returns inner data
    fn into_raw(self) -> List<u8>
    where
        Self: Sized;
    /// Replaces inner data with given list
    fn set_raw(&mut self, raw: List<u8>);

    /// Constructs an empty instance of `Bytes`
    fn empty() -> Self
    where
        Self: Sized,
    {
        Self::new(vec![])
    }

    /// Converts byte sequence to a string slice, returns error if there are invalid UTF-8 chars
    fn as_str_lossy(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.as_raw())
    }

    /// Converts byte sequnce to a string, all invalid UTF-8 chars are converted into "replacement char"
    fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(self.as_raw()).into_owned()
    }

    /// Converts byte sequence to a String, returns error if there are invalid UTF-8 chars
    fn to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.as_raw().to_vec())
    }

    /// Consumes itself and convrters it into a string, returns error if there are invalid UTF-8 chars
    fn into_string(self) -> Result<String, std::string::FromUtf8Error>
    where
        Self: Sized,
    {
        String::from_utf8(self.into_raw().into())
    }

    /// Returns `true` if `self` represents a valid UTF-8 string
    fn is_valid_utf8(&self) -> bool {
        std::str::from_utf8(self.as_raw()).is_ok()
    }

    /// Returns `true` if byte sequence is empty
    fn is_empty(&self) -> bool {
        self.as_raw().is_empty()
    }

    /// Returns length of the byte sequence
    fn len(&self) -> usize {
        self.as_raw().len()
    }

    /// Clears inner data
    fn clear(&mut self) {
        self.set_raw(List::<u8>::new())
    }

    /// Appends a byte
    fn push(&mut self, byte: u8);
}

#[cfg(not(feature = "compile-with-external-structures"))]
mod bytes {
    use super::{BytesTrait, List};

    /// Representation of a byte sequence, see [`BytesTrait`] for available methods
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[repr(C)]
    pub struct Bytes {
        /// Raw vector of bytes
        pub raw: List<u8>,
    }

    impl Default for Bytes {
        fn default() -> Self {
            Self::new(vec![])
        }
    }

    impl BytesTrait for Bytes {
        fn new(raw: Vec<u8>) -> Self {
            Self { raw: raw.into() }
        }

        fn as_raw(&self) -> &[u8] {
            &self.raw
        }

        fn into_raw(self) -> List<u8> {
            self.raw
        }

        fn set_raw(&mut self, raw: List<u8>) {
            self.raw = raw
        }

        fn push(&mut self, item: u8) {
            self.raw.push(item);
        }
    }

    impl std::ops::Index<usize> for Bytes {
        type Output = u8;

        fn index(&self, index: usize) -> &Self::Output {
            self.raw.index(index)
        }
    }
}

#[cfg(feature = "compile-with-external-structures")]
mod bytes {
    use super::BytesTrait;
    use crate::containers::size::BYTES_SIZE;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct BytesBlob {
        blob: [u8; BYTES_SIZE],
    }

    /// Byte sequence based on external implementation
    #[repr(C)]
    pub struct Bytes {
        blob: BytesBlob,
    }

    use crate::containers::list::external::{List, ListBlob};

    extern "C" {
        fn lib_ruby_parser_bytes_blob_from_list_blob(list_blob: ListBlob) -> BytesBlob;
        fn lib_ruby_parser_bytes_blob_free(bytes_blob: BytesBlob);
        fn lib_ruby_parser_bytes_blob_new() -> BytesBlob;
        fn lib_ruby_parser_list_blob_from_bytes_blob(bytes_blob: BytesBlob) -> ListBlob;
    }

    impl Drop for Bytes {
        fn drop(&mut self) {
            unsafe { lib_ruby_parser_bytes_blob_free(self.blob) }
            self.blob = unsafe { lib_ruby_parser_bytes_blob_new() };
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

    impl BytesTrait for Bytes {
        fn new(raw: Vec<u8>) -> Self {
            let list: List<u8> = raw.into();
            let list_blob: ListBlob = list.into();
            let bytes_blob = unsafe { lib_ruby_parser_bytes_blob_from_list_blob(list_blob) };
            Self { blob: bytes_blob }
        }

        fn as_raw(&self) -> &[u8] {
            let list_blob = unsafe { lib_ruby_parser_list_blob_from_bytes_blob(self.blob) };
            let list: List<u8> = list_blob.into();
            let slice = unsafe { std::slice::from_raw_parts(list.as_ptr(), list.len()) };
            std::mem::forget(list);
            slice
        }

        fn into_raw(mut self) -> List<u8> {
            let list_blob = unsafe { lib_ruby_parser_list_blob_from_bytes_blob(self.blob) };
            self.blob = unsafe { lib_ruby_parser_bytes_blob_new() };
            list_blob.into()
        }

        fn set_raw(&mut self, raw: List<u8>) {
            let list_blob = unsafe { lib_ruby_parser_list_blob_from_bytes_blob(self.blob) };
            let list: List<u8> = list_blob.into();
            drop(list);

            let list_blob: ListBlob = raw.into();
            self.blob = unsafe { lib_ruby_parser_bytes_blob_from_list_blob(list_blob) };
        }

        fn push(&mut self, byte: u8) {
            let bytes_blob = self.blob;
            let list_blob = unsafe { lib_ruby_parser_list_blob_from_bytes_blob(bytes_blob) };
            let mut list: List<u8> = list_blob.into();
            list.push(byte);
            let list_blob: ListBlob = list.into();
            let bytes_blob = unsafe { lib_ruby_parser_bytes_blob_from_list_blob(list_blob) };
            self.blob = bytes_blob;
        }
    }

    impl std::ops::Index<usize> for Bytes {
        type Output = u8;

        fn index(&self, index: usize) -> &Self::Output {
            self.as_raw().index(index)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{Bytes, BytesTrait, List, BYTES_SIZE};

        #[test]
        fn test_size() {
            assert_eq!(std::mem::size_of::<Bytes>(), BYTES_SIZE);
        }

        #[test]
        fn test_new() {
            let bytes = Bytes::new(vec![1, 2, 3]);
            drop(bytes);
        }

        #[test]
        fn test_as_raw() {
            let bytes = Bytes::new(vec![1, 2, 3]);

            assert_eq!(bytes.as_raw(), &[1, 2, 3])
        }

        #[test]
        fn test_into_raw() {
            let bytes = Bytes::new(vec![1, 2, 3]);

            assert_eq!(bytes.into_raw(), List::<u8>::from(vec![1, 2, 3]))
        }

        #[test]
        fn test_set_raw() {
            let mut bytes = Bytes::new(vec![1, 2, 3]);
            bytes.set_raw(vec![4, 5, 6].into());

            assert_eq!(bytes.as_raw(), &[4, 5, 6])
        }

        #[test]
        fn test_push() {
            let mut bytes = Bytes::default();
            for i in 0..10 {
                bytes.push(i);
            }
            assert_eq!(bytes.as_raw(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
        }
    }
}

pub use bytes::Bytes;
