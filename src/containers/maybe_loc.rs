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
        fn lib_ruby_parser__external__maybe_loc__new_some(loc_blob: LocBlob) -> MaybeLocBlob;
        fn lib_ruby_parser__external__maybe_loc__new_none() -> MaybeLocBlob;
        fn lib_ruby_parser__external__maybe_loc__drop(blob: *mut MaybeLocBlob);
        fn lib_ruby_parser__external__maybe_loc__is_some(blob: *const MaybeLocBlob) -> bool;
        fn lib_ruby_parser__external__maybe_loc__is_none(blob: *const MaybeLocBlob) -> bool;
        fn lib_ruby_parser__external__maybe_loc__as_loc(
            blob: *const MaybeLocBlob,
        ) -> *const LocBlob;
        fn lib_ruby_parser__external__maybe_loc__into_loc(blob: MaybeLocBlob) -> LocBlob;
    }

    impl MaybeLocAPI for MaybeLoc {
        fn some(loc: Loc) -> Self {
            let blob = unsafe { lib_ruby_parser__external__maybe_loc__new_some(loc.blob) };
            Self { blob }
        }

        fn none() -> Self {
            let blob = unsafe { lib_ruby_parser__external__maybe_loc__new_none() };
            Self { blob }
        }
    }

    impl MaybeLoc {
        /// Equivalent of Option::is_some
        pub fn is_some(&self) -> bool {
            unsafe { lib_ruby_parser__external__maybe_loc__is_some(&self.blob) }
        }

        /// Equivalent of Option::is_none
        pub fn is_none(&self) -> bool {
            unsafe { lib_ruby_parser__external__maybe_loc__is_none(&self.blob) }
        }

        unsafe fn into_loc(self) -> Loc {
            let loc_blob = lib_ruby_parser__external__maybe_loc__into_loc(self.blob);
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
            self.into_option().unwrap()
        }

        /// Equivalent of Option::unwrap_or_else
        pub fn unwrap_or_else<F>(self, f: F) -> Loc
        where
            F: FnOnce() -> Loc,
        {
            self.into_option().unwrap_or_else(f)
        }

        /// Equivalent of Option::expect
        pub fn expect(self, message: &str) -> Loc {
            self.into_option().expect(message)
        }

        /// Equivalent of Option::map
        pub fn map<F>(self, f: F) -> Self
        where
            F: FnOnce(Loc) -> Loc,
        {
            if self.is_some() {
                let mut loc = unsafe { self.into_loc() };
                loc = f(loc);
                Self::some(loc)
            } else {
                Self::none()
            }
        }

        /// Equivalent of Option::as_ref
        pub fn as_ref(&self) -> Option<&Loc> {
            unsafe {
                (lib_ruby_parser__external__maybe_loc__as_loc(&self.blob) as *const Loc).as_ref()
            }
        }

        fn into_option(self) -> Option<Loc> {
            if self.is_some() {
                Some(unsafe { self.into_loc() })
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

    impl From<Option<Loc>> for MaybeLoc {
        fn from(maybe_loc: Option<Loc>) -> Self {
            if let Some(loc) = maybe_loc {
                MaybeLoc::some(loc)
            } else {
                MaybeLoc::none()
            }
        }
    }

    impl From<MaybeLoc> for Option<Loc> {
        fn from(maybe_loc: MaybeLoc) -> Self {
            maybe_loc.into_option()
        }
    }

    impl Drop for MaybeLoc {
        fn drop(&mut self) {
            unsafe { lib_ruby_parser__external__maybe_loc__drop(&mut self.blob) }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{Loc, MaybeLoc, MaybeLocAPI};

        #[test]
        fn test_some() {
            let some_loc = MaybeLoc::some(Loc::new(1, 2));
            assert!(some_loc.is_some());
            assert!(!some_loc.is_none());
            assert_eq!(some_loc.as_ref(), Some(&Loc::new(1, 2)));
            drop(some_loc)
        }

        #[test]
        fn test_none() {
            let none_loc = MaybeLoc::none();
            assert!(!none_loc.is_some());
            assert!(none_loc.is_none());
            assert_eq!(none_loc.as_ref(), None);
            drop(none_loc)
        }
    }
}
