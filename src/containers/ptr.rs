#[cfg(feature = "compile-with-external-structures")]
use crate::containers::get_drop_fn::GetFreePtrFn;

#[cfg(not(feature = "compile-with-external-structures"))]
pub(crate) mod rust {
    /// Rust-compatible not-null pointer
    pub type Ptr<T> = Box<T>;

    use super::UnPtr;
    impl<T> UnPtr<T> for Ptr<T> {
        fn unptr(self) -> T {
            *self
        }
    }
}

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod external {
    use super::GetFreePtrFn;

    // use crate::containers::deleter::{Deleter, GetDeleter};
    use std::ops::Deref;
    use std::{ffi::c_void, ops::DerefMut};

    use crate::containers::size::PTR_SIZE;

    /// PtrBlob, exposed only because it's used in some pub trait
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct PtrBlob {
        blob: [u8; PTR_SIZE],
    }

    /// C-compatible not-null pointer
    #[repr(C)]
    pub struct Ptr<T: GetFreePtrFn> {
        pub(crate) blob: PtrBlob,
        _t: std::marker::PhantomData<T>,
    }

    impl<T: GetFreePtrFn> Drop for Ptr<T> {
        fn drop(&mut self) {
            let free_ptr_fn = T::get_free_ptr_fn();
            let blob_ptr: *mut PtrBlob = &mut self.blob;
            unsafe { free_ptr_fn(blob_ptr) };
        }
    }

    impl<T> std::fmt::Debug for Ptr<T>
    where
        T: std::fmt::Debug + GetFreePtrFn,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&**self, f)
        }
    }

    impl<T> PartialEq for Ptr<T>
    where
        T: PartialEq + GetFreePtrFn,
    {
        fn eq(&self, other: &Self) -> bool {
            PartialEq::eq(self.as_ref(), other.as_ref())
        }
    }

    impl<T> Clone for Ptr<T>
    where
        T: Clone + GetFreePtrFn,
    {
        fn clone(&self) -> Self {
            let value = self.as_ref().clone();
            Self::new(value)
        }
    }

    impl<T: GetFreePtrFn> Deref for Ptr<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.as_ptr() }
        }
    }

    impl<T: GetFreePtrFn> DerefMut for Ptr<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { &mut *self.as_mut_ptr() }
        }
    }

    impl<T: GetFreePtrFn> AsRef<T> for Ptr<T> {
        fn as_ref(&self) -> &T {
            unsafe { self.as_ptr().as_ref().unwrap() }
        }
    }

    impl<T: GetFreePtrFn> AsMut<T> for Ptr<T> {
        fn as_mut(&mut self) -> &mut T {
            unsafe { self.as_mut_ptr().as_mut().unwrap() }
        }
    }

    extern "C" {
        fn lib_ruby_parser__internal__containers__ptr__new(ptr: *mut c_void) -> PtrBlob;
        fn lib_ruby_parser__internal__containers__ptr__get_raw(
            ptr_blob: *mut PtrBlob,
        ) -> *mut c_void;
    }

    impl<T: GetFreePtrFn> Ptr<T> {
        /// Constructs a pointer with a given value
        pub fn new(t: T) -> Self {
            let ptr = Box::into_raw(Box::new(t));
            Self::from_raw(ptr)
        }

        /// Constructs a pointer from a given raw pointer
        pub(crate) fn from_raw(ptr: *mut T) -> Self {
            debug_assert!(!ptr.is_null());
            let blob =
                unsafe { lib_ruby_parser__internal__containers__ptr__new(ptr as *mut c_void) };
            Self {
                blob,
                _t: std::marker::PhantomData,
            }
        }

        /// Converts self into raw pointer
        pub(crate) fn into_raw(mut self) -> *mut T {
            let ptr = self.as_mut_ptr();
            std::mem::forget(self);
            ptr
        }

        /// Returns borrowed raw pointer stored in Ptr
        pub(crate) fn as_ptr(&self) -> *const T {
            let ptr_blob: *const PtrBlob = &self.blob;
            unsafe {
                lib_ruby_parser__internal__containers__ptr__get_raw(ptr_blob as *mut PtrBlob)
                    as *const T
            }
        }

        pub(crate) fn as_mut_ptr(&mut self) -> *mut T {
            let ptr_blob: *mut PtrBlob = &mut self.blob;
            unsafe { lib_ruby_parser__internal__containers__ptr__get_raw(ptr_blob) as *mut T }
        }
    }

    impl<T: GetFreePtrFn> From<Box<T>> for Ptr<T> {
        fn from(boxed: Box<T>) -> Self {
            Self::from_raw(Box::into_raw(boxed))
        }
    }

    use super::UnPtr;
    impl<T: Sized + GetFreePtrFn> UnPtr<T> for Ptr<T> {
        fn unptr(self) -> T {
            *unsafe { Box::from_raw(self.into_raw()) }
        }
    }

    #[cfg(test)]
    mod tests {
        use std::ffi::c_void;

        use super::{GetFreePtrFn, Ptr, PtrBlob, UnPtr};

        #[derive(Debug, PartialEq)]
        struct Foo {
            bar: Vec<i32>,
        }

        extern "C" fn drop_ptr_of_foo(ptr: *mut PtrBlob) {
            let ptr = unsafe { *(ptr as *mut *mut Foo) };
            unsafe {
                std::ptr::drop_in_place(ptr);
                drop(Box::from_raw(ptr as *mut c_void))
            }
        }

        impl GetFreePtrFn for Foo {
            fn get_free_ptr_fn() -> unsafe extern "C" fn(*mut PtrBlob) {
                drop_ptr_of_foo
            }
        }

        #[test]
        fn test_ptr() {
            let foo = Foo { bar: vec![42] };
            let raw: *mut Foo = Box::leak(Box::new(foo));
            let ptr = Ptr::from_raw(raw);

            assert_eq!(ptr.as_ref(), &Foo { bar: vec![42] });
        }

        #[test]
        fn test_unptr() {
            let ptr = Ptr::from_raw(Box::leak(Box::new(Foo { bar: vec![42] })));
            assert_eq!(ptr.unptr(), Foo { bar: vec![42] })
        }
    }
}

pub(crate) trait UnPtr<T: Sized> {
    fn unptr(self) -> T
    where
        Self: Sized;
}
