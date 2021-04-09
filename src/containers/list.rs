#[cfg(not(feature = "c-structures"))]
pub mod rust {
    /// Rust-compatible list
    pub type List<T> = Vec<T>;
}

#[cfg(feature = "c-structures")]
pub mod c {
    /// C-compatible list
    #[derive(Debug)]
    #[repr(C)]
    pub struct List<T> {
        ptr: *mut T,
        len: usize,
        capacity: usize,
    }

    impl<T> PartialEq for List<T>
    where
        T: PartialEq,
    {
        fn eq(&self, other: &Self) -> bool {
            todo!()
        }
    }

    impl<T> Clone for List<T>
    where
        T: Clone,
    {
        fn clone(&self) -> Self {
            todo!()
        }
    }

    impl<T> List<T> {
        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            todo!()
        }
    }

    impl<T> From<Vec<T>> for List<T> {
        fn from(_: Vec<T>) -> Self {
            todo!()
        }
    }

    impl<T> From<List<T>> for Vec<T> {
        fn from(_: List<T>) -> Self {
            todo!()
        }
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            todo!()
        }

        pub fn is_empty(&self) -> bool {
            todo!()
        }

        pub fn with_capacity(capacity: usize) -> Self {
            todo!()
        }

        pub fn push(&mut self, item: T) {
            todo!()
        }

        pub fn last(&self) -> Option<&T> {
            todo!()
        }

        pub fn remove(&mut self, index: usize) -> T {
            todo!()
        }
    }

    impl<T> std::ops::Deref for List<T> {
        type Target = [T];

        fn deref(&self) -> &[T] {
            // unsafe { slice::from_raw_parts(self.as_ptr(), self.len) }
            todo!()
        }
    }

    impl<T, I: std::slice::SliceIndex<[T]>> std::ops::Index<I> for List<T> {
        type Output = I::Output;

        fn index(&self, index: I) -> &Self::Output {
            std::ops::Index::index(&**self, index)
        }
    }

    impl<T> Default for List<T> {
        fn default() -> Self {
            todo!()
        }
    }

    use super::TakeFirst;
    impl<T> TakeFirst<T> for List<T> {
        fn take_first(self) -> T {
            todo!()
        }
    }
}

pub(crate) trait TakeFirst<T> {
    fn take_first(self) -> T;
}
