use crate::Loc;

#[cfg(not(feature = "c-structures"))]
pub(crate) mod rust {
    use super::Loc;

    /// Rust-compatible nullable pointer
    pub type MaybeLocPtr = Option;

    use super::MaybeLocPtrSome;
    impl MaybeLocPtrSome for MaybePtr {
        fn some(loc: Loc) -> Self {
            Some(loc)
        }
    }

    use super::MaybeLocPtrNone;
    impl MaybeLocPtrNone for MaybePtr {
        fn none() -> Self {
            None
        }
    }
}

#[cfg(feature = "c-structures")]
pub(crate) mod c {
    use super::Loc;
    use std::ops::Deref;

    /// C-compatible nullable pointer
    #[derive(Debug)]
    #[repr(C)]
    pub struct MaybeLocPtr {
        ptr: *mut Loc,
    }

    impl PartialEq for MaybeLocPtr {
        fn eq(&self, other: &Self) -> bool {
            PartialEq::eq(&**self, &**other)
        }
    }

    impl Clone for MaybeLocPtr {
        fn clone(&self) -> Self {
            todo!()
        }
    }

    impl Deref for MaybeLocPtr {
        type Target = Option<Loc>;

        fn deref(&self) -> &Self::Target {
            todo!()
            // if self.ptr.is_null() {
            //     &None
            // } else {
            //     let value = unsafe { *self.ptr };
            //     &Some(value)
            // }
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

        /// Equivalent of Option::expect
        pub fn expect(_message: &str) -> crate::containers::LocPtr {
            todo!()
        }

        /// Equivalent of Option::map
        pub fn map<Return, F>(self, _f: F) -> Return
        where
            F: FnOnce(Loc) -> Return,
        {
            todo!()
        }
    }

    use super::IntoLocPtrOrElse;
    impl IntoLocPtrOrElse for MaybeLocPtr {
        fn into_ptr_or_else<F>(self, _f: F) -> crate::containers::LocPtr
        where
            F: FnOnce() -> crate::containers::LocPtr,
        {
            todo!()
        }
    }

    // impl From<Option<Box>> for MaybeLocPtr {
    //     fn from(maybe_boxed: Option<Box>) -> Self {
    //         match maybe_boxed {
    //             Some(boxed) => Self::some(*boxed),
    //             None => Self::none(),
    //         }
    //     }
    // }

    use super::IntoLocPtr;
    impl IntoLocPtr for MaybeLocPtr {
        fn into_ptr(self, _message: &str) -> crate::containers::LocPtr {
            todo!()
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

pub(crate) trait IntoLocPtrOrElse {
    fn into_ptr_or_else<F>(self, f: F) -> crate::containers::LocPtr
    where
        F: FnOnce() -> crate::containers::LocPtr,
        Self: Sized;
}

pub(crate) trait IntoLocPtr {
    fn into_ptr(self, message: &str) -> crate::containers::LocPtr;
}
