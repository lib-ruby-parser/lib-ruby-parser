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
    use std::ops::Deref;

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
            todo!()
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

    impl<T> Deref for MaybePtr<T> {
        type Target = Option<T>;

        fn deref(&self) -> &Self::Target {
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
        pub fn new(ptr: *mut T) -> Self {
            Self { ptr }
        }

        pub fn or_else<F>(self, f: F) -> Self
        where
            F: FnOnce() -> Self,
        {
            todo!()
        }

        pub fn expect(message: &str) -> crate::containers::Ptr<T> {
            todo!()
        }

        pub fn map<Return, F>(self, f: F) -> Return
        where
            F: FnOnce(T) -> Return,
        {
            todo!()
        }
    }

    use super::IntoPtrOrElse;
    impl<T> IntoPtrOrElse<T> for MaybePtr<T> {
        fn into_ptr_or_else<F>(self, f: F) -> crate::containers::Ptr<T>
        where
            F: FnOnce() -> crate::containers::Ptr<T>,
            Self: Sized,
        {
            todo!()
        }
    }

    impl<T> From<Option<Box<T>>> for MaybePtr<T> {
        fn from(_: Option<Box<T>>) -> Self {
            todo!()
        }
    }

    use super::IntoPtr;
    impl<T> IntoPtr<T> for MaybePtr<T> {
        fn into_ptr(self, message: &str) -> crate::containers::Ptr<T> {
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

pub(crate) trait IntoPtrOrElse<T> {
    fn into_ptr_or_else<F>(self, f: F) -> crate::containers::Ptr<T>
    where
        F: FnOnce() -> crate::containers::Ptr<T>,
        Self: Sized;
}

pub(crate) trait IntoPtr<T> {
    fn into_ptr(self, message: &str) -> crate::containers::Ptr<T>;
}
