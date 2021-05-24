#[cfg(not(feature = "compile-with-external-structures"))]
pub(crate) mod rust {
    /// Rust-compatible nullable string container
    pub type MaybeStringPtr = Option<String>;

    use super::MaybeStringPtrSome;
    impl MaybeStringPtrSome for MaybeStringPtr {
        fn some<T>(value: T) -> Self
        where
            T: Into<String>,
        {
            Some(value.into())
        }
    }

    use super::MaybeStringPtrNone;
    impl MaybeStringPtrNone for MaybeStringPtr {
        fn none() -> Self {
            None
        }
    }
}

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod c {
    use super::{MaybeStringPtrNone, MaybeStringPtrSome};
    use crate::containers::ExternalStringPtr;

    /// C-compatible nullable String container
    #[repr(C)]
    pub struct MaybeStringPtr {
        ptr: *mut u8,
        len: usize,
    }

    impl Drop for MaybeStringPtr {
        fn drop(&mut self) {
            if self.ptr.is_null() {
                return;
            }

            drop(unsafe { Box::from_raw(self.ptr) });
            self.ptr = std::ptr::null_mut();
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
        /// Equivalent of Option::unwrap()
        pub fn unwrap(self) -> ExternalStringPtr {
            let len = self.len;
            let ptr = self.take();

            if ptr.is_null() {
                panic!("failed to unwrap null StringPtr")
            } else {
                let bytes = unsafe { Vec::from_raw_parts(ptr, len, len) };
                let s = String::from_utf8(bytes).unwrap();
                ExternalStringPtr::from(s)
            }
        }

        /// Returns raw pointer
        pub fn take(mut self) -> *mut u8 {
            let ptr = self.ptr;
            self.ptr = std::ptr::null_mut();
            ptr
        }

        /// Equivalent of Option::as_ref
        pub fn as_ref(&self) -> Option<&str> {
            if self.ptr.is_null() {
                None
            } else {
                let bytes = unsafe { std::slice::from_raw_parts(self.ptr, self.len) };
                let s = std::str::from_utf8(bytes).unwrap();
                Some(s)
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

    impl MaybeStringPtrSome for MaybeStringPtr {
        fn some<T>(value: T) -> Self
        where
            T: Into<String>,
        {
            let value: String = value.into();
            let mut bytes = value.into_bytes();
            let ptr = bytes.as_mut_ptr();
            let len = bytes.len();
            std::mem::forget(bytes);
            Self { ptr, len }
        }
    }

    impl MaybeStringPtrNone for MaybeStringPtr {
        fn none() -> Self {
            Self {
                ptr: std::ptr::null_mut(),
                len: 0,
            }
        }
    }

    impl PartialEq<MaybeStringPtr> for MaybeStringPtr {
        fn eq(&self, other: &MaybeStringPtr) -> bool {
            self.as_ref() == other.as_ref()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{MaybeStringPtr, MaybeStringPtrNone, MaybeStringPtrSome};

        #[test]
        fn test_some() {
            let s = MaybeStringPtr::some("foo");
            assert_eq!(s.as_ref(), Some("foo"))
        }

        #[test]
        fn test_none() {
            let s = MaybeStringPtr::none();
            assert_eq!(s.as_ref(), None)
        }

        #[test]
        fn test_as_ref() {
            let s = MaybeStringPtr::some("foo");
            assert_eq!(s.as_ref(), Some("foo"))
        }
    }
}

pub(crate) trait MaybeStringPtrSome {
    fn some<T>(value: T) -> Self
    where
        T: Into<String>,
        Self: Sized;
}

pub(crate) trait MaybeStringPtrNone {
    fn none() -> Self
    where
        Self: Sized;
}
