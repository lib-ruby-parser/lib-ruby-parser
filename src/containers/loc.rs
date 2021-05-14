use crate::containers::MaybeLoc;
use crate::Loc;

#[cfg(not(feature = "compile-with-external-structures"))]
pub(crate) mod rust {
    use super::{IntoMaybeLoc, MaybeLoc};

    /// Rust-compatible not-null Loc pointer (technically not a pointer, but it mimics it)
    pub type Loc = super::Loc;

    impl IntoMaybeLoc for Loc {
        fn into_maybe_ptr(self) -> MaybeLoc {
            Some(self)
        }
    }
}

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod c {
    use super::{IntoMaybeLoc, MaybeLoc};

    /// C-compatible not-null Loc pointer
    pub type Loc = super::Loc;

    impl IntoMaybeLoc for Loc {
        fn into_maybe_ptr(self) -> MaybeLoc {
            use crate::containers::maybe_loc::MaybeLocSome;
            MaybeLoc::some(self)
        }
    }
}

/// Unwraps the pointer and returns stack value
pub trait IntoMaybeLoc {
    /// Unwraps the pointer and returns stack value
    fn into_maybe_ptr(self) -> MaybeLoc
    where
        Self: Sized;
}
