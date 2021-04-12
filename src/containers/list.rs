#[cfg(not(feature = "c-structures"))]
pub(crate) mod rust {
    /// Rust-compatible list
    pub type List<T> = Vec<T>;

    use super::TakeFirst;
    impl<T> TakeFirst<T> for List<T> {
        fn take_first(self) -> T {
            self.into_iter()
                .next()
                .expect("expected at least 1 element")
        }
    }
}

#[cfg(feature = "c-structures")]
pub(crate) mod c {
    /// C-compatible list
    #[repr(C)]
    pub struct List<T> {
        ptr: *mut T,
        len: usize,
        capacity: usize,
    }

    impl<T> std::fmt::Debug for List<T>
    where
        T: std::fmt::Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&**self, f)
        }
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
            let copied = self.as_ref().iter().map(|e| e.clone()).collect::<Vec<_>>();
            Self::from(copied)
        }
    }

    impl<T> From<Vec<T>> for List<T>
    where
        T: Clone,
    {
        fn from(mut vec: Vec<T>) -> Self {
            let ptr = vec.as_mut_ptr();
            let len = vec.len();
            let capacity = vec.capacity();
            std::mem::forget(vec);
            Self { ptr, len, capacity }
        }
    }

    impl<T> From<List<T>> for Vec<T>
    where
        T: Clone,
    {
        fn from(list: List<T>) -> Self {
            unsafe { Vec::from_raw_parts(list.ptr, list.len, list.capacity) }
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
            self.as_ref().iter()
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
            if self.capacity == 0 {
                self.capacity = 1;
            } else {
                self.capacity *= 2;
            }
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
                self.len += 1;
            }
        }

        /// Equivalent of Vec::remove
        pub fn remove(&mut self, index: usize) -> T {
            if index > self.len {
                panic!("can't remove index {}, len is {}", index, self.len)
            }
            unsafe {
                let ptr = self.ptr.add(index);
                let result = ptr.read();
                std::ptr::copy(ptr.offset(1), ptr, self.len - index - 1);
                self.len -= 1;
                result
            }
        }
    }

    impl<T> std::ops::Deref for List<T> {
        type Target = [T];

        fn deref(&self) -> &[T] {
            unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
        }
    }

    impl<T> std::ops::DerefMut for List<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
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
            Self::new()
        }
    }

    use std::alloc::GlobalAlloc;

    use super::TakeFirst;
    impl<T> TakeFirst<T> for List<T> {
        fn take_first(mut self) -> T {
            if self.is_empty() {
                panic!("can't get the first item from an empty list")
            } else {
                let result = unsafe { self.ptr.read() };
                self.ptr = unsafe { self.ptr.add(std::mem::size_of::<T>()) };
                result
            }
        }
    }
}

pub(crate) trait TakeFirst<T> {
    fn take_first(self) -> T;
}
