use crate::containers::MaybeLocPtr;
use crate::Loc;

#[cfg(not(feature = "c-structures"))]
pub(crate) mod rust {
    use super::Loc;

    /// Rust-compatible not-null Loc pointer (technically not a pointer, but it mimics it)
    pub type LocPtr = Loc;

    use super::IntoMaybeLocPtr;
    impl IntoMaybeLocPtr for LocPtr {
        fn into_maybe_ptr(self) -> crate::containers::MaybeLocPtr {
            Some(self)
        }
    }

    use super::UnPtr;
    impl UnPtr for LocPtr {
        fn unptr(self) -> Loc {
            self
        }
    }

    use super::LocPtrNew;
    impl LocPtrNew for LocPtr {
        fn new_ptr(loc: Loc) -> Self {
            loc
        }
    }
}

#[cfg(feature = "c-structures")]
pub(crate) mod c {
    use super::{Loc, MaybeLocPtr};
    use std::ops::Deref;

    /// C-compatible not-null Loc pointer
    #[repr(C)]
    pub struct LocPtr {
        ptr: *mut Loc,
    }

    impl Drop for LocPtr {
        fn drop(&mut self) {
            if self.ptr.is_null() {
                return;
            }

            drop(unsafe { Box::from_raw(self.ptr) });
            self.ptr = std::ptr::null_mut();
        }
    }

    impl std::fmt::Debug for LocPtr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // std::fmt::Debug::fmt(&**self, f)
            f.debug_struct("LocPtr").field("ptr", &self.ptr).finish()
        }
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

        /// Constructs LocPtr from a raw pointer
        pub fn from_raw(ptr: *mut Loc) -> Self {
            Self { ptr }
        }

        /// Returns a raw pointer, consumes self
        pub fn into_raw(mut self) -> *mut Loc {
            let ptr = self.ptr;
            self.ptr = std::ptr::null_mut();
            ptr
        }
    }

    use super::IntoMaybeLocPtr;
    impl IntoMaybeLocPtr for LocPtr {
        fn into_maybe_ptr(self) -> MaybeLocPtr {
            use crate::containers::maybe_loc_ptr::MaybeLocPtrSome;
            MaybeLocPtr::some(self.unptr())
        }
    }

    use super::UnPtr;
    impl UnPtr for LocPtr {
        fn unptr(self) -> Loc {
            *unsafe { Box::from_raw(self.into_raw()) }
        }
    }

    use super::LocPtrNew;
    impl LocPtrNew for LocPtr {
        fn new_ptr(loc: Loc) -> Self {
            Self::new(loc)
        }
    }
}

/// Constructs a LocPtr from Loc
pub trait LocPtrNew {
    /// Constructs a LocPtr from Loc
    fn new_ptr(loc: Loc) -> Self
    where
        Self: Sized;
}

/// Unwraps the pointer and returns stack value
pub trait IntoMaybeLocPtr {
    /// Unwraps the pointer and returns stack value
    fn into_maybe_ptr(self) -> MaybeLocPtr
    where
        Self: Sized;
}

pub(crate) trait UnPtr {
    fn unptr(self) -> Loc
    where
        Self: Sized;
}
