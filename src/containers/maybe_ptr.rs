#[cfg(not(feature = "compile-with-external-structures"))]
pub(crate) mod rust {
    /// Rust-compatible nullable pointer
    pub type MaybePtr<T> = Option<Box<T>>;

    use super::MaybePtrAPI;
    impl<T> MaybePtrAPI<T> for MaybePtr<T> {
        fn some(value: T) -> Self {
            Some(Box::new(value))
        }

        fn none() -> Self {
            None
        }
    }
}

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod external {
    use crate::blobs::MaybePtrBlob;
    use crate::containers::get_drop_fn::{GetDropMaybePtrFn, GetDropPtrFn};
    use crate::containers::ExternalPtr;
    use std::ffi::c_void;

    /// C-compatible nullable pointer
    #[repr(C)]
    pub struct MaybePtr<T: GetDropMaybePtrFn> {
        pub(crate) blob: MaybePtrBlob,
        pub(crate) _t: std::marker::PhantomData<T>,
    }

    impl<T: GetDropMaybePtrFn> Drop for MaybePtr<T> {
        fn drop(&mut self) {
            let drop_maybe_ptr_pfn = T::get_drop_maybe_ptr_fn();
            unsafe { drop_maybe_ptr_pfn(&mut self.blob) };
        }
    }

    impl<T> std::fmt::Debug for MaybePtr<T>
    where
        T: std::fmt::Debug + GetDropMaybePtrFn,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.as_ref(), f)
        }
    }

    impl<T> PartialEq for MaybePtr<T>
    where
        T: PartialEq + GetDropMaybePtrFn,
    {
        fn eq(&self, other: &Self) -> bool {
            PartialEq::eq(&self.as_ref(), &other.as_ref())
        }
    }

    impl<T> Clone for MaybePtr<T>
    where
        T: Clone + GetDropMaybePtrFn,
    {
        fn clone(&self) -> Self {
            match self.as_ref() {
                Some(value) => Self::some(value.clone()),
                None => Self::none(),
            }
        }
    }

    use super::MaybePtrAPI;
    impl<T> MaybePtrAPI<T> for MaybePtr<T>
    where
        T: GetDropMaybePtrFn,
    {
        fn some(value: T) -> Self {
            let ptr = Box::into_raw(Box::new(value));
            Self::from_raw(ptr)
        }

        fn none() -> Self {
            let blob = unsafe { lib_ruby_parser__external__maybe_ptr__new_null() };
            Self {
                blob,
                _t: std::marker::PhantomData,
            }
        }
    }

    extern "C" {
        fn lib_ruby_parser__external__maybe_ptr__new(ptr: *mut c_void) -> MaybePtrBlob;
        fn lib_ruby_parser__external__maybe_ptr__new_null() -> MaybePtrBlob;
        fn lib_ruby_parser__external__maybe_ptr__get_raw(
            blob_ptr: *mut MaybePtrBlob,
        ) -> *mut c_void;
    }

    impl<T> MaybePtr<T>
    where
        T: GetDropMaybePtrFn,
    {
        /// Constructs a pointer with a given raw pointer
        pub fn from_raw(ptr: *mut T) -> Self {
            let blob = unsafe { lib_ruby_parser__external__maybe_ptr__new(ptr as *mut c_void) };
            Self {
                blob,
                _t: std::marker::PhantomData,
            }
        }

        /// Returns borrowed raw pointer stored in MaybePtr
        pub(crate) fn as_ptr(&self) -> *const T {
            let blob_ptr: *const MaybePtrBlob = &self.blob;
            unsafe {
                lib_ruby_parser__external__maybe_ptr__get_raw(blob_ptr as *mut MaybePtrBlob)
                    as *const T
            }
        }

        /// Returns borrowed raw pointer stored in MaybePtr
        pub(crate) fn as_mut_ptr(&mut self) -> *mut T {
            unsafe { lib_ruby_parser__external__maybe_ptr__get_raw(&mut self.blob) as *mut T }
        }

        /// Converts self into raw pointer
        pub fn into_raw(mut self) -> *mut T {
            let raw = self.as_mut_ptr();
            std::mem::forget(self);
            raw
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
            T: std::fmt::Debug + GetDropMaybePtrFn + GetDropPtrFn,
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
            T: std::fmt::Debug + GetDropMaybePtrFn + GetDropPtrFn,
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
        pub fn unwrap(self) -> ExternalPtr<T>
        where
            T: GetDropPtrFn,
        {
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

        /// Equivalent of Option::as_mut
        pub fn as_mut(&mut self) -> Option<&mut T> {
            let ptr = self.as_mut_ptr();
            if ptr.is_null() {
                return None;
            } else {
                Some(unsafe { &mut *ptr })
            }
        }
    }

    impl<T> From<Option<Box<T>>> for MaybePtr<T>
    where
        T: GetDropMaybePtrFn,
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
        T: GetDropMaybePtrFn,
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

    impl<T> From<ExternalPtr<T>> for MaybePtr<T>
    where
        T: GetDropMaybePtrFn + GetDropPtrFn,
    {
        fn from(ptr: ExternalPtr<T>) -> Self {
            let ptr = ptr.into_raw();
            Self::from_raw(ptr)
        }
    }

    impl<T> Default for MaybePtr<T>
    where
        T: GetDropMaybePtrFn,
    {
        fn default() -> Self {
            Self::none()
        }
    }

    #[cfg(test)]
    mod test {
        use super::{GetDropMaybePtrFn, MaybePtr, MaybePtrAPI, MaybePtrBlob};
        use std::ffi::c_void;

        #[derive(Debug, PartialEq)]
        struct Foo {
            bar: Vec<i32>,
        }

        extern "C" fn drop_maybe_ptr_of_foo(ptr: *mut MaybePtrBlob) {
            let ptr = unsafe { *(ptr as *mut *mut Foo) };
            if ptr.is_null() {
                return;
            }
            unsafe {
                std::ptr::drop_in_place(ptr);
                drop(Box::from_raw(ptr as *mut c_void))
            }
        }

        impl GetDropMaybePtrFn for Foo {
            fn get_drop_maybe_ptr_fn() -> unsafe extern "C" fn(*mut MaybePtrBlob) {
                drop_maybe_ptr_of_foo
            }
        }

        #[test]
        fn test_maybe_ptr() {
            let ptr = MaybePtr::some(Foo { bar: vec![42] });
            assert_eq!(ptr.as_ref(), Some(&Foo { bar: vec![42] }));
            assert_eq!(format!("{:?}", ptr), "Some(Foo { bar: [42] })");

            let ptr: MaybePtr<Foo> = MaybePtr::none();
            assert_eq!(ptr.as_ref(), None);
            assert_eq!(format!("{:?}", ptr), "None");

            let ptr: MaybePtr<crate::Node> = MaybePtr::none();
            assert_eq!(format!("{:?}", ptr), "None");
        }
    }
}

pub(crate) trait MaybePtrAPI<T> {
    fn some(value: T) -> Self
    where
        Self: Sized;

    fn none() -> Self
    where
        Self: Sized;
}
