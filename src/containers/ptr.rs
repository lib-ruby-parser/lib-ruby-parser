#[cfg(not(feature = "c-structures"))]
pub mod rust {
    /// Rust-compatible not-null pointer
    pub type Ptr<T> = Box<T>;

    use super::ToMaybePtr;
    impl<T> ToMaybePtr<T> for Ptr<T> {
        fn to_maybe_ptr(self) -> crate::containers::MaybePtr<T>
        where
            Self: Sized,
        {
            Some(self)
        }
    }
}

#[cfg(feature = "c-structures")]
pub mod c {
    /// C-compatible not-null pointer
    #[derive(Debug)]
    #[repr(C)]
    pub struct Ptr<T> {
        ptr: *mut T,
    }
}

pub(crate) trait ToMaybePtr<T> {
    fn to_maybe_ptr(self) -> crate::containers::MaybePtr<T>
    where
        Self: Sized;
}
