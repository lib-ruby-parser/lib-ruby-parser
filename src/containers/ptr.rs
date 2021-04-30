use crate::containers::get_drop_fn::GetDropFn;
use crate::containers::MaybePtr;

#[cfg(not(feature = "compile-with-external-structures"))]
pub(crate) mod rust {
    use super::GetDeleter;

    /// Rust-compatible not-null pointer
    pub type Ptr<T> = Box<T>;

    use super::IntoMaybePtr;
    impl<T: std::fmt::Debug + GetDeleter> IntoMaybePtr<T> for Ptr<T> {
        fn into_maybe_ptr(self) -> crate::containers::MaybePtr<T> {
            Some(self)
        }
    }

    use super::UnPtr;
    impl<T: GetDeleter> UnPtr<T> for Ptr<T> {
        fn unptr(self) -> T {
            *self
        }
    }
}

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod c {
    use super::{GetDropFn, MaybePtr};
    // use crate::containers::deleter::{Deleter, GetDeleter};
    use std::ffi::c_void;
    use std::ops::Deref;

    type PtrBlob = u64;

    /// C-compatible not-null pointer
    #[repr(C)]
    pub struct Ptr<T: GetDropFn> {
        blob: PtrBlob,
        _t: std::marker::PhantomData<T>,
    }

    impl<T: GetDropFn> Drop for Ptr<T> {
        fn drop(&mut self) {
            let ptr =
                unsafe { lib_ruby_parser_containers_raw_ptr_from_ptr_blob(self.blob) as *mut T };
            if ptr.is_null() {
                return;
            }

            // 1. propagate Drop
            unsafe { std::ptr::drop_in_place(ptr) };
            // 2. call free on allocated data
            let deleter = T::get_drop_ptr_fn();
            unsafe { lib_ruby_parser_containers_free_ptr_blob(self.blob, deleter) };
            // 3. nullify blob
            self.blob = unsafe { lib_ruby_parser_containers_null_ptr_blob() };
        }
    }

    impl<T> std::fmt::Debug for Ptr<T>
    where
        T: std::fmt::Debug + GetDropFn,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&**self, f)
        }
    }

    impl<T> PartialEq for Ptr<T>
    where
        T: PartialEq + GetDropFn,
    {
        fn eq(&self, other: &Self) -> bool {
            PartialEq::eq(self.as_ref(), other.as_ref())
        }
    }

    impl<T> Clone for Ptr<T>
    where
        T: Clone + GetDropFn,
    {
        fn clone(&self) -> Self {
            let value = self.as_ref().clone();
            Self::new(value)
        }
    }

    impl<T: GetDropFn> Deref for Ptr<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.as_ptr() }
        }
    }

    impl<T: GetDropFn> AsRef<T> for Ptr<T> {
        fn as_ref(&self) -> &T {
            unsafe { self.as_ptr().as_ref().unwrap() }
        }
    }

    use super::IntoMaybePtr;
    impl<T: GetDropFn> IntoMaybePtr<T> for Ptr<T> {
        fn into_maybe_ptr(self) -> MaybePtr<T> {
            MaybePtr::from_raw(self.into_raw())
        }
    }

    use crate::containers::get_drop_fn::DropPtrFn;
    extern "C" {
        fn lib_ruby_parser_containers_make_ptr_blob(ptr: *mut c_void) -> PtrBlob;
        fn lib_ruby_parser_containers_free_ptr_blob(ptr: PtrBlob, deleter: DropPtrFn);
        fn lib_ruby_parser_containers_raw_ptr_from_ptr_blob(ptr: PtrBlob) -> *mut c_void;
        fn lib_ruby_parser_containers_null_ptr_blob() -> PtrBlob;
    }

    impl<T: GetDropFn> Ptr<T> {
        /// Constructs a pointer with a given value
        pub fn new(t: T) -> Self {
            let ptr = Box::into_raw(Box::new(t));
            Self::from_raw(ptr)
        }

        /// Constructs a pointer from a given raw pointer
        pub fn from_raw(ptr: *mut T) -> Self {
            debug_assert!(!ptr.is_null());
            let blob = unsafe { lib_ruby_parser_containers_make_ptr_blob(ptr as *mut c_void) };
            Self {
                blob,
                _t: std::marker::PhantomData,
            }
        }

        /// Converts self into raw pointer
        pub fn into_raw(mut self) -> *mut T {
            let ptr =
                unsafe { lib_ruby_parser_containers_raw_ptr_from_ptr_blob(self.blob) } as *mut T;
            self.blob = unsafe { lib_ruby_parser_containers_null_ptr_blob() };
            ptr
        }

        /// Returns borrowed raw pointer stored in Ptr
        pub fn as_ptr(&self) -> *const T {
            unsafe { lib_ruby_parser_containers_raw_ptr_from_ptr_blob(self.blob) as *const T }
        }
    }

    impl<T: GetDropFn> From<Box<T>> for Ptr<T> {
        fn from(boxed: Box<T>) -> Self {
            Self::from_raw(Box::into_raw(boxed))
        }
    }

    use super::UnPtr;
    impl<T: Sized + GetDropFn> UnPtr<T> for Ptr<T> {
        fn unptr(self) -> T {
            *unsafe { Box::from_raw(self.into_raw()) }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{DropPtrFn, GetDropFn, Ptr, PtrBlob, UnPtr};

        #[derive(Debug, PartialEq)]
        struct Foo {
            bar: i32,
        }

        extern "C" fn lib_ruby_parser_containers_ptr_delete_foo(ptr: *mut std::ffi::c_void) {
            println!("Running foreign Foo deleter on {:?}", unsafe {
                &*(ptr as *mut Foo)
            });
            drop(unsafe { Box::from_raw(ptr as *mut Foo) })
        }

        impl GetDropFn for Foo {
            fn get_drop_ptr_fn() -> DropPtrFn {
                lib_ruby_parser_containers_ptr_delete_foo
            }

            fn get_drop_in_place_fn() -> crate::containers::get_drop_fn::DropInPlaceFn {
                unreachable!()
            }

            fn get_drop_list_blob_fn() -> crate::containers::get_drop_fn::DropListBlobFn {
                unreachable!()
            }
        }

        #[test]
        fn test_size() {
            assert_eq!(std::mem::size_of::<PtrBlob>(), 8);
        }

        #[test]
        fn test_ptr() {
            let ptr = Ptr::from_raw(Box::leak(Box::new(Foo { bar: 42 })));

            assert_eq!(ptr.as_ref(), &Foo { bar: 42 });
        }

        #[test]
        fn test_unptr() {
            let ptr = Ptr::from_raw(Box::leak(Box::new(Foo { bar: 42 })));
            assert_eq!(ptr.unptr(), Foo { bar: 42 })
        }
    }
}

/// Unwraps the pointer and returns stack value
pub trait IntoMaybePtr<T: GetDropFn> {
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
