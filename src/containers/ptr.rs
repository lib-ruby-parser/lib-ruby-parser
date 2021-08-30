#[cfg(not(feature = "compile-with-external-structures"))]
pub(crate) mod rust {
    /// Rust-compatible not-null pointer
    pub type Ptr<T> = Box<T>;

    use super::PtrAPI;
    impl<T> PtrAPI<T> for Ptr<T> {
        fn unptr(self) -> T {
            *self
        }
    }
}

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod external {
    use crate::blobs::PtrBlob;
    use crate::containers::get_drop_fn::GetDropPtrFn;
    use std::ops::Deref;
    use std::{ffi::c_void, ops::DerefMut};

    /// C-compatible not-null pointer
    #[repr(C)]
    pub struct Ptr<T: GetDropPtrFn> {
        pub(crate) blob: PtrBlob,
        _t: std::marker::PhantomData<T>,
    }

    impl<T: GetDropPtrFn> Drop for Ptr<T> {
        fn drop(&mut self) {
            let drop_ptr_fn = T::get_drop_ptr_fn();
            unsafe { drop_ptr_fn(&mut self.blob) };
        }
    }

    impl<T> std::fmt::Debug for Ptr<T>
    where
        T: std::fmt::Debug + GetDropPtrFn,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&**self, f)
        }
    }

    impl<T> PartialEq for Ptr<T>
    where
        T: PartialEq + GetDropPtrFn,
    {
        fn eq(&self, other: &Self) -> bool {
            PartialEq::eq(self.as_ref(), other.as_ref())
        }
    }

    impl<T> Eq for Ptr<T> where T: PartialEq + GetDropPtrFn {}

    impl<T> Clone for Ptr<T>
    where
        T: Clone + GetDropPtrFn,
    {
        fn clone(&self) -> Self {
            let value = self.as_ref().clone();
            Self::new(value)
        }
    }

    impl<T: GetDropPtrFn> Deref for Ptr<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.as_ptr() }
        }
    }

    impl<T: GetDropPtrFn> DerefMut for Ptr<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { &mut *self.as_mut_ptr() }
        }
    }

    impl<T: GetDropPtrFn> AsRef<T> for Ptr<T> {
        fn as_ref(&self) -> &T {
            unsafe { self.as_ptr().as_ref().unwrap() }
        }
    }

    impl<T: GetDropPtrFn> AsMut<T> for Ptr<T> {
        fn as_mut(&mut self) -> &mut T {
            unsafe { self.as_mut_ptr().as_mut().unwrap() }
        }
    }

    extern "C" {
        fn lib_ruby_parser__external__ptr__new(ptr: *mut c_void) -> PtrBlob;
        fn lib_ruby_parser__external__ptr__get_raw(ptr_blob: *mut PtrBlob) -> *mut c_void;
    }

    impl<T: GetDropPtrFn> Ptr<T> {
        /// Constructs a pointer with a given value
        pub fn new(t: T) -> Self {
            let ptr = Box::into_raw(Box::new(t));
            Self::from_raw(ptr)
        }

        /// Constructs a pointer from a given raw pointer
        pub(crate) fn from_raw(ptr: *mut T) -> Self {
            debug_assert!(!ptr.is_null());
            let blob = unsafe { lib_ruby_parser__external__ptr__new(ptr as *mut c_void) };
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
            unsafe { lib_ruby_parser__external__ptr__get_raw(ptr_blob as *mut PtrBlob) as *const T }
        }

        pub(crate) fn as_mut_ptr(&mut self) -> *mut T {
            let ptr_blob: *mut PtrBlob = &mut self.blob;
            unsafe { lib_ruby_parser__external__ptr__get_raw(ptr_blob) as *mut T }
        }
    }

    impl<T: GetDropPtrFn> From<Box<T>> for Ptr<T> {
        fn from(boxed: Box<T>) -> Self {
            Self::from_raw(Box::into_raw(boxed))
        }
    }

    use super::PtrAPI;
    impl<T: Sized + GetDropPtrFn> PtrAPI<T> for Ptr<T> {
        fn unptr(self) -> T {
            *unsafe { Box::from_raw(self.into_raw()) }
        }
    }

    #[cfg(test)]
    mod tests {
        use std::ffi::c_void;

        use super::{GetDropPtrFn, Ptr, PtrAPI, PtrBlob};

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

        impl GetDropPtrFn for Foo {
            fn get_drop_ptr_fn() -> unsafe extern "C" fn(*mut PtrBlob) {
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

pub(crate) trait PtrAPI<T: Sized> {
    fn unptr(self) -> T
    where
        Self: Sized;
}
