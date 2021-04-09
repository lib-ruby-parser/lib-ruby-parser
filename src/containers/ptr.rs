#[cfg(not(feature = "c-structures"))]
pub mod rust {
    /// Rust-compatible not-null pointer
    pub type Ptr<T> = Box<T>;

    use super::IntoMaybePtr;
    impl<T> IntoMaybePtr<T> for Ptr<T> {
        fn into_maybe_ptr(self) -> crate::containers::MaybePtr<T>
        where
            Self: Sized,
        {
            Some(self)
        }
    }
}

#[cfg(feature = "c-structures")]
pub mod c {
    use std::ops::Deref;

    /// C-compatible not-null pointer
    #[derive(Debug)]
    #[repr(C)]
    pub struct Ptr<T> {
        ptr: *mut T,
    }

    impl<T> PartialEq for Ptr<T>
    where
        T: PartialEq,
    {
        fn eq(&self, other: &Self) -> bool {
            todo!()
        }
    }

    impl<T> Clone for Ptr<T>
    where
        T: Clone,
    {
        fn clone(&self) -> Self {
            todo!()
        }
    }

    impl<T> Deref for Ptr<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            todo!()
        }
    }

    impl<T> AsRef<T> for Ptr<T> {
        fn as_ref(&self) -> &T {
            todo!()
        }
    }

    use super::IntoMaybePtr;
    impl<T> IntoMaybePtr<T> for Ptr<T> {
        fn into_maybe_ptr(self) -> crate::containers::MaybePtr<T>
        where
            Self: Sized,
        {
            crate::containers::MaybePtr::new(self.ptr)
        }
    }

    impl<T> Ptr<T> {
        pub fn new(t: T) -> Self {
            let ptr = Box::into_raw(Box::new(t));
            Self { ptr }
        }
    }

    impl<T> From<Box<T>> for Ptr<T> {
        fn from(_: Box<T>) -> Self {
            todo!()
        }
    }

    use super::UnwrapPtr;
    impl<T> UnwrapPtr<T> for Ptr<T> {
        fn unwrap_ptr(self) -> T
        where
            Self: Sized,
        {
            todo!()
        }
    }
}

/// Unwraps the pointer and returns stack value
pub trait IntoMaybePtr<T> {
    /// Unwraps the pointer and returns stack value
    fn into_maybe_ptr(self) -> crate::containers::MaybePtr<T>
    where
        Self: Sized;
}

pub(crate) trait UnwrapPtr<T> {
    fn unwrap_ptr(self) -> T
    where
        Self: Sized;
}
