use crate::containers::StringPtr;

#[cfg(not(feature = "c-structures"))]
pub(crate) mod rust {
    pub type MaybeStringPtr = Option<String>;
}

#[cfg(feature = "c-structures")]
pub(crate) mod c {
    use super::{MaybeStringPtrAsStringOption, MaybeStringPtrNone, MaybeStringPtrSome, StringPtr};

    /// C-compatible String container
    #[repr(C)]
    pub struct MaybeStringPtr {
        ptr: *mut u8,
        len: usize,
    }

    impl std::fmt::Debug for MaybeStringPtr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.as_str(), f)
        }
    }

    impl Clone for MaybeStringPtr {
        fn clone(&self) -> Self {
            match self.as_str() {
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
        pub fn unwrap(self) -> StringPtr {
            match self.into_string() {
                Some(s) => StringPtr::from(s),
                None => panic!("failed to unwrap null StringPtr"),
            }
        }

        /// Returns raw pointer
        pub fn take(mut self) -> *mut u8 {
            let ptr = self.ptr;
            self.ptr = std::ptr::null_mut();
            ptr
        }
    }

    impl MaybeStringPtrAsStringOption for MaybeStringPtr {
        fn into_string(self) -> Option<String> {
            let len = self.len;
            let ptr = self.take();

            if ptr.is_null() {
                None
            } else {
                let bytes = unsafe { std::slice::from_raw_parts(ptr, len) }.to_vec();
                Some(String::from_utf8(bytes).unwrap())
            }
        }

        fn as_str(&self) -> Option<&str> {
            if self.ptr.is_null() {
                None
            } else {
                let bytes = unsafe { std::slice::from_raw_parts(self.ptr, self.len) };
                Some(std::str::from_utf8(bytes).unwrap())
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
            self.as_str() == other.as_str()
        }
    }
}

pub(crate) trait MaybeStringPtrAsStringOption {
    fn into_string(self) -> Option<String>
    where
        Self: Sized;

    fn as_str(&self) -> Option<&str>;
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
