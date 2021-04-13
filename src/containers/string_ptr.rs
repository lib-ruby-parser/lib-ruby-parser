#[cfg(not(feature = "c-structures"))]
pub(crate) mod rust {
    pub type StringPtr = std::string::String;
}

#[cfg(feature = "c-structures")]
pub(crate) mod c {
    use std::convert::TryFrom;
    use std::ops::Deref;

    type Utf8Error = std::string::FromUtf8Error;

    /// C-compatible String container
    #[repr(C)]
    pub struct StringPtr {
        ptr: *mut u8,
        len: usize,
    }

    impl std::fmt::Debug for StringPtr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&**self, f)
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
        /// Converts self to Rust String (by copying). Invalid chars are replaced with ? char
        pub fn to_string_lossy(&self) -> String {
            String::from_utf8_lossy(self.as_ref()).into_owned()
        }

        /// Converts self to Rust String (by copying). Returns Err if there are utf-8 invalid bytes
        pub fn to_string(&self) -> Result<String, Utf8Error> {
            String::from_utf8(self.to_vec())
        }

        /// Performs uppercase on self. Returns Err if stored byte array is invalid in UTF-8
        pub fn to_uppercase(&self) -> Result<Self, Utf8Error> {
            Ok(Self::from(self.to_string()?.to_uppercase()))
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

    impl TryFrom<StringPtr> for String {
        type Error = std::string::FromUtf8Error;

        fn try_from(value: StringPtr) -> Result<Self, Self::Error> {
            String::from_utf8(value.to_vec())
        }
    }

    impl PartialEq<&str> for StringPtr {
        fn eq(&self, other: &&str) -> bool {
            self.as_ref() == other.as_bytes()
        }
    }
}
