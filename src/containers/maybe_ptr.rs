#[cfg(not(feature = "c-structures"))]
pub mod rust {
    /// Rust-compatible nullable pointer
    pub type MaybePtr<T> = Option<Box<T>>;

    use super::MaybePtrSome;
    impl<T> MaybePtrSome<T> for MaybePtr<T> {
        fn some(value: T) -> Self
        where
            Self: Sized,
        {
            Some(Box::new(value))
        }
    }

    use super::MaybePtrNone;
    impl<T> MaybePtrNone<T> for MaybePtr<T> {
        fn none() -> Self
        where
            Self: Sized,
        {
            None
        }
    }
}

#[cfg(feature = "c-structures")]
pub mod c {
    /// C-compatible nullable pointer
    #[derive(Debug)]
    #[repr(C)]
    pub struct MaybePtr<T> {
        ptr: *mut T,
    }
}

pub(crate) trait MaybePtrSome<T> {
    fn some(value: T) -> Self
    where
        Self: Sized;
}

pub(crate) trait MaybePtrNone<T> {
    fn none() -> Self
    where
        Self: Sized;
}
