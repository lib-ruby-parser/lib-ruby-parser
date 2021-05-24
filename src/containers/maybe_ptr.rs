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
}

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod external {
    use std::ffi::c_void;

    use crate::containers::get_drop_fn::{DropPtrFn, GetDropFn};
    use crate::containers::ExternalPtr;

    use crate::containers::size::MAYBE_PTR_SIZE;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub(crate) struct MaybePtrBlob {
        blob: [u8; MAYBE_PTR_SIZE],
    }

    /// C-compatible nullable pointer
    #[repr(C)]
    pub struct MaybePtr<T: GetDropFn> {
        blob: MaybePtrBlob,
        _t: std::marker::PhantomData<T>,
    }

    impl<T: GetDropFn> Drop for MaybePtr<T> {
        fn drop(&mut self) {
            let drop_item_in_place = T::get_drop_ptr_in_place_fn();
            unsafe {
                lib_ruby_parser_containers_free_maybe_ptr_blob(self.blob, drop_item_in_place)
            };
            self.blob = unsafe { lib_ruby_parser_containers_null_maybe_ptr_blob() };
        }
    }

    impl<T> std::fmt::Debug for MaybePtr<T>
    where
        T: std::fmt::Debug + GetDropFn,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.as_ref(), f)
        }
    }

    impl<T> PartialEq for MaybePtr<T>
    where
        T: PartialEq + GetDropFn,
    {
        fn eq(&self, other: &Self) -> bool {
            PartialEq::eq(&self.as_ref(), &other.as_ref())
        }
    }

    impl<T> Clone for MaybePtr<T>
    where
        T: Clone + GetDropFn,
    {
        fn clone(&self) -> Self {
            match self.as_ref() {
                Some(value) => Self::some(value.clone()),
                None => Self::none(),
            }
        }
    }

    use super::MaybePtrSome;
    impl<T> MaybePtrSome<T> for MaybePtr<T>
    where
        T: GetDropFn,
    {
        fn some(value: T) -> Self {
            let ptr = Box::into_raw(Box::new(value));
            Self::from_raw(ptr)
        }
    }

    use super::MaybePtrNone;
    impl<T> MaybePtrNone<T> for MaybePtr<T>
    where
        T: GetDropFn,
    {
        fn none() -> Self {
            Self::from_raw(std::ptr::null_mut())
        }
    }

    extern "C" {
        fn lib_ruby_parser_containers_make_maybe_ptr_blob(ptr: *mut c_void) -> MaybePtrBlob;
        fn lib_ruby_parser_containers_free_maybe_ptr_blob(blob: MaybePtrBlob, deleter: DropPtrFn);
        fn lib_ruby_parser_containers_raw_ptr_from_maybe_ptr_blob(
            blob: MaybePtrBlob,
        ) -> *mut c_void;
        fn lib_ruby_parser_containers_null_maybe_ptr_blob() -> MaybePtrBlob;
    }

    impl<T> MaybePtr<T>
    where
        T: GetDropFn,
    {
        /// Constructs a pointer with a given raw pointer
        pub fn from_raw(ptr: *mut T) -> Self {
            let blob =
                unsafe { lib_ruby_parser_containers_make_maybe_ptr_blob(ptr as *mut c_void) };
            Self {
                blob,
                _t: std::marker::PhantomData,
            }
        }

        /// Returns borrowed raw pointer stored in MaybePtr
        pub(crate) fn as_ptr(&self) -> *const T {
            unsafe { lib_ruby_parser_containers_raw_ptr_from_maybe_ptr_blob(self.blob) as *const T }
        }

        /// Converts self into raw pointer
        pub fn into_raw(mut self) -> *mut T {
            let ptr = unsafe { lib_ruby_parser_containers_raw_ptr_from_maybe_ptr_blob(self.blob) }
                as *mut T;
            self.blob = unsafe { lib_ruby_parser_containers_null_maybe_ptr_blob() };
            ptr
        }

        /// Equivalent of Option::or_else
        pub fn or_else<F>(self, f: F) -> Self
        where
            F: FnOnce() -> Self,
        {
            if self.as_ptr().is_null() {
                f()
            } else {
                self
            }
        }

        /// Equivalent of Option::expect
        pub fn expect(self, message: &str) -> ExternalPtr<T>
        where
            T: std::fmt::Debug + GetDropFn,
        {
            let ptr = self.into_raw();
            if ptr.is_null() {
                panic!("MaybePtr::expect failed {:?}", message)
            } else {
                ExternalPtr::from_raw(ptr)
            }
        }

        /// Equivalent of Option::map
        pub fn map<F>(self, f: F) -> Self
        where
            T: std::fmt::Debug + GetDropFn,
            F: FnOnce(ExternalPtr<T>) -> ExternalPtr<T>,
        {
            if self.as_ptr().is_null() {
                self
            } else {
                let ptr = self.into_raw();
                let ptr = ExternalPtr::from_raw(ptr);
                let ptr = f(ptr);
                let ptr = ptr.into_raw();
                Self::from_raw(ptr)
            }
        }

        /// Helper for swapping &mut self with Self::default()
        pub fn take(&mut self) -> Self {
            std::mem::take(self)
        }

        /// Equivalent of Option::unwrap
        pub fn unwrap(self) -> ExternalPtr<T> {
            if self.as_ptr().is_null() {
                panic!("unwrapping MaybePtr::None");
            } else {
                let ptr = self.into_raw();
                ExternalPtr::from_raw(ptr)
            }
        }

        /// Equivalent of Option::as_ref
        pub fn as_ref(&self) -> Option<&T> {
            let ptr = self.as_ptr();
            if ptr.is_null() {
                None
            } else {
                Some(unsafe { &*ptr })
            }
        }
    }

    impl<T> From<Option<Box<T>>> for MaybePtr<T>
    where
        T: GetDropFn,
    {
        fn from(maybe_boxed: Option<Box<T>>) -> Self {
            match maybe_boxed {
                Some(boxed) => Self::from_raw(Box::into_raw(boxed)),
                None => Self::none(),
            }
        }
    }

    impl<T> From<MaybePtr<T>> for Option<Box<T>>
    where
        T: GetDropFn,
    {
        fn from(ptr: MaybePtr<T>) -> Self {
            let ptr = ptr.into_raw();
            if ptr.is_null() {
                None
            } else {
                Some(unsafe { Box::from_raw(ptr) })
            }
        }
    }

    impl<T: GetDropFn> From<ExternalPtr<T>> for MaybePtr<T> {
        fn from(ptr: ExternalPtr<T>) -> Self {
            let ptr = ptr.into_raw();
            Self::from_raw(ptr)
        }
    }

    impl<T> Default for MaybePtr<T>
    where
        T: GetDropFn,
    {
        fn default() -> Self {
            Self::none()
        }
    }

    #[cfg(test)]
    mod test {
        use super::{
            GetDropFn, MaybePtr, MaybePtrBlob, MaybePtrNone, MaybePtrSome, MAYBE_PTR_SIZE,
        };

        #[test]
        fn test_size() {
            assert_eq!(std::mem::size_of::<MaybePtrBlob>(), MAYBE_PTR_SIZE);
        }

        #[derive(Debug, PartialEq)]
        struct Foo {
            bar: Vec<i32>,
        }

        extern "C" fn drop_in_place_foo(ptr: *mut std::ffi::c_void) {
            unsafe { std::ptr::drop_in_place(ptr as *mut Foo) }
        }

        impl GetDropFn for Foo {
            fn get_drop_ptr_fn() -> crate::containers::get_drop_fn::DropPtrFn {
                unreachable!()
            }

            fn get_drop_ptr_in_place_fn() -> crate::containers::get_drop_fn::DropInPlaceFn {
                drop_in_place_foo
            }

            fn get_drop_list_blob_fn() -> crate::containers::get_drop_fn::DropListBlobFn {
                unreachable!()
            }
        }

        #[test]
        fn test_maybe_ptr() {
            let ptr = MaybePtr::some(Foo { bar: vec![42] });
            assert_eq!(ptr.as_ref(), Some(&Foo { bar: vec![42] }));

            let ptr: MaybePtr<Foo> = MaybePtr::none();
            assert_eq!(ptr.as_ref(), None);
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
