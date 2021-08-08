#[cfg(not(feature = "compile-with-external-structures"))]
pub(crate) mod rust {
    /// Rust-compatible nullable string container
    pub type MaybeStringPtr = Option<String>;

    use super::MaybeStringPtrAPI;
    impl MaybeStringPtrAPI for MaybeStringPtr {
        fn some<T>(value: T) -> Self
        where
            T: Into<String>,
        {
            Some(value.into())
        }

        fn none() -> Self {
            None
        }
    }
}

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod external {
    use super::MaybeStringPtrAPI;
    use crate::containers::size::MAYBE_STRING_PTR_SIZE;
    use crate::containers::ExternalStringPtr;

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub(crate) struct MaybeStringPtrBlob {
        blob: [u8; MAYBE_STRING_PTR_SIZE],
    }

    /// C-compatible nullable MaybeString container
    #[repr(C)]
    pub struct MaybeStringPtr {
        pub(crate) blob: MaybeStringPtrBlob,
    }

    extern "C" {
        fn lib_ruby_parser__internal__containers__maybe_string_ptr__make_some(
            ptr: *const u8,
            suze: u64,
        ) -> MaybeStringPtrBlob;
        fn lib_ruby_parser__internal__containers__maybe_string_ptr__make_none() -> MaybeStringPtrBlob;

        fn lib_ruby_parser__internal__containers__maybe_string_ptr__is_some(
            blob: *const MaybeStringPtrBlob,
        ) -> bool;
        fn lib_ruby_parser__internal__containers__maybe_string_ptr__is_none(
            blob: *const MaybeStringPtrBlob,
        ) -> bool;

        fn lib_ruby_parser__internal__containers__maybe_string_ptr__free(
            blob: *mut MaybeStringPtrBlob,
        );
        fn lib_ruby_parser__internal__containers__maybe_string_ptr__get_raw_const(
            blob: *const MaybeStringPtrBlob,
        ) -> *const u8;
        fn lib_ruby_parser__internal__containers__maybe_string_ptr__into_raw(
            blob: *mut MaybeStringPtrBlob,
        ) -> *mut u8;
        fn lib_ruby_parser__internal__containers__maybe_string_ptr__len(
            blob: *const MaybeStringPtrBlob,
        ) -> u64;
    }

    impl Drop for MaybeStringPtr {
        fn drop(&mut self) {
            let blob_ptr: *mut MaybeStringPtrBlob = &mut self.blob;
            unsafe { lib_ruby_parser__internal__containers__maybe_string_ptr__free(blob_ptr) }
        }
    }

    impl std::fmt::Debug for MaybeStringPtr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.as_ref(), f)
        }
    }

    impl Clone for MaybeStringPtr {
        fn clone(&self) -> Self {
            match self.as_ref() {
                Some(s) => Self::some(s.to_string()),
                None => Self::none(),
            }
        }
    }

    impl Default for MaybeStringPtr {
        fn default() -> Self {
            Self::none()
        }
    }

    impl MaybeStringPtr {
        /// Equivalent of Option::is_some()
        pub fn is_some(&self) -> bool {
            let blob_ptr: *const MaybeStringPtrBlob = &self.blob;
            unsafe { lib_ruby_parser__internal__containers__maybe_string_ptr__is_some(blob_ptr) }
        }

        /// Equivalent of Option::is_none()
        pub fn is_none(&self) -> bool {
            let blob_ptr: *const MaybeStringPtrBlob = &self.blob;
            unsafe { lib_ruby_parser__internal__containers__maybe_string_ptr__is_none(blob_ptr) }
        }

        /// Equivalent of Option::unwrap()
        pub fn unwrap(mut self) -> ExternalStringPtr {
            if self.is_some() {
                let blob_ptr: *mut MaybeStringPtrBlob = &mut self.blob;
                let len = unsafe {
                    lib_ruby_parser__internal__containers__maybe_string_ptr__len(blob_ptr) as usize
                };
                let ptr = unsafe {
                    lib_ruby_parser__internal__containers__maybe_string_ptr__into_raw(blob_ptr)
                };
                let bytes = unsafe { Vec::from_raw_parts(ptr, len, len) };
                let s = String::from_utf8(bytes).unwrap();
                ExternalStringPtr::from(s)
            } else {
                panic!("failed to unwrap null MaybeStringPtr")
            }
        }

        /// Equivalent of Option::as_ref
        pub fn as_ref(&self) -> Option<&str> {
            if self.is_some() {
                let blob_ptr: *const MaybeStringPtrBlob = &self.blob;
                let len = unsafe {
                    lib_ruby_parser__internal__containers__maybe_string_ptr__len(blob_ptr) as usize
                };
                let ptr = unsafe {
                    lib_ruby_parser__internal__containers__maybe_string_ptr__get_raw_const(blob_ptr)
                };
                let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
                let s = std::str::from_utf8(bytes).unwrap();
                Some(s)
            } else {
                None
            }
        }
    }

    impl From<Option<String>> for MaybeStringPtr {
        fn from(maybe_string: Option<String>) -> Self {
            match maybe_string {
                Some(string) => Self::some(string),
                None => Self::none(),
            }
        }
    }

    impl MaybeStringPtrAPI for MaybeStringPtr {
        fn some<T>(value: T) -> Self
        where
            T: Into<String>,
        {
            let value: String = value.into();
            let mut bytes = value.into_bytes();
            let ptr = bytes.as_mut_ptr();
            let len = bytes.len() as u64;
            let blob = unsafe {
                lib_ruby_parser__internal__containers__maybe_string_ptr__make_some(ptr, len)
            };
            Self { blob }
        }

        fn none() -> Self {
            let blob =
                unsafe { lib_ruby_parser__internal__containers__maybe_string_ptr__make_none() };
            Self { blob }
        }
    }

    impl PartialEq<MaybeStringPtr> for MaybeStringPtr {
        fn eq(&self, other: &MaybeStringPtr) -> bool {
            self.as_ref() == other.as_ref()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{MaybeStringPtr, MaybeStringPtrAPI};

        #[test]
        fn test_some() {
            let s = MaybeStringPtr::some("foobar");
            assert_eq!(s.as_ref(), Some("foobar"))
        }

        #[test]
        fn test_none() {
            let s = MaybeStringPtr::none();
            assert_eq!(s.as_ref(), None)
        }

        #[test]
        fn test_as_ref() {
            let s = MaybeStringPtr::some("foobar");
            assert_eq!(s.as_ref(), Some("foobar"))
        }
    }
}

pub(crate) trait MaybeStringPtrAPI {
    fn some<T>(value: T) -> Self
    where
        T: Into<String>,
        Self: Sized;

    fn none() -> Self
    where
        Self: Sized;
}
