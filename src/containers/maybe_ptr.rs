use crate::containers::deleter::{Deleter, GetDeleter};

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
    use std::ffi::c_void;

    use super::{Deleter, GetDeleter};
    use crate::containers::Ptr;

    type MaybePtrBlob = u64;

    /// C-compatible nullable pointer
    #[repr(C)]
    pub struct MaybePtr<T: GetDeleter> {
        ptr_blob: MaybePtrBlob,
        _t: std::marker::PhantomData<T>,
    }

    impl<T: GetDeleter> Drop for MaybePtr<T> {
        fn drop(&mut self) {
            let ptr = unsafe {
                lib_ruby_parser_containers_raw_ptr_from_maybe_ptr_blob(self.ptr_blob) as *mut T
            };
            if ptr.is_null() {
                return;
            }

            // 1. propagate Drop
            unsafe { std::ptr::drop_in_place(ptr) };
            // 2. call free on allocated data
            let deleter = T::get_deleter();
            unsafe { lib_ruby_parser_containers_free_maybe_ptr_blob(self.ptr_blob, deleter) };
            // 3. nullify ptr_blob
            self.ptr_blob = unsafe { lib_ruby_parser_containers_null_maybe_ptr_blob() };
        }
    }

    impl<T> std::fmt::Debug for MaybePtr<T>
    where
        T: std::fmt::Debug + GetDeleter,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.as_option(), f)
        }
    }

    impl<T> PartialEq for MaybePtr<T>
    where
        T: PartialEq + GetDeleter,
    {
        fn eq(&self, other: &Self) -> bool {
            PartialEq::eq(&self.as_option(), &other.as_option())
        }
    }

    impl<T> Clone for MaybePtr<T>
    where
        T: Clone + GetDeleter,
    {
        fn clone(&self) -> Self {
            match self.as_option() {
                Some(value) => Self::some(value.clone()),
                None => Self::none(),
            }
        }
    }

    use super::MaybePtrSome;
    impl<T> MaybePtrSome<T> for MaybePtr<T>
    where
        T: GetDeleter,
    {
        fn some(value: T) -> Self {
            let ptr = Box::into_raw(Box::new(value));
            Self::from_raw(ptr)
        }
    }

    use super::MaybePtrNone;
    impl<T> MaybePtrNone<T> for MaybePtr<T>
    where
        T: GetDeleter,
    {
        fn none() -> Self {
            Self::from_raw(std::ptr::null_mut())
        }
    }

    extern "C" {
        fn lib_ruby_parser_containers_make_maybe_ptr_blob(ptr: *mut c_void) -> MaybePtrBlob;
        fn lib_ruby_parser_containers_free_maybe_ptr_blob(blob: MaybePtrBlob, deleter: Deleter);
        fn lib_ruby_parser_containers_raw_ptr_from_maybe_ptr_blob(
            blob: MaybePtrBlob,
        ) -> *mut c_void;
        fn lib_ruby_parser_containers_null_maybe_ptr_blob() -> MaybePtrBlob;
    }

    impl<T> MaybePtr<T>
    where
        T: GetDeleter,
    {
        /// Constructs a pointer with a given raw pointer
        pub fn from_raw(ptr: *mut T) -> Self {
            let ptr_blob =
                unsafe { lib_ruby_parser_containers_make_maybe_ptr_blob(ptr as *mut c_void) };
            Self {
                ptr_blob,
                _t: std::marker::PhantomData,
            }
        }

        /// Returns borrowed raw pointer stored in MaybePtr
        pub fn as_ptr(&self) -> *const T {
            unsafe {
                lib_ruby_parser_containers_raw_ptr_from_maybe_ptr_blob(self.ptr_blob) as *const T
            }
        }

        /// Converts self into raw pointer
        pub fn into_raw(mut self) -> *mut T {
            let ptr =
                unsafe { lib_ruby_parser_containers_raw_ptr_from_maybe_ptr_blob(self.ptr_blob) }
                    as *mut T;
            self.ptr_blob = unsafe { lib_ruby_parser_containers_null_maybe_ptr_blob() };
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
            if self.as_ptr().is_null() {
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

    impl<T> From<Option<Box<T>>> for MaybePtr<T>
    where
        T: GetDeleter,
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
        T: GetDeleter,
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

    use super::AsOption;
    impl<T> AsOption<T> for MaybePtr<T>
    where
        T: GetDeleter,
    {
        fn as_option(&self) -> Option<&T> {
            unsafe { self.as_ptr().as_ref() }
        }
    }

    use super::IntoOption;
    impl<T> IntoOption<T> for MaybePtr<T>
    where
        T: Clone + GetDeleter,
    {
        fn into_option(self) -> Option<T> {
            self.as_option().map(|t| t.clone())
        }
    }

    impl<T> Default for MaybePtr<T>
    where
        T: GetDeleter,
    {
        fn default() -> Self {
            Self::none()
        }
    }

    #[cfg(test)]
    mod test {
        use super::{AsOption, GetDeleter, MaybePtr, MaybePtrBlob, MaybePtrNone, MaybePtrSome};

        #[test]
        fn test_size() {
            assert_eq!(std::mem::size_of::<MaybePtrBlob>(), 8);
        }

        #[derive(Debug, PartialEq)]
        struct Foo {
            bar: i32,
        }

        extern "C" fn lib_ruby_parser_containers_maybe_ptr_free_foo(ptr: *mut std::ffi::c_void) {
            drop(unsafe { Box::from_raw(ptr) })
        }

        impl GetDeleter for Foo {
            fn get_deleter() -> crate::containers::deleter::Deleter {
                lib_ruby_parser_containers_maybe_ptr_free_foo
            }
        }

        #[test]
        fn test_maybe_ptr() {
            let ptr = MaybePtr::some(Foo { bar: 42 });
            assert_eq!(ptr.as_option(), Some(&Foo { bar: 42 }));

            let ptr: MaybePtr<Foo> = MaybePtr::none();
            assert_eq!(ptr.as_option(), None);
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
