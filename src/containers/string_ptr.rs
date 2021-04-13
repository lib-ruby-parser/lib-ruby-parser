#[cfg(not(feature = "c-structures"))]
pub(crate) mod rust {
    pub type StringPtr = String;
}

#[cfg(feature = "c-structures")]
pub(crate) mod c {
    use super::StringPtrAsString;
    use std::ops::Deref;

    /// C-compatible String container
    #[repr(C)]
    pub struct StringPtr {
        ptr: *mut u8,
        len: usize,
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
    }

    impl Deref for StringPtr {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
        }
    }

    impl StringPtrAsString for StringPtr {
        fn into_string(self) -> String {
            let bytes = unsafe { Vec::from_raw_parts(self.ptr, self.len, self.len) };
            String::from_utf8(bytes).unwrap()
        }

        fn as_str(&self) -> &str {
            let bytes = self.deref();
            std::str::from_utf8(bytes).unwrap()
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
}

pub(crate) trait StringPtrAsString {
    fn into_string(self) -> String
    where
        Self: Sized;

    fn as_str(&self) -> &str;
}
