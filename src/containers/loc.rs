use crate::containers::MaybeLoc;
use crate::Loc;

#[cfg(not(feature = "c-structures"))]
pub(crate) mod rust {
    /// Rust-compatible not-null Loc pointer (technically not a pointer, but it mimics it)
    pub type Loc = super::Loc;

    use super::IntoMaybeLoc;
    impl IntoMaybeLoc for Loc {
        fn into_maybe_ptr(self) -> crate::containers::MaybeLoc {
            Some(self)
        }
    }

    // use super::UnPtr;
    // impl UnPtr for Loc {
    //     fn unptr(self) -> Loc {
    //         self
    //     }
    // }

    // use super::LocNew;
    // impl LocNew for Loc {
    //     fn new_ptr(loc: Loc) -> Self {
    //         loc
    //     }
    // }
}

#[cfg(feature = "c-structures")]
pub(crate) mod c {
    use super::MaybeLoc;

    /// C-compatible not-null Loc pointer
    pub type Loc = super::Loc;

    // impl Drop for Loc {
    //     fn drop(&mut self) {
    //         if self.ptr.is_null() {
    //             return;
    //         }

    //         drop(unsafe { Box::from_raw(self.ptr) });
    //         self.ptr = std::ptr::null_mut();
    //     }
    // }

    // impl std::fmt::Debug for Loc {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         std::fmt::Debug::fmt(&**self, f)
    //     }
    // }

    // impl PartialEq for Loc {
    //     fn eq(&self, other: &Self) -> bool {
    //         PartialEq::eq(&**self, &**other)
    //     }
    // }

    // impl Clone for Loc {
    //     fn clone(&self) -> Self {
    //         let value = self.as_ref().clone();
    //         Self::new(value)
    //     }
    // }

    // impl Deref for Loc {
    //     type Target = Loc;

    //     fn deref(&self) -> &Self::Target {
    //         unsafe { &*self.ptr }
    //     }
    // }

    // impl AsRef<Loc> for Loc {
    //     fn as_ref(&self) -> &Loc {
    //         unsafe { &*self.ptr }
    //     }
    // }

    impl Loc {
        // / Constructs a Loc from Loc
        // pub fn new(loc: Loc) -> Self {
        //     loc
        // }

        // / Constructs Loc from a raw pointer
        // pub fn from_raw(ptr: *mut Loc) -> Self {
        //     Self { ptr }
        // }

        // / Returns a raw pointer, consumes self
        // pub fn into_raw(mut self) -> *mut Loc {
        //     let ptr = self.ptr;
        //     self.ptr = std::ptr::null_mut();
        //     ptr
        // }
    }

    use super::IntoMaybeLoc;
    impl IntoMaybeLoc for Loc {
        fn into_maybe_ptr(self) -> MaybeLoc {
            use crate::containers::maybe_loc::MaybeLocSome;
            MaybeLoc::some(self)
        }
    }

    // use super::UnPtr;
    // impl UnPtr for Loc {
    //     fn unptr(self) -> Loc {
    //         *unsafe { Box::from_raw(self.into_raw()) }
    //     }
    // }

    // use super::LocNew;
    // impl LocNew for Loc {
    //     fn new_ptr(loc: Loc) -> Self {
    //         Self::new(loc)
    //     }
    // }
}

// /// Constructs a Loc from Loc
// pub trait LocNew {
//     /// Constructs a Loc from Loc
//     fn new_ptr(loc: Loc) -> Self
//     where
//         Self: Sized;
// }

/// Unwraps the pointer and returns stack value
pub trait IntoMaybeLoc {
    /// Unwraps the pointer and returns stack value
    fn into_maybe_ptr(self) -> MaybeLoc
    where
        Self: Sized;
}

// pub(crate) trait UnPtr {
//     fn unptr(self) -> Loc
//     where
//         Self: Sized;
// }
