use crate::containers::MaybePtr;

#[cfg(not(feature = "c-structures"))]
pub(crate) mod rust {
    /// Rust-compatible not-null pointer
    pub type Ptr<T> = Box<T>;

    use super::IntoMaybePtr;
    impl<T: std::fmt::Debug> IntoMaybePtr<T> for Ptr<T> {
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
    use super::MaybePtr;
    use std::ops::Deref;

    /// C-compatible not-null pointer
    #[repr(C)]
    pub struct Ptr<T: std::fmt::Debug> {
        ptr: *mut T,
    }

    impl<T: std::fmt::Debug> Drop for Ptr<T> {
        fn drop(&mut self) {
            if self.ptr.is_null() {
                return;
            }
            println!("drop(Ptr {:?})", self);

            drop(unsafe { Box::from_raw(self.ptr) });
            self.ptr = std::ptr::null_mut();
        }
    }

    impl<T> std::fmt::Debug for Ptr<T>
    where
        T: std::fmt::Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // std::fmt::Debug::fmt(&**self, f)
            f.debug_struct("Ptr").field("ptr", &self.ptr).finish()
        }
    }

    impl<T> PartialEq for Ptr<T>
    where
        T: PartialEq + std::fmt::Debug,
    {
        fn eq(&self, other: &Self) -> bool {
            PartialEq::eq(self.as_ref(), other.as_ref())
        }
    }

    impl<T> Clone for Ptr<T>
    where
        T: Clone + std::fmt::Debug,
    {
        fn clone(&self) -> Self {
            let value = self.as_ref().clone();
            Self::new(value)
        }
    }

    impl<T: std::fmt::Debug> Deref for Ptr<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.ptr }
        }
    }

    impl<T: std::fmt::Debug> AsRef<T> for Ptr<T> {
        fn as_ref(&self) -> &T {
            unsafe { self.ptr.as_ref().unwrap() }
        }
    }

    use super::IntoMaybePtr;
    impl<T: std::fmt::Debug> IntoMaybePtr<T> for Ptr<T> {
        fn into_maybe_ptr(self) -> MaybePtr<T> {
            MaybePtr::from_raw(self.into_raw())
        }
    }

    impl<T: std::fmt::Debug> Ptr<T> {
        /// Constructs a pointer with a given value
        pub fn new(t: T) -> Self {
            let ptr = Box::into_raw(Box::new(t));
            Self::from_raw(ptr)
        }

        /// Constructs a pointer from a given raw pointer
        pub fn from_raw(ptr: *mut T) -> Self {
            debug_assert!(!ptr.is_null());
            println!("Ptr::from_raw({:?} == {:?})", ptr, unsafe { &*ptr });
            Self { ptr }
        }

        /// Converts self into raw pointer
        pub fn into_raw(mut self) -> *mut T {
            let ptr = self.ptr;
            self.ptr = std::ptr::null_mut();
            ptr
        }
    }

    impl<T: std::fmt::Debug> From<Box<T>> for Ptr<T> {
        fn from(boxed: Box<T>) -> Self {
            Self::from_raw(Box::into_raw(boxed))
        }
    }

    use super::UnPtr;
    impl<T: Sized + std::fmt::Debug> UnPtr<T> for Ptr<T> {
        fn unptr(self) -> T {
            *unsafe { Box::from_raw(self.into_raw()) }
        }
    }
}

/// Unwraps the pointer and returns stack value
pub trait IntoMaybePtr<T: std::fmt::Debug> {
    /// Unwraps the pointer and returns stack value
    fn into_maybe_ptr(self) -> MaybePtr<T>
    where
        Self: Sized;
}

pub(crate) trait UnPtr<T: Sized> {
    fn unptr(self) -> T
    where
        Self: Sized;
}
