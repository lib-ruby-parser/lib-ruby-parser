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
}

#[cfg(feature = "c-structures")]
pub(crate) mod c {
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
            PartialEq::eq(&**self, &**other)
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
            // if self.ptr.is_null() {
            //     &None
            // } else {
            //     let value = unsafe { *self.ptr };
            //     &Some(value)
            // }
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

    // impl<T> MaybePtr<T> {
    //     fn unwrap_or_else<F>(self, _f: F) -> crate::containers::Ptr<T>
    //     where
    //         F: FnOnce() -> crate::containers::Ptr<T>,
    //         Self: Sized,
    //     {
    //         todo!()
    //     }
    // }

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

    // use super::IntoPtr;
    // impl<T> MaybePtr<T> {
    //     fn expect(self, _message: &str) -> crate::containers::Ptr<T> {
    //         todo!()
    //     }
    // }
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

// pub(crate) trait IntoPtr<T> {
//     fn into_ptr(self, message: &str) -> crate::containers::Ptr<T>;
// }
