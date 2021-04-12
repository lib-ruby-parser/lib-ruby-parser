#[cfg(not(feature = "c-structures"))]
pub(crate) mod rust {
    /// Rust-compatible not-null pointer
    pub type Ptr<T> = Box<T>;

    use super::IntoMaybePtr;
    impl<T> IntoMaybePtr<T> for Ptr<T> {
        fn into_maybe_ptr(self) -> crate::containers::MaybePtr<T> {
            Some(self)
        }
    }

    use super::UnPtr;
    impl<T> UnPtr<T> for Ptr<T> {
        fn unptr(self) -> T {
            *self
        }
    }
}

#[cfg(feature = "c-structures")]
pub(crate) mod c {
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
            PartialEq::eq(&**self, &**other)
        }
    }

    impl<T> Clone for Ptr<T>
    where
        T: Clone,
    {
        fn clone(&self) -> Self {
            let value = self.as_ref().clone();
            Self::new(value)
        }
    }

    impl<T> Deref for Ptr<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.ptr }
        }
    }

    impl<T> AsRef<T> for Ptr<T> {
        fn as_ref(&self) -> &T {
            unsafe { &*self.ptr }
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
        /// Constructs a pointer with a given value
        pub fn new(t: T) -> Self {
            let ptr = Box::into_raw(Box::new(t));
            Self { ptr }
        }
    }

    impl<T> From<Box<T>> for Ptr<T> {
        fn from(boxed: Box<T>) -> Self {
            let value = *boxed;
            Self::new(value)
        }
    }

    use super::UnPtr;
    impl<T: Sized> UnPtr<T> for Ptr<T> {
        fn unptr(self) -> T
        where
            Self: Sized,
        {
            unsafe { self.ptr.read() }
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

pub(crate) trait UnPtr<T: Sized> {
    fn unptr(self) -> T
    where
        Self: Sized;
}
