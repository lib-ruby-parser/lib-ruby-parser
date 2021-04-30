use crate::containers::SharedList;

#[cfg(not(feature = "compile-with-external-structures"))]
pub(crate) mod rust {
    /// Rust-compatible list
    pub type List<T> = Vec<T>;

    use super::TakeFirst;
    impl<T: Clone> TakeFirst<T> for List<T> {
        fn take_first(self) -> T {
            self.into_iter()
                .next()
                .expect("expected at least 1 element")
        }
    }

    use super::{AsSharedList, SharedList};
    impl<T> AsSharedList<T> for List<T> {
        fn shared(&self) -> SharedList<T> {
            &self
        }
    }
}

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod c {
    use super::CppVector;
    use crate::containers::get_drop_fn::{DropInPlaceFn, GetDropFn};
    use std::ffi::c_void;

    /// List blob
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ListBlob {
        a: u64,
        b: u64,
        c: u64,
    }

    /// C-compatible list
    #[repr(C)]
    pub struct List<T>
    where
        T: GetDropFn,
    {
        blob: ListBlob,
        _t: std::marker::PhantomData<T>,
    }

    impl<T> Drop for List<T>
    where
        T: GetDropFn,
    {
        fn drop(&mut self) {
            let drop_item_in_place = T::get_drop_in_place_fn();
            let drop_list_blob = T::get_drop_list_blob_fn();
            unsafe { drop_list_blob(self.blob, drop_item_in_place) }
        }
    }

    // impl<T> Drop for List<T> {
    //     fn drop(&mut self) {
    // if self.as_ptr().is_null() {
    //     return;
    // }

    // if self.len() != 0 {
    //     unsafe {
    //         // propagate Drop on items
    //         std::ptr::drop_in_place(std::ptr::slice_from_raw_parts_mut(
    //             self.as_ptr(),
    //             self.len(),
    //         ));
    //     }
    // }

    // if self.capacity() != 0 {
    //     unsafe {
    //         // free memory
    //         let layout = std::alloc::Layout::array::<T>(self.capacity()).unwrap();
    //         std::alloc::Global
    //             .deallocate(std::ptr::NonNull::new(self.ptr as *mut u8).unwrap(), layout);
    //     }
    // }

    // self.ptr = std::ptr::null_mut();
    // self.len = 0;
    // self.capacity() = 0;
    //     }
    // }

    impl<T> std::fmt::Debug for List<T>
    where
        T: std::fmt::Debug + GetDropFn,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&**self, f)
        }
    }

    impl<T> PartialEq for List<T>
    where
        T: PartialEq + GetDropFn,
    {
        fn eq(&self, other: &Self) -> bool {
            self.as_ref() == other.as_ref()
        }
    }

    impl<T> PartialEq<&[T]> for List<T>
    where
        T: PartialEq + GetDropFn,
    {
        fn eq(&self, other: &&[T]) -> bool {
            self.as_ref() == *other
        }
    }

    impl From<String> for List<u8> {
        fn from(s: String) -> Self {
            Self::from(s.into_bytes())
        }
    }

    impl From<&str> for List<u8> {
        fn from(s: &str) -> Self {
            List::from(s.to_string())
        }
    }

    impl From<&String> for List<u8> {
        fn from(s: &String) -> Self {
            List::from(s.clone())
        }
    }

    impl From<&[u8]> for List<u8> {
        fn from(bytes: &[u8]) -> Self {
            Self::from(bytes.to_vec())
        }
    }

    extern "C" {
        fn lib_ruby_parser_containers_list_as_ptr(blob: ListBlob) -> *mut c_void;
        fn lib_ruby_parser_containers_list_len(blob: ListBlob) -> u64;
        fn lib_ruby_parser_containers_list_capacity(blob: ListBlob) -> u64;
    }

    impl<T> List<T>
    where
        T: GetDropFn,
    {
        /// Equivalent of Vec::new
        // pub fn new() -> Self {
        //     todo!()
        // }

        fn as_ptr(&self) -> *mut T {
            unsafe { lib_ruby_parser_containers_list_as_ptr(self.blob) as *mut T }
        }

        /// Equivalent of Vec::len
        pub fn len(&self) -> usize {
            unsafe { lib_ruby_parser_containers_list_len(self.blob) as usize }
        }

        /// Equivalent of Vec::capacity
        pub fn capacity(&self) -> usize {
            unsafe { lib_ruby_parser_containers_list_capacity(self.blob) as usize }
        }

        /// Equivalent of Vec::is_empty
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// Equivalent of Vec::iter
        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.as_ref().iter()
        }
    }

    impl<T> std::ops::Deref for List<T>
    where
        T: GetDropFn,
    {
        type Target = [T];

        fn deref(&self) -> &[T] {
            unsafe { std::slice::from_raw_parts(self.as_ptr(), self.len()) }
        }
    }

    impl<T> std::ops::DerefMut for List<T>
    where
        T: GetDropFn,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::slice::from_raw_parts_mut(self.as_ptr(), self.len()) }
        }
    }

    impl<T, I> std::ops::Index<I> for List<T>
    where
        T: GetDropFn,
        I: std::slice::SliceIndex<[T]>,
    {
        type Output = I::Output;

        fn index(&self, index: I) -> &Self::Output {
            std::ops::Index::index(&**self, index)
        }
    }

    use super::TakeFirst;
    impl<T> TakeFirst<T> for List<T>
    where
        T: Clone + GetDropFn,
    {
        fn take_first(self) -> T {
            if self.is_empty() {
                panic!("can't get the first item from an empty list")
            } else {
                unsafe { self.as_ptr().as_ref() }.unwrap().clone()
            }
        }
    }

    use super::{AsSharedList, SharedList};
    impl<T> AsSharedList<T> for List<T>
    where
        T: GetDropFn,
    {
        fn shared(&self) -> SharedList<T> {
            SharedList::from_raw(self.as_ptr(), self.len())
        }
    }

    macro_rules! cpp_vector_impl_for {
        ($t:ty, $new:ident, $with_capacity:ident, $from_raw:ident, $shrink_to_fit:ident, $push:ident, $remove:ident) => {
            extern "C" {
                fn $new() -> ListBlob;
                fn $with_capacity(capacity: u64) -> ListBlob;
                fn $from_raw(ptr: *mut $t, size: u64) -> ListBlob;
                fn $shrink_to_fit(blob: ListBlob);
                fn $push(blob: ListBlob, item: $t);
                fn $remove(blob: ListBlob, index: u64) -> $t;
            }

            impl CppVector<$t> for List<$t> {
                fn new() -> Self {
                    let blob = unsafe { $new() };
                    Self {
                        blob,
                        _t: std::marker::PhantomData,
                    }
                }

                fn with_capacity(capacity: usize) -> Self {
                    let blob = unsafe { $with_capacity(capacity as u64) };
                    Self {
                        blob,
                        _t: std::marker::PhantomData,
                    }
                }

                fn from_raw(ptr: *mut $t, size: usize) -> Self {
                    let blob = unsafe { $from_raw(ptr, size as u64) };
                    Self {
                        blob,
                        _t: std::marker::PhantomData,
                    }
                }

                fn push(&mut self, item: $t) {
                    unsafe { $push(self.blob, item) }
                }

                fn remove(&mut self, index: usize) -> $t {
                    unsafe { $remove(self.blob, index as u64) }
                }
            }

            impl Default for List<$t> {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl From<Vec<$t>> for List<$t> {
                fn from(mut vec: Vec<$t>) -> Self {
                    vec.shrink_to_fit();
                    let ptr = vec.as_mut_ptr();
                    let len = vec.len();
                    std::mem::forget(vec);
                    Self::from_raw(ptr, len)
                }
            }

            impl Clone for List<$t> {
                fn clone(&self) -> Self {
                    let copied = self.as_ref().iter().map(|e| e.clone()).collect::<Vec<_>>();
                    Self::from(copied)
                }
            }

            impl From<List<$t>> for Vec<$t> {
                fn from(mut list: List<$t>) -> Self {
                    unsafe { $shrink_to_fit(list.blob) };
                    let ptr = list.as_ptr();
                    let len = list.len();
                    list.blob = unsafe { $new() };
                    // let ptr = list.ptr;
                    // let len = list.len;
                    // let capacity = list.capacity;
                    // list.ptr = std::ptr::null_mut();
                    // list.len = 0;
                    // list.capacity = 0;
                    unsafe { Vec::from_raw_parts(ptr, len, len) }
                }
            }

            // impl Drop for List<$t> {
            //     fn drop(&mut self) {}
            // }
        };
    }

    cpp_vector_impl_for!(
        crate::Node,
        lib_ruby_parser_containers_node_list_blob_new,
        lib_ruby_parser_containers_node_list_blob_with_capacity,
        lib_ruby_parser_containers_node_list_blob_from_raw,
        lib_ruby_parser_containers_node_list_blob_shrink_to_fit,
        lib_ruby_parser_containers_node_list_blob_push,
        lib_ruby_parser_containers_node_list_blob_remove
    );
    cpp_vector_impl_for!(
        crate::Diagnostic,
        lib_ruby_parser_containers_diagnostic_list_blob_new,
        lib_ruby_parser_containers_diagnostic_list_blob_with_capacity,
        lib_ruby_parser_containers_diagnostic_list_blob_from_raw,
        lib_ruby_parser_containers_diagnostic_list_blob_shrink_to_fit,
        lib_ruby_parser_containers_diagnostic_list_blob_push,
        lib_ruby_parser_containers_diagnostic_list_blob_remove
    );
    cpp_vector_impl_for!(
        crate::source::Comment,
        lib_ruby_parser_containers_comment_list_blob_new,
        lib_ruby_parser_containers_comment_list_blob_with_capacity,
        lib_ruby_parser_containers_comment_list_blob_from_raw,
        lib_ruby_parser_containers_comment_list_blob_shrink_to_fit,
        lib_ruby_parser_containers_comment_list_blob_push,
        lib_ruby_parser_containers_comment_list_blob_remove
    );
    cpp_vector_impl_for!(
        crate::source::MagicComment,
        lib_ruby_parser_containers_magic_comment_list_blob_new,
        lib_ruby_parser_containers_magic_comment_list_blob_with_capacity,
        lib_ruby_parser_containers_magic_comment_list_blob_from_raw,
        lib_ruby_parser_containers_magic_comment_list_blob_shrink_to_fit,
        lib_ruby_parser_containers_magic_comment_list_blob_push,
        lib_ruby_parser_containers_magic_comment_list_blob_remove
    );
    cpp_vector_impl_for!(
        crate::Token,
        lib_ruby_parser_containers_token_list_blob_new,
        lib_ruby_parser_containers_token_list_blob_with_capacity,
        lib_ruby_parser_containers_token_list_blob_from_raw,
        lib_ruby_parser_containers_token_list_blob_shrink_to_fit,
        lib_ruby_parser_containers_token_list_blob_push,
        lib_ruby_parser_containers_token_list_blob_remove
    );
    cpp_vector_impl_for!(
        crate::source::SourceLine,
        lib_ruby_parser_containers_source_line_list_blob_new,
        lib_ruby_parser_containers_source_line_list_blob_with_capacity,
        lib_ruby_parser_containers_source_line_list_blob_from_raw,
        lib_ruby_parser_containers_source_line_list_blob_shrink_to_fit,
        lib_ruby_parser_containers_source_line_list_blob_push,
        lib_ruby_parser_containers_source_line_list_blob_remove
    );
    cpp_vector_impl_for!(
        u8,
        lib_ruby_parser_containers_byte_list_blob_new,
        lib_ruby_parser_containers_byte_list_blob_with_capacity,
        lib_ruby_parser_containers_byte_list_blob_from_raw,
        lib_ruby_parser_containers_byte_list_blob_shrink_to_fit,
        lib_ruby_parser_containers_byte_list_blob_push,
        lib_ruby_parser_containers_byte_list_blob_remove
    );

    #[cfg(test)]
    mod tests {
        use super::{CppVector, List as GenericList, ListBlob, TakeFirst};
        type List = GenericList<u8>;

        #[test]
        fn test_size() {
            println!("{:?}", std::mem::size_of::<crate::Node>());
            assert_eq!(std::mem::size_of::<ListBlob>(), 24);
        }

        #[test]
        fn test_new() {
            let list = List::new();
            assert_eq!(list.len(), 0);
            assert_eq!(list.capacity(), 0);
        }

        #[test]
        fn test_with_capacity() {
            let list = List::with_capacity(20);
            assert_eq!(list.len(), 0);
            assert_eq!(list.capacity(), 20);
        }

        #[test]
        fn test_push() {
            let mut list = List::new();
            let mut vec = vec![];
            for i in 1..20 {
                list.push(i);
                vec.push(i);
            }
            assert_eq!(list.as_ref(), &vec);
        }

        #[test]
        fn test_take_first() {
            let mut list = List::new();
            list.push(10);
            list.push(20);
            assert_eq!(list.take_first(), 10)
        }

        #[test]
        fn test_from_vec() {
            let list = List::from(vec![1, 2, 3]);
            assert_eq!(list.as_ref(), &[1, 2, 3])
        }
    }
}

pub type ListBlob = c::ListBlob;

pub(crate) trait TakeFirst<T: Clone> {
    fn take_first(self) -> T;
}

pub(crate) trait AsSharedList<T> {
    fn shared(&self) -> SharedList<T>;
}

pub(crate) trait CppVector<T> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn from_raw(ptr: *mut T, size: usize) -> Self;
    fn push(&mut self, item: T);
    fn remove(&mut self, index: usize) -> T;
}
