#[cfg(not(feature = "compile-with-external-structures"))]
pub(crate) mod rust {
    /// Rust-compatible shared list
    pub type SharedList<'a, T> = &'a [T];
}

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod c {
    use std::ops::Deref;

    /// C-compatible shared list
    #[repr(C)]
    pub struct SharedList<T> {
        ptr: *mut T,
        len: usize,
    }

    impl<T> std::fmt::Debug for SharedList<T>
    where
        T: std::fmt::Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&**self, f)
        }
    }

    impl<T> Deref for SharedList<T> {
        type Target = [T];

        fn deref(&self) -> &[T] {
            unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
        }
    }

    impl<T> SharedList<T> {
        pub(crate) fn from_raw(ptr: *mut T, len: usize) -> Self {
            Self { ptr, len }
        }
    }
}
