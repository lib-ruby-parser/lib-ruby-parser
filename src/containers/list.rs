#[cfg(not(feature = "c-structures"))]
pub(crate) mod rust {
    /// Rust-compatible list
    pub type List<T> = Vec<T>;
}

#[cfg(feature = "c-structures")]
pub(crate) mod c {
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
            if self.len != other.len {
                return false;
            }
            for i in 0..self.len {
                if self[i] != other[i] {
                    return false;
                }
            }
            true
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
        /// Equivalent of Vec::new
        pub fn new() -> Self {
            Self {
                ptr: std::ptr::null_mut(),
                len: 0,
                capacity: 0,
            }
        }

        /// Equivalent of Vec::is_empty
        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        /// Equivalent of Vec::iter
        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            todo!()
        }

        /// Equivalent of Vec::with_capacity
        pub fn with_capacity(capacity: usize) -> Self {
            let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
            let ptr = unsafe { std::alloc::System.alloc(layout) } as *mut T;
            Self {
                ptr,
                len: 0,
                capacity,
            }
        }

        fn grow(&mut self) {
            self.capacity *= 2;
            let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
            self.ptr = unsafe { std::alloc::System.alloc(layout) } as *mut T;
        }

        /// Equivalent of Vec::push
        pub fn push(&mut self, item: T) {
            if self.len == self.capacity {
                self.grow()
            }
            unsafe {
                let end = self.ptr.add(self.len);
                end.write(item);
            }
        }

        /// Equivalent of Vec::last
        pub fn last(&self) -> Option<&T> {
            todo!()
        }

        /// Equivalent of Vec::remove
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

    impl<T> std::ops::DerefMut for List<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
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

    use std::alloc::GlobalAlloc;

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
