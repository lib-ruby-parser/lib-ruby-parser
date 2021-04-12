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
    use super::Loc;

    /// C-compatible nullable pointer
    #[derive(Debug)]
    #[repr(C)]
    pub struct MaybeLocPtr {
        ptr: *mut Loc,
    }

    impl PartialEq for MaybeLocPtr {
        fn eq(&self, other: &Self) -> bool {
            PartialEq::eq(&self.as_option(), &other.as_option())
        }
    }

    impl Clone for MaybeLocPtr {
        fn clone(&self) -> Self {
            todo!()
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
        pub fn new(ptr: *mut Loc) -> Self {
            Self { ptr }
        }

        /// Equivalent of Option::or_else
        pub fn or_else<F>(self, _f: F) -> Self
        where
            F: FnOnce() -> Self,
        {
            todo!()
        }

        /// Equivalent of Option::unwrap
        pub fn unwrap(self) -> crate::containers::LocPtr {
            todo!()
        }

        /// Equivalent of Option::unwrap_or_else
        pub fn unwrap_or_else<F>(self, _f: F) -> crate::containers::LocPtr
        where
            F: FnOnce() -> crate::containers::LocPtr,
        {
            todo!()
        }

        /// Equivalent of Option::expect
        pub fn expect(self, _message: &str) -> crate::containers::LocPtr {
            todo!()
        }

        /// Equivalent of Option::map
        pub fn map<F>(self, _f: F) -> Self
        where
            F: FnOnce(crate::containers::LocPtr) -> crate::containers::LocPtr,
        {
            todo!()
        }

        /// Equivalent of Option::is_none
        pub fn is_none(&self) -> bool {
            todo!()
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
