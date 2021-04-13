#[cfg(not(feature = "c-structures"))]
pub(crate) mod rust {
    /// Rust-compatible not nullable String container
    pub type StringPtr = String;
}

#[cfg(feature = "c-structures")]
pub(crate) mod c {
    use std::ops::Deref;

    /// C-compatible not nullable String container
    #[repr(C)]
    pub struct StringPtr {
        ptr: *mut u8,
        len: usize,
    }

    impl Drop for StringPtr {
        fn drop(&mut self) {
            if self.ptr.is_null() || self.len == 0 {
                return;
            }

            drop(unsafe { Box::from_raw(self.ptr) });
            self.ptr = std::ptr::null_mut();
        }
    }

    impl std::fmt::Debug for StringPtr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(self.as_str(), f)
        }
    }

    impl Clone for StringPtr {
        fn clone(&self) -> Self {
            let mut vec = Vec::with_capacity(self.len);
            unsafe {
                std::ptr::copy(self.ptr, vec.as_mut_ptr(), self.len);
                vec.set_len(self.len);
            }
            Self::from(vec)
        }
    }

    impl Default for StringPtr {
        fn default() -> Self {
            Self::from("")
        }
    }

    impl StringPtr {
        /// Performs uppercase on self. Returns Err if stored byte array is invalid in UTF-8
        pub fn to_uppercase(&self) -> Self {
            Self::from(self.as_str().to_uppercase())
        }

        /// Takes a raw pointer
        pub fn take(mut self) -> *mut u8 {
            let ptr = self.ptr;
            self.ptr = std::ptr::null_mut();
            ptr
        }

        /// Equivalent of String::as_str()
        pub fn as_str(&self) -> &str {
            let bytes = self.deref();
            std::str::from_utf8(bytes).unwrap()
        }
    }

    impl Deref for StringPtr {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
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
            let ptr = bytes.as_mut_ptr();
            let len = bytes.len();
            std::mem::forget(bytes);
            Self { ptr, len }
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

    impl PartialEq<&str> for StringPtr {
        fn eq(&self, other: &&str) -> bool {
            self.as_ref() == other.as_bytes()
        }
    }

    impl PartialEq<StringPtr> for StringPtr {
        fn eq(&self, other: &StringPtr) -> bool {
            self.deref() == other.deref()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::StringPtr;

        #[test]
        fn test_new() {
            let s = StringPtr::from("foo");
            assert_eq!(s, "foo")
        }

        #[test]
        fn test_clone() {
            let s = StringPtr::from("foo");
            let s2 = s.clone();
            assert_eq!(s2, "foo")
        }

        #[test]
        fn test_deref() {
            let s = StringPtr::from("foo");
            let s_ref = s.as_ref();
            assert_eq!(s_ref, b"foo")
        }

        #[test]
        fn test_empty() {
            let s = StringPtr::from("");
            assert_eq!(s.as_str(), "")
        }
    }
}
