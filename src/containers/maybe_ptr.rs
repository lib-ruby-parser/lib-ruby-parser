#[cfg(not(feature = "c-structures"))]
pub(crate) mod rust {
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

    use super::AsOption;
    impl<T> AsOption<T> for MaybePtr<T> {
        fn as_option(&self) -> Option<&T> {
            self.as_ref().map(|t| &**t)
        }
    }

    use super::IntoOption;
    impl<T> IntoOption<T> for MaybePtr<T> {
        fn into_option(self) -> Option<T> {
            self.map(|t| *t)
        }
    }
}

#[cfg(feature = "c-structures")]
pub(crate) mod c {
    /// C-compatible nullable pointer
    #[derive(Debug)]
    #[repr(C)]
    pub struct MaybePtr<T> {
        ptr: *mut T,
    }

    impl<T> PartialEq for MaybePtr<T>
    where
        T: PartialEq,
    {
        fn eq(&self, other: &Self) -> bool {
            PartialEq::eq(&self.as_option(), &other.as_option())
        }
    }

    impl<T> Clone for MaybePtr<T>
    where
        T: Clone,
    {
        fn clone(&self) -> Self {
            todo!()
        }
    }

    use super::MaybePtrSome;
    impl<T> MaybePtrSome<T> for MaybePtr<T> {
        fn some(value: T) -> Self
        where
            Self: Sized,
        {
            let ptr = Box::into_raw(Box::new(value));
            Self { ptr }
        }
    }

    use super::MaybePtrNone;
    impl<T> MaybePtrNone<T> for MaybePtr<T> {
        fn none() -> Self
        where
            Self: Sized,
        {
            Self {
                ptr: std::ptr::null_mut(),
            }
        }
    }

    impl<T> MaybePtr<T> {
        /// Constructs a pointer with a given raw pointer
        pub fn new(ptr: *mut T) -> Self {
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
        pub fn expect(self, _message: &str) -> crate::containers::Ptr<T> {
            todo!()
        }

        /// Equivalent of Option::map
        pub fn map<F>(self, _f: F) -> Self
        where
            F: FnOnce(crate::containers::Ptr<T>) -> crate::containers::Ptr<T>,
        {
            todo!()
        }
    }

    impl<T> From<Option<Box<T>>> for MaybePtr<T> {
        fn from(maybe_boxed: Option<Box<T>>) -> Self {
            match maybe_boxed {
                Some(boxed) => Self::some(*boxed),
                None => Self::none(),
            }
        }
    }

    impl<T> From<MaybePtr<T>> for Option<Box<T>> {
        fn from(_: MaybePtr<T>) -> Self {
            todo!()
        }
    }

    use super::AsOption;
    impl<T> AsOption<T> for MaybePtr<T> {
        fn as_option(&self) -> Option<&T> {
            todo!()
        }
    }

    use super::IntoOption;
    impl<T> IntoOption<T> for MaybePtr<T> {
        fn into_option(self) -> Option<T>
        where
            Self: Sized,
        {
            todo!()
        }
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

/// Trait for converting &Ptr<T> into Option<&T>
pub trait AsOption<T> {
    /// Converts &Ptr<T> into Option<&T>
    fn as_option(&self) -> Option<&T>;
}

pub(crate) trait IntoOption<T> {
    fn into_option(self) -> Option<T>
    where
        Self: Sized;
}
