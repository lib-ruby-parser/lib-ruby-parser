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
    use crate::blobs::{HasBlob, PtrBlob};
    use std::ops::Deref;

    pub trait PtrValue {
        fn drop_fn() -> unsafe extern "C" fn(blob: *mut PtrBlob);
    }

    /// C-compatible not-null pointer
    #[repr(C)]
    pub struct Ptr<T>
    where
        T: PtrValue,
    {
        pub(crate) blob: PtrBlob,
        pub(crate) _t: std::marker::PhantomData<T>,
    }

    impl<T> Drop for Ptr<T>
    where
        T: PtrValue,
    {
        fn drop(&mut self) {
            let f = T::drop_fn();
            unsafe { f(&mut self.blob) };
        }
    }

    macro_rules! define_impl {
        ($t:ty, $drop:ident) => {
            extern "C" {
                fn $drop(ptr: *mut PtrBlob);
            }

            impl PtrValue for $t {
                fn drop_fn() -> unsafe extern "C" fn(*mut PtrBlob) {
                    $drop
                }
            }
        };
    }

    define_impl!(crate::Node, lib_ruby_parser__external__ptr__of_node__drop);
    define_impl!(crate::Token, lib_ruby_parser__external__ptr__of_token__drop);

    impl<T> std::fmt::Debug for Ptr<T>
    where
        T: std::fmt::Debug + PtrValue,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&**self, f)
        }
    }

    impl<T> PartialEq for Ptr<T>
    where
        T: PartialEq + PtrValue,
    {
        fn eq(&self, other: &Self) -> bool {
            PartialEq::eq(self.as_ref(), other.as_ref())
        }
    }

    impl<T> Eq for Ptr<T> where T: PartialEq + PtrValue {}

    impl<T> Clone for Ptr<T>
    where
        T: Clone + PtrValue,
    {
        fn clone(&self) -> Self {
            let value = self.as_ref().clone();
            Self::new(value)
        }
    }

    impl<T: PtrValue> Deref for Ptr<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.as_ptr() }
        }
    }

    impl<T: PtrValue> AsRef<T> for Ptr<T> {
        fn as_ref(&self) -> &T {
            unsafe { self.as_ptr().as_ref().unwrap() }
        }
    }

    extern "C" {
        fn lib_ruby_parser__external__ptr__new(ptr: *mut std::ffi::c_void) -> PtrBlob;
        fn lib_ruby_parser__external__ptr__get_raw(
            ptr_blob: *const PtrBlob,
        ) -> *const std::ffi::c_void;
        fn lib_ruby_parser__external__ptr__into_raw(ptr_blob: PtrBlob) -> *mut std::ffi::c_void;
    }

    impl<T: PtrValue> Ptr<T> {
        /// Constructs a pointer with a given value
        pub fn new(t: T) -> Self {
            let ptr = Box::into_raw(Box::new(t));
            Self::from_raw(ptr)
        }

        /// Constructs a pointer from a given raw pointer
        pub(crate) fn from_raw(ptr: *mut T) -> Self {
            debug_assert!(!ptr.is_null());
            let blob = unsafe { lib_ruby_parser__external__ptr__new(ptr as *mut std::ffi::c_void) };
            Self {
                blob,
                _t: std::marker::PhantomData,
            }
        }

        /// Converts self into raw pointer
        pub(crate) fn into_raw(self) -> *mut T {
            unsafe { lib_ruby_parser__external__ptr__into_raw(self.into_blob()) as *mut T }
        }

        /// Returns borrowed raw pointer stored in Ptr
        pub(crate) fn as_ptr(&self) -> *const T {
            unsafe { lib_ruby_parser__external__ptr__get_raw(&self.blob) as *const T }
        }
    }

    impl<T: PtrValue> From<Box<T>> for Ptr<T> {
        fn from(boxed: Box<T>) -> Self {
            Self::from_raw(Box::into_raw(boxed))
        }
    }

    use super::PtrAPI;
    impl<T: PtrValue> PtrAPI<T> for Ptr<T> {
        fn unptr(self) -> T {
            *unsafe { Box::from_raw(self.into_raw()) }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{Ptr, PtrAPI};
        use crate::{Node, Token};

        #[test]
        fn test_ptr_token() {
            fn make() -> Token {
                Token::new(
                    280,
                    crate::Bytes::new(crate::containers::ExternalList::from(vec![97, 98, 99])),
                    crate::Loc::new(3, 4),
                    crate::LexState { value: 1 },
                    crate::LexState { value: 2 },
                )
            }

            let ptr = Ptr::new(make());
            assert_eq!(ptr.as_ref(), &make());
            assert_eq!(ptr.clone(), Ptr::new(make()));
            assert_eq!(ptr.unptr(), make());
        }

        #[test]
        fn test_ptr_node() {
            fn make() -> Node {
                Node::new_file(crate::Loc::new(1, 2))
            }

            let ptr = Ptr::new(make());
            assert_eq!(ptr.as_ref(), &make());
            assert_eq!(ptr.clone(), Ptr::new(make()));
            assert_eq!(ptr.unptr(), make());
        }
    }
}

pub(crate) trait PtrAPI<T: Sized> {
    fn unptr(self) -> T
    where
        Self: Sized;
}
