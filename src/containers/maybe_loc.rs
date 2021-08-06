use crate::Loc;

pub(crate) trait MaybeLocAPI {
    fn some(loc: Loc) -> Self
    where
        Self: Sized;
    fn none() -> Self
    where
        Self: Sized;
}

#[cfg(not(feature = "compile-with-external-structures"))]
pub(crate) mod rust {
    use super::{Loc, MaybeLocAPI};

    /// Rust-compatible optional loc
    pub type MaybeLoc = Option<Loc>;

    impl MaybeLocAPI for MaybeLoc {
        fn some(loc: Loc) -> Self {
            Some(loc)
        }

        fn none() -> Self {
            None
        }
    }
}

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod external {
    use super::{Loc, MaybeLocAPI};
    use crate::containers::size::MAYBE_LOC_SIZE;
    use crate::loc::LocBlob;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub(crate) struct MaybeLocBlob {
        blob: [u8; MAYBE_LOC_SIZE],
    }

    /// C-compatible Option<Loc>
    #[repr(C)]
    pub struct MaybeLoc {
        pub(crate) blob: MaybeLocBlob,
    }

    extern "C" {
        fn lib_ruby_parser__internal__containers__maybe_loc__make_some(
            loc_blob: LocBlob,
        ) -> MaybeLocBlob;
        fn lib_ruby_parser__internal__containers__maybe_loc__make_none() -> MaybeLocBlob;
        fn lib_ruby_parser__internal__containers__maybe_loc__is_some(blob: MaybeLocBlob) -> bool;
        fn lib_ruby_parser__internal__containers__maybe_loc__is_none(blob: MaybeLocBlob) -> bool;
        fn lib_ruby_parser__internal__containers__maybe_loc__borrow_loc(
            blob: *const MaybeLocBlob,
        ) -> *const LocBlob;
        fn lib_ruby_parser__internal__containers__maybe_loc__into_loc(
            blob: MaybeLocBlob,
        ) -> LocBlob;
    }

    impl MaybeLocAPI for MaybeLoc {
        fn some(loc: Loc) -> Self {
            let blob =
                unsafe { lib_ruby_parser__internal__containers__maybe_loc__make_some(loc.blob) };
            Self { blob }
        }

        fn none() -> Self {
            let blob = unsafe { lib_ruby_parser__internal__containers__maybe_loc__make_none() };
            Self { blob }
        }
    }

    impl MaybeLoc {
        /// Equivalent of Option::is_some
        pub fn is_some(&self) -> bool {
            unsafe { lib_ruby_parser__internal__containers__maybe_loc__is_some(self.blob) }
        }

        /// Equivalent of Option::is_none
        pub fn is_none(&self) -> bool {
            unsafe { lib_ruby_parser__internal__containers__maybe_loc__is_none(self.blob) }
        }

        fn loc(&self) -> &Loc {
            let maybe_loc_blob_ptr: *const MaybeLocBlob = &self.blob;
            let loc_ptr = unsafe {
                lib_ruby_parser__internal__containers__maybe_loc__borrow_loc(maybe_loc_blob_ptr)
                    as *const Loc
            };
            unsafe { loc_ptr.as_ref().unwrap() }
        }

        fn into_loc(self) -> Loc {
            let loc_blob =
                unsafe { lib_ruby_parser__internal__containers__maybe_loc__into_loc(self.blob) };
            Loc { blob: loc_blob }
        }
    }

    impl std::fmt::Debug for MaybeLoc {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.as_ref(), f)
        }
    }

    impl PartialEq for MaybeLoc {
        fn eq(&self, other: &Self) -> bool {
            self.as_ref() == other.as_ref()
        }
    }

    impl Clone for MaybeLoc {
        fn clone(&self) -> Self {
            match self.as_ref() {
                Some(loc) => Self::some(loc.clone()),
                None => Self::none(),
            }
        }
    }

    impl MaybeLoc {
        /// Equivalent of Option::or_else
        pub fn or_else<F>(self, f: F) -> Self
        where
            F: FnOnce() -> Self,
        {
            if self.is_none() {
                f()
            } else {
                self
            }
        }

        /// Equivalent of Option::unwrap
        pub fn unwrap(self) -> Loc {
            if self.is_some() {
                return self.loc().to_owned();
            } else {
                panic!("failed to unwrap MaybeLoc::None")
            }
        }

        /// Equivalent of Option::unwrap_or_else
        pub fn unwrap_or_else<F>(self, f: F) -> Loc
        where
            F: FnOnce() -> Loc,
        {
            match self.as_ref() {
                Some(loc) => loc.to_owned(),
                None => f(),
            }
        }

        /// Equivalent of Option::expect
        pub fn expect(self, message: &str) -> Loc {
            if self.is_some() {
                self.into_loc()
            } else {
                panic!("{}", message)
            }
        }

        /// Equivalent of Option::map
        pub fn map<F>(self, f: F) -> Self
        where
            F: FnOnce(Loc) -> Loc,
        {
            if self.is_some() {
                Self::some(f(self.into_loc()))
            } else {
                Self::none()
            }
        }

        /// Equivalent of Option::as_ref
        pub fn as_ref(&self) -> Option<&Loc> {
            if self.is_some() {
                Some(self.loc())
            } else {
                None
            }
        }
    }

    impl From<Loc> for MaybeLoc {
        fn from(loc: Loc) -> Self {
            Self::some(loc)
        }
    }
}
