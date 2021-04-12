use crate::containers::LocPtr;
use crate::Loc;

#[cfg(not(feature = "c-structures"))]
pub(crate) mod rust {
    use super::Loc;

    /// Rust-compatible nullable pointer
    pub type MaybeLocPtr = Option<Loc>;

    use super::MaybeLocPtrSome;
    impl MaybeLocPtrSome for MaybeLocPtr {
        fn some(loc: Loc) -> Self {
            Some(loc)
        }
    }

    use super::MaybeLocPtrNone;
    impl MaybeLocPtrNone for MaybeLocPtr {
        fn none() -> Self {
            None
        }
    }

    use super::AsLocOption;
    impl AsLocOption for MaybeLocPtr {
        fn as_option(&self) -> Option<&Loc> {
            self.as_ref()
        }
    }

    use super::IntoLocOption;
    impl IntoLocOption for MaybeLocPtr {
        fn into_option(self) -> Option<Loc> {
            self
        }
    }
}

#[cfg(feature = "c-structures")]
pub(crate) mod c {
    use super::{Loc, LocPtr};

    /// C-compatible nullable pointer
    #[repr(C)]
    pub struct MaybeLocPtr {
        ptr: *mut Loc,
    }

    impl Drop for MaybeLocPtr {
        fn drop(&mut self) {
            if self.ptr.is_null() {
                return;
            }

            drop(unsafe { Box::from_raw(self.ptr) });
            self.ptr = std::ptr::null_mut();
        }
    }

    impl std::fmt::Debug for MaybeLocPtr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.as_option(), f)
        }
    }

    impl PartialEq for MaybeLocPtr {
        fn eq(&self, other: &Self) -> bool {
            PartialEq::eq(&self.as_option(), &other.as_option())
        }
    }

    impl Clone for MaybeLocPtr {
        fn clone(&self) -> Self {
            match self.as_option() {
                Some(loc) => Self::some(loc.clone()),
                None => Self::none(),
            }
        }
    }

    use super::MaybeLocPtrSome;
    impl MaybeLocPtrSome for MaybeLocPtr {
        fn some(loc: Loc) -> Self {
            let ptr = Box::into_raw(Box::new(loc));
            Self { ptr }
        }
    }

    use super::MaybeLocPtrNone;
    impl MaybeLocPtrNone for MaybeLocPtr {
        fn none() -> Self {
            Self {
                ptr: std::ptr::null_mut(),
            }
        }
    }

    impl MaybeLocPtr {
        /// Constructs a pointer with a given raw pointer
        pub fn from_raw(ptr: *mut Loc) -> Self {
            Self { ptr }
        }

        /// Unwraps into raw pointer, consumes self
        pub fn into_raw(mut self) -> *mut Loc {
            let ptr = self.ptr;
            self.ptr = std::ptr::null_mut();
            ptr
        }

        /// Equivalent of Option::or_else
        pub fn or_else<F>(self, f: F) -> Self
        where
            F: FnOnce() -> Self,
        {
            if self.ptr.is_null() {
                f()
            } else {
                self
            }
        }

        /// Equivalent of Option::unwrap
        pub fn unwrap(self) -> LocPtr {
            let ptr = self.into_raw();
            if ptr.is_null() {
                panic!("failed to unwrap null MaybeLocPtr")
            } else {
                LocPtr::from_raw(ptr)
            }
        }

        /// Equivalent of Option::unwrap_or_else
        pub fn unwrap_or_else<F>(self, f: F) -> LocPtr
        where
            F: FnOnce() -> LocPtr,
        {
            let ptr = self.into_raw();
            if ptr.is_null() {
                f()
            } else {
                LocPtr::from_raw(ptr)
            }
        }

        /// Equivalent of Option::expect
        pub fn expect(self, message: &str) -> LocPtr {
            let ptr = self.into_raw();
            if ptr.is_null() {
                panic!("{}", message)
            } else {
                LocPtr::from_raw(ptr)
            }
        }

        /// Equivalent of Option::map
        pub fn map<F>(self, f: F) -> Self
        where
            F: FnOnce(LocPtr) -> LocPtr,
        {
            if self.ptr.is_null() {
                self
            } else {
                let ptr = self.into_raw();
                let ptr = LocPtr::from_raw(ptr);
                let ptr = f(ptr);
                let ptr = ptr.into_raw();
                Self::from_raw(ptr)
            }
        }

        /// Equivalent of Option::is_none
        pub fn is_none(&self) -> bool {
            self.ptr.is_null()
        }
    }

    use super::AsLocOption;
    impl AsLocOption for MaybeLocPtr {
        fn as_option(&self) -> Option<&Loc> {
            unsafe { self.ptr.as_ref() }
        }
    }

    use super::IntoLocOption;
    impl IntoLocOption for MaybeLocPtr {
        fn into_option(self) -> Option<Loc> {
            if self.ptr.is_null() {
                None
            } else {
                use crate::containers::loc_ptr::UnPtr;
                Some(self.unwrap().unptr())
            }
        }
    }
}

pub(crate) trait MaybeLocPtrSome {
    fn some(value: Loc) -> Self
    where
        Self: Sized;
}

pub(crate) trait MaybeLocPtrNone {
    fn none() -> Self
    where
        Self: Sized;
}

/// Trait for converting &MaybeLocPtr into Option<&Loc>
pub trait AsLocOption {
    /// Converts &MaybeLocPtr into Option<&Loc>
    fn as_option(&self) -> Option<&Loc>;
}

pub(crate) trait IntoLocOption {
    fn into_option(self) -> Option<Loc>
    where
        Self: Sized;
}
