#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod external {
    use crate::blobs::{Blob, HasBlob};
    use std::ops::Deref;

    /// C-compatible list
    #[repr(C)]
    pub struct StringPtr {
        pub(crate) blob: Blob<StringPtr>,
    }

    extern "C" {
        fn lib_ruby_parser__external__string_ptr__new(ptr: *const u8, len: u64) -> Blob<StringPtr>;
        fn lib_ruby_parser__external__string_ptr__drop(blob: *mut Blob<StringPtr>);
        fn lib_ruby_parser__external__string_ptr__as_raw(
            blob_ptr: *const Blob<StringPtr>,
        ) -> *const u8;
        fn lib_ruby_parser__external__string_ptr__into_raw(blob_ptr: Blob<StringPtr>) -> *mut u8;
        fn lib_ruby_parser__external__string_ptr__get_len(blob: *const Blob<StringPtr>) -> u64;
    }

    impl Drop for StringPtr {
        fn drop(&mut self) {
            unsafe { lib_ruby_parser__external__string_ptr__drop(&mut self.blob) }
        }
    }

    impl std::fmt::Debug for StringPtr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(self.as_str(), f)
        }
    }

    impl Clone for StringPtr {
        fn clone(&self) -> Self {
            let vec: &[u8] = self.into();
            Self::from(vec.to_vec())
        }
    }

    impl Default for StringPtr {
        fn default() -> Self {
            Self::from("")
        }
    }

    impl StringPtr {
        pub(crate) fn from_raw(ptr: *mut u8, len: u64) -> Self {
            let blob = unsafe { lib_ruby_parser__external__string_ptr__new(ptr, len) };
            Self { blob }
        }

        /// Performs uppercase on self. Returns Err if stored byte array is invalid in UTF-8
        pub fn to_uppercase(&self) -> Self {
            Self::from(self.as_str().to_uppercase())
        }

        /// Equivalent of String::as_str()
        pub fn as_str(&self) -> &str {
            let bytes = self.deref();
            std::str::from_utf8(bytes).unwrap()
        }

        /// Equivalent of String::len
        pub fn len(&self) -> usize {
            unsafe { lib_ruby_parser__external__string_ptr__get_len(&self.blob) as usize }
        }

        /// Equivalent of String::is_empty
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        pub(crate) fn as_ptr(&self) -> *const u8 {
            unsafe { lib_ruby_parser__external__string_ptr__as_raw(&self.blob) }
        }

        pub(crate) fn into_ptr(self) -> *mut u8 {
            unsafe { lib_ruby_parser__external__string_ptr__into_raw(self.into_blob()) }
        }
    }

    impl Deref for StringPtr {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            let s: &[u8] = self.into();
            s
        }
    }

    impl From<String> for StringPtr {
        fn from(s: String) -> Self {
            Self::from(s.into_bytes())
        }
    }

    impl From<&String> for StringPtr {
        fn from(s: &String) -> Self {
            Self::from(s.to_owned())
        }
    }

    impl From<&str> for StringPtr {
        fn from(s: &str) -> Self {
            Self::from(s.to_string())
        }
    }

    impl From<Vec<u8>> for StringPtr {
        fn from(mut bytes: Vec<u8>) -> Self {
            bytes.shrink_to_fit();
            let len = bytes.len() as u64;
            let ptr = if len == 0 {
                std::ptr::null_mut()
            } else {
                bytes.as_mut_ptr()
            };
            std::mem::forget(bytes);

            Self::from_raw(ptr, len)
        }
    }

    impl From<&[u8]> for StringPtr {
        fn from(bytes: &[u8]) -> Self {
            Self::from(bytes.to_vec())
        }
    }

    impl From<StringPtr> for String {
        fn from(value: StringPtr) -> Self {
            String::from_utf8(value.to_vec()).unwrap()
        }
    }

    impl From<StringPtr> for Vec<u8> {
        fn from(value: StringPtr) -> Self {
            let len = value.len();
            let ptr = value.into_ptr();
            unsafe { Vec::from_raw_parts(ptr, len, len) }
        }
    }

    impl From<&StringPtr> for &[u8] {
        fn from(value: &StringPtr) -> Self {
            let ptr = value.as_ptr();
            let len = value.len();
            unsafe { std::slice::from_raw_parts(ptr, len) }
        }
    }

    impl PartialEq<&str> for StringPtr {
        fn eq(&self, other: &&str) -> bool {
            self.as_ref() == other.as_bytes()
        }
    }

    impl PartialEq<str> for StringPtr {
        fn eq(&self, other: &str) -> bool {
            self.as_ref() == other.as_bytes()
        }
    }

    impl PartialEq<StringPtr> for &str {
        fn eq(&self, other: &StringPtr) -> bool {
            self.as_bytes() == other.as_ref()
        }
    }

    impl PartialEq<String> for StringPtr {
        fn eq(&self, other: &String) -> bool {
            self.as_ref() == other.as_bytes()
        }
    }

    impl PartialEq for StringPtr {
        fn eq(&self, other: &StringPtr) -> bool {
            self.deref() == other.deref()
        }
    }

    impl Eq for StringPtr {}

    use crate::containers::ExternalList;
    impl From<StringPtr> for ExternalList<u8> {
        fn from(s: StringPtr) -> Self {
            ExternalList::from(s.to_vec())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::StringPtr;
        use std::ops::Deref;

        const SHORT_STR: &str = "a";
        const LONG_STR: &str = "aaaaaaaaaaaaaaaaaaaaaaaaa";

        #[test]
        fn test_new() {
            let short = StringPtr::from(SHORT_STR);
            assert_eq!(short, SHORT_STR);

            let long = StringPtr::from(LONG_STR);
            assert_eq!(long, LONG_STR)
        }

        #[test]
        fn test_len() {
            let short = String::from(SHORT_STR);
            assert_eq!(short.len(), 1);

            let long = String::from(LONG_STR);
            assert_eq!(long.len(), 25);
        }

        #[test]
        fn test_as_ptr() {
            let short = StringPtr::from(SHORT_STR);
            assert_eq!(short.as_ptr(), short.as_ptr());
            assert_ne!(short.as_ptr(), std::ptr::null_mut());

            let long = StringPtr::from(LONG_STR);
            assert_eq!(long.as_ptr(), long.as_ptr());
            assert_ne!(long.as_ptr(), std::ptr::null_mut());
        }

        #[test]
        fn test_clone() {
            let short = StringPtr::from(SHORT_STR);
            assert_eq!(short.clone(), SHORT_STR);
            assert_ne!(short.as_ptr(), short.clone().as_ptr());

            let long = StringPtr::from(LONG_STR);
            assert_eq!(long.clone(), LONG_STR);
            assert_ne!(long.as_ptr(), long.clone().as_ptr());
        }

        #[test]
        fn test_deref() {
            let short = StringPtr::from(SHORT_STR);
            let short_ref = short.deref();
            assert_eq!(short_ref, SHORT_STR.as_bytes());

            let long = StringPtr::from(LONG_STR);
            let long_ref = long.deref();
            assert_eq!(long_ref, LONG_STR.as_bytes());
        }

        #[test]
        fn test_empty() {
            let empty = StringPtr::from("");
            assert_eq!(empty.len(), 0);
            assert_eq!(empty.as_ptr(), std::ptr::null_mut());
            assert_eq!(empty.as_str(), "");

            let empty_ref = empty.deref();
            assert_eq!(empty_ref, &[]);
        }

        #[test]
        fn test_from() {
            let string_ptr = StringPtr::from("foo");
            drop(string_ptr);

            let string_ptr = StringPtr::from(String::from("foo"));
            drop(string_ptr);

            let string_ptr = StringPtr::from(&String::from("foo"));
            drop(string_ptr);

            let string_ptr = StringPtr::from(vec![1, 2, 3]);
            drop(string_ptr);

            let vec = Vec::from(StringPtr::from("foo"));
            assert_eq!(vec, vec![b'f', b'o', b'o']);
        }
    }
}
