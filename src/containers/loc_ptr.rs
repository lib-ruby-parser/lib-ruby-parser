use crate::Loc;

#[cfg(not(feature = "c-structures"))]
pub(crate) mod rust {
    use super::Loc;

    /// Rust-compatible not-null Loc pointer (technically not a pointer, but it mimics it)
    pub type LocPtr = Loc;
}

#[cfg(feature = "c-structures")]
pub(crate) mod c {
    use super::Loc;
    use std::ops::Deref;

    /// C-compatible not-null Loc pointer
    #[derive(Debug)]
    #[repr(C)]
    pub struct LocPtr {
        ptr: *mut Loc,
    }

    impl PartialEq for LocPtr {
        fn eq(&self, other: &Self) -> bool {
            PartialEq::eq(&**self, &**other)
        }
    }

    impl Clone for LocPtr {
        fn clone(&self) -> Self {
            let value = self.as_ref().clone();
            Self::new(value)
        }
    }

    impl Deref for LocPtr {
        type Target = Loc;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.ptr }
        }
    }

    impl AsRef<Loc> for LocPtr {
        fn as_ref(&self) -> &Loc {
            unsafe { &*self.ptr }
        }
    }

    impl LocPtr {
        /// Constructs a LocPtr from Loc
        pub fn new(loc: Loc) -> Self {
            let ptr = Box::into_raw(Box::new(loc));
            Self { ptr }
        }
    }

    use super::IntoMaybeLocPtr;
    impl IntoMaybeLocPtr for LocPtr {
        fn into_maybe_ptr(self) -> crate::containers::MaybeLocPtr
        where
            Self: Sized,
        {
            todo!()
        }
    }

    use super::UnwrapLocPtr;
    impl UnwrapLocPtr for LocPtr {
        fn unwrap_ptr(self) -> Loc
        where
            Self: Sized,
        {
            todo!()
        }
    }
}

/// Unwraps the pointer and returns stack value
pub trait IntoMaybeLocPtr {
    /// Unwraps the pointer and returns stack value
    fn into_maybe_ptr(self) -> crate::containers::MaybeLocPtr
    where
        Self: Sized;
}

pub(crate) trait UnwrapLocPtr {
    fn unwrap_ptr(self) -> Loc
    where
        Self: Sized;
}