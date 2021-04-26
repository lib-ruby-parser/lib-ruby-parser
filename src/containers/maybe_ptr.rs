#[cfg(not(feature = "compile-with-external-structures"))]
pub(crate) mod rust {
    /// Rust-compatible nullable pointer
    pub type MaybePtr<T> = Option<Box<T>>;

    use super::MaybePtrSome;
    impl<T> MaybePtrSome<T> for MaybePtr<T> {
        fn some(value: T) -> Self {
            Some(Box::new(value))
        }
    }

    use super::MaybePtrNone;
    impl<T> MaybePtrNone<T> for MaybePtr<T> {
        fn none() -> Self {
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

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod c {
    use crate::containers::deleter::GetDeleter;
    use crate::containers::Ptr;

    /// C-compatible nullable pointer
    #[repr(C)]
    pub struct MaybePtr<T> {
        ptr: *mut T,
    }

    impl<T> Drop for MaybePtr<T> {
        fn drop(&mut self) {
            if self.ptr.is_null() {
                return;
            }

            drop(unsafe { Box::from_raw(self.ptr) });
            self.ptr = std::ptr::null_mut();
        }
    }

    impl<T> std::fmt::Debug for MaybePtr<T>
    where
        T: std::fmt::Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.as_option(), f)
        }
    }

    impl<T> PartialEq for MaybePtr<T>
    where
        T: PartialEq + std::fmt::Debug,
    {
        fn eq(&self, other: &Self) -> bool {
            PartialEq::eq(&self.as_option(), &other.as_option())
        }
    }

    impl<T> Clone for MaybePtr<T>
    where
        T: Clone + std::fmt::Debug,
    {
        fn clone(&self) -> Self {
            match self.as_option() {
                Some(value) => Self::some(value.clone()),
                None => Self::none(),
            }
        }
    }

    use super::MaybePtrSome;
    impl<T> MaybePtrSome<T> for MaybePtr<T> {
        fn some(value: T) -> Self {
            let ptr = Box::into_raw(Box::new(value));
            Self { ptr }
        }
    }

    use super::MaybePtrNone;
    impl<T> MaybePtrNone<T> for MaybePtr<T> {
        fn none() -> Self {
            Self {
                ptr: std::ptr::null_mut(),
            }
        }
    }

    impl<T> MaybePtr<T> {
        /// Constructs a pointer with a given raw pointer
        pub fn from_raw(ptr: *mut T) -> Self {
            Self { ptr }
        }

        /// Converts self into raw pointer
        pub fn into_raw(mut self) -> *mut T {
            let ptr = self.ptr;
            self.ptr = std::ptr::null_mut();
            ptr
        }

        /// Equivalent of Option::or_else
        pub fn or_else<F>(self, f: F) -> Self
        where
            F: FnOnce() -> Self,
        {
            if self.ptr.is_null() {
                f()
            } else {
                self
            }
        }

        /// Equivalent of Option::expect
        pub fn expect(self, message: &str) -> Ptr<T>
        where
            T: std::fmt::Debug + GetDeleter,
        {
            let ptr = self.into_raw();
            if ptr.is_null() {
                panic!("MaybePtr::expect failed {:?}", message)
            } else {
                Ptr::from_raw(ptr)
            }
        }

        /// Equivalent of Option::map
        pub fn map<F>(self, f: F) -> Self
        where
            T: std::fmt::Debug + GetDeleter,
            F: FnOnce(Ptr<T>) -> Ptr<T>,
        {
            if self.ptr.is_null() {
                self
            } else {
                let ptr = self.into_raw();
                let ptr = Ptr::from_raw(ptr);
                let ptr = f(ptr);
                let ptr = ptr.into_raw();
                Self::from_raw(ptr)
            }
        }

        /// Helper for swapping &mut self with Self::default()
        pub fn take(&mut self) -> Self {
            std::mem::take(self)
        }
    }

    impl<T> From<Option<Box<T>>> for MaybePtr<T> {
        fn from(maybe_boxed: Option<Box<T>>) -> Self {
            match maybe_boxed {
                Some(boxed) => Self::from_raw(Box::into_raw(boxed)),
                None => Self::none(),
            }
        }
    }

    impl<T> From<MaybePtr<T>> for Option<Box<T>> {
        fn from(ptr: MaybePtr<T>) -> Self {
            let ptr = ptr.into_raw();
            if ptr.is_null() {
                None
            } else {
                Some(unsafe { Box::from_raw(ptr) })
            }
        }
    }

    use super::AsOption;
    impl<T> AsOption<T> for MaybePtr<T> {
        fn as_option(&self) -> Option<&T> {
            unsafe { self.ptr.as_ref() }
        }
    }

    use super::IntoOption;
    impl<T> IntoOption<T> for MaybePtr<T>
    where
        T: Clone,
    {
        fn into_option(self) -> Option<T> {
            self.as_option().map(|t| t.clone())
        }
    }

    impl<T> Default for MaybePtr<T> {
        fn default() -> Self {
            Self::none()
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
