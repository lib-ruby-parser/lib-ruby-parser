#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalSharedByteList;

#[cfg(not(feature = "compile-with-external-structures"))]
pub(crate) mod rust {
    /// Rust-compatible list
    pub type List<T> = Vec<T>;

    use super::ListAPI;
    impl<T: Clone> ListAPI<T> for List<T> {
        fn take_first(self) -> T {
            self.into_iter()
                .next()
                .expect("expected at least 1 element")
        }
    }
}

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod external {
    use crate::blobs::ListBlob;
    use crate::containers::get_drop_fn::GetDropListFn;

    /// C-compatible list
    #[repr(C)]
    pub struct List<T>
    where
        T: GetDropListFn,
    {
        pub(crate) blob: ListBlob,
        _t: std::marker::PhantomData<T>,
    }

    impl<T> Drop for List<T>
    where
        T: GetDropListFn,
    {
        fn drop(&mut self) {
            let drop_list_fn = T::get_drop_list_fn();
            unsafe { drop_list_fn(&mut self.blob) }
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

    impl<T: GetDropListFn> From<ListBlob> for List<T> {
        fn from(blob: ListBlob) -> Self {
            unsafe { std::mem::transmute(blob) }
        }
    }

    impl<T: GetDropListFn> From<List<T>> for ListBlob {
        fn from(list: List<T>) -> Self {
            unsafe { std::mem::transmute(list) }
        }
    }

    impl<T: GetDropListFn> IntoIterator for List<T>
    where
        Vec<T>: From<List<T>>,
    {
        type Item = T;
        type IntoIter = std::vec::IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            let v: Vec<T> = self.into();
            v.into_iter()
        }
    }

    use super::ExternalSharedByteList;
    use super::ListAPI;

    macro_rules! gen_list_impl_for {
        (
            $t:ty,
            $new:ident,
            $with_capacity:ident,
            $from_raw:ident,
            $push:ident,
            $remove:ident,
            $shrink_to_fit:ident,
            $as_ptr:ident,
            $len:ident,
            $capacity:ident
        ) => {
            use super::{List, ListAPI, ListBlob};

            extern "C" {
                fn $new() -> ListBlob;
                fn $with_capacity(capacity: u64) -> ListBlob;
                fn $from_raw(ptr: *mut $t, size: u64) -> ListBlob;
                fn $push(blob: *mut ListBlob, item: $t);
                fn $remove(blob: *mut ListBlob, index: u64) -> $t;
                fn $shrink_to_fit(blob: *mut ListBlob);
                fn $as_ptr(blob: *mut ListBlob) -> *mut $t;
                fn $len(blob: *const ListBlob) -> u64;
                fn $capacity(blob: *const ListBlob) -> u64;
            }

            impl List<$t> {
                /// Equivalent of Vec::new
                pub fn new() -> Self {
                    let blob = unsafe { $new() };
                    Self::from_blob(blob)
                }

                /// Equivalent of Vec::with_capacity
                pub fn with_capacity(capacity: usize) -> Self {
                    let blob = unsafe { $with_capacity(capacity as u64) };
                    Self::from_blob(blob)
                }

                pub(crate) fn from_raw(ptr: *mut $t, size: usize) -> Self {
                    let blob = unsafe { $from_raw(ptr, size as u64) };
                    Self::from_blob(blob)
                }

                pub(crate) fn shrink_to_fit(&mut self) {
                    unsafe { $shrink_to_fit(&mut self.blob) };
                }

                /// Equivalent of Vec::push
                pub fn push(&mut self, item: $t) {
                    unsafe { $push(&mut self.blob, item) };
                }

                /// Equivalent of Vec::rmeove
                pub fn remove(&mut self, index: usize) -> $t {
                    unsafe { $remove(&mut self.blob, index as u64) }
                }

                pub(crate) fn as_ptr(&self) -> *const $t {
                    let blob_ptr: *const ListBlob = &self.blob;
                    unsafe { $as_ptr(blob_ptr as *mut ListBlob) }
                }

                pub(crate) fn as_mut_ptr(&mut self) -> *mut $t {
                    unsafe { $as_ptr(&mut self.blob) }
                }

                pub(crate) fn into_ptr(mut self) -> *mut $t {
                    let ptr = self.as_mut_ptr();
                    std::mem::forget(self);
                    ptr
                }

                /// Equivalent of Vec::len
                pub fn len(&self) -> usize {
                    unsafe { $len(&self.blob) as usize }
                }

                /// Equivalent of Vec::capacity
                pub fn capacity(&self) -> usize {
                    unsafe { $capacity(&self.blob) as usize }
                }

                /// Creates List from ListBlob
                pub fn from_blob(blob: ListBlob) -> Self {
                    Self {
                        blob,
                        _t: std::marker::PhantomData,
                    }
                }
            }

            impl List<$t> {
                /// Equivalent of Vec::iter
                #[allow(dead_code)]
                fn iter(&self) -> std::slice::Iter<'_, $t> {
                    self.as_ref().iter()
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

            impl<const N: usize> From<[$t; N]> for List<$t> {
                fn from(array: [$t; N]) -> Self {
                    Self::from(Vec::<$t>::from(array))
                }
            }

            impl<const N: usize> From<&[$t; N]> for List<$t> {
                fn from(array: &[$t; N]) -> Self {
                    Self::from(Vec::<$t>::from(array.to_owned()))
                }
            }

            impl Clone for List<$t> {
                fn clone(&self) -> Self {
                    let copied = self.as_ref().iter().map(|e| e.clone()).collect::<Vec<$t>>();
                    let copy = Self::from(copied);
                    drop(copy);

                    let copied = self.as_ref().iter().map(|e| e.clone()).collect::<Vec<$t>>();
                    let copy = Self::from(copied);
                    copy
                }
            }

            impl From<List<$t>> for Vec<$t> {
                fn from(mut list: List<$t>) -> Self {
                    list.shrink_to_fit();
                    let len = list.len();
                    let ptr = list.into_ptr();
                    unsafe { Vec::from_raw_parts(ptr, len, len) }
                }
            }

            impl std::ops::Deref for List<$t> {
                type Target = [$t];

                fn deref(&self) -> &[$t] {
                    unsafe { std::slice::from_raw_parts(self.as_ptr(), self.len()) }
                }
            }

            impl std::ops::DerefMut for List<$t> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    unsafe { std::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
                }
            }

            impl std::fmt::Debug for List<$t> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    std::fmt::Debug::fmt(&**self, f)
                }
            }

            impl PartialEq for List<$t> {
                fn eq(&self, other: &Self) -> bool {
                    self.as_ref() == other.as_ref()
                }
            }

            impl PartialEq<List<$t>> for [$t] {
                fn eq(&self, other: &List<$t>) -> bool {
                    self == other.as_ref()
                }
            }

            impl Eq for List<$t> {}

            impl PartialEq<&[$t]> for List<$t> {
                fn eq(&self, other: &&[$t]) -> bool {
                    self.as_ref() == *other
                }
            }

            impl PartialEq<[$t]> for List<$t> {
                fn eq(&self, other: &[$t]) -> bool {
                    self.as_ref() == other
                }
            }

            impl PartialEq<Vec<$t>> for List<$t> {
                fn eq(&self, other: &Vec<$t>) -> bool {
                    self.as_ref() == other.as_slice()
                }
            }

            impl PartialEq<List<$t>> for Vec<$t> {
                fn eq(&self, other: &List<$t>) -> bool {
                    self.as_slice() == other.as_ref()
                }
            }

            impl<I> std::ops::Index<I> for List<$t>
            where
                I: std::slice::SliceIndex<[$t]>,
            {
                type Output = I::Output;

                fn index(&self, index: I) -> &Self::Output {
                    std::ops::Index::index(&**self, index)
                }
            }

            impl ListAPI<$t> for List<$t> {
                fn take_first(self) -> $t {
                    if self.is_empty() {
                        panic!("can't get the first item from an empty list")
                    } else {
                        unsafe { self.as_ptr().as_ref() }.unwrap().clone()
                    }
                }
            }

            #[cfg(test)]
            mod tests {
                use super::{new_item, List as GenericList, ListAPI};
                use std::ops::{Deref, DerefMut};
                type List = GenericList<$t>;

                #[test]
                fn test_new() {
                    let list = List::new();
                    drop(list);
                }

                #[test]
                fn test_with_capacity() {
                    let list = List::with_capacity(10);
                    drop(list);
                }

                fn list_with_items(n: usize) -> List {
                    let mut list = List::new();
                    for _ in 0..n {
                        list.push(new_item());
                    }
                    list
                }

                fn vec_with_items(n: usize) -> Vec<$t> {
                    let mut list = vec![];
                    for _ in 0..n {
                        list.push(new_item());
                    }
                    list
                }

                #[test]
                fn test_from_raw() {
                    let mut vec = vec_with_items(20);
                    let ptr = vec.as_mut_ptr();
                    let len = vec.len();
                    std::mem::forget(vec);

                    let list = List::from_raw(ptr, len);
                    assert_eq!(list.len(), 20);
                }

                #[test]
                fn test_push() {
                    let mut list = List::with_capacity(10);
                    list.push(new_item());
                }

                #[test]
                fn test_remove() {
                    let mut list = list_with_items(10);

                    assert_eq!(list.remove(0), new_item());
                    assert_eq!(list.len(), 9);

                    assert_eq!(list.remove(3), new_item());
                    assert_eq!(list.len(), 8);
                }

                #[test]
                fn test_as_ptr() {
                    let mut list = List::new();
                    assert_eq!(list.as_ptr(), std::ptr::null_mut());

                    list.push(new_item());
                    assert_eq!(unsafe { &*list.as_ptr() }, &new_item());
                }

                #[test]
                fn test_len() {
                    let list = list_with_items(10);
                    assert_eq!(list.len(), 10);
                }

                #[test]
                fn test_capacity() {
                    let list = List::with_capacity(10);
                    assert_eq!(list.capacity(), 10);
                }

                #[test]
                fn test_iter() {
                    let list = list_with_items(20);
                    let mut iterated = vec![];
                    for i in list.iter() {
                        iterated.push(i.clone());
                    }

                    assert_eq!(iterated.len(), 20);
                }

                #[test]
                fn test_default() {
                    let list = List::default();
                    assert_eq!(list.len(), 0);
                    assert_eq!(list.len(), 0);
                    assert_eq!(list.as_ptr(), std::ptr::null_mut());
                }

                #[test]
                fn test_list_from_vec() {
                    let list = List::from(vec_with_items(10));
                    assert_eq!(list.len(), 10);
                }

                #[test]
                fn test_clone() {
                    let mut list = List::new();
                    // assert_eq!(list.clone().len(), 0);

                    list.push(new_item());
                    list.push(new_item());
                    let copy = list.clone();
                    assert_eq!(copy.len(), 2);
                    drop(copy);
                    drop(list);
                }

                #[test]
                fn test_vec_from_list() {
                    let mut list = list_with_items(1);
                    list.shrink_to_fit();
                    assert_eq!(Vec::from(list_with_items(1)), vec_with_items(1));
                }

                #[test]
                fn test_deref() {
                    let mut list = List::new();
                    let mut vec = vec![];
                    assert_eq!(list.deref(), &vec);

                    list.push(new_item());
                    vec.push(new_item());
                    assert_eq!(list.deref(), &vec);
                }

                #[test]
                fn test_deref_mut() {
                    let mut list = List::new();
                    let mut vec = vec![];
                    assert_eq!(list.deref_mut(), &vec);

                    list.push(new_item());
                    vec.push(new_item());
                    assert_eq!(list.deref_mut(), &vec);
                }

                #[test]
                fn test_fmt() {
                    let mut list = List::new();
                    list.push(new_item());

                    let vec = vec![new_item()];
                    let slice = &vec;
                    assert_eq!(format!("{:?}", list), format!("{:?}", slice));
                }

                #[test]
                fn test_eq_list_list() {
                    assert_eq!(list_with_items(10), list_with_items(10));
                    assert_ne!(list_with_items(10), list_with_items(20));
                }

                #[test]
                fn test_eq_slice_list() {
                    let list = list_with_items(0);
                    let vec = vec_with_items(0);
                    let slice: &[$t] = &vec;
                    assert_eq!(list, slice);

                    let list = list_with_items(20);
                    let vec = vec_with_items(20);
                    let slice: &[$t] = &vec;
                    assert_eq!(list, slice);
                }

                #[test]
                fn test_index() {
                    let mut list = List::new();
                    list.push(new_item());

                    assert_eq!(list[0], new_item());
                }

                #[test]
                fn test_take_first() {
                    let mut list = List::new();
                    list.push(new_item());

                    assert_eq!(list.take_first(), new_item());
                }

                #[test]
                fn test_vec_item() {
                    vec_with_items(10).truncate(5)
                }
            }
        };
    }

    mod of_nodes {
        #[cfg(test)]
        fn new_item() -> crate::Node {
            crate::Node::new_true(crate::Loc::default())
        }

        gen_list_impl_for!(
            crate::Node,
            lib_ruby_parser__external__list__of_nodes__new,
            lib_ruby_parser__external__list__of_nodes__with_capacity,
            lib_ruby_parser__external__list__of_nodes__from_raw,
            lib_ruby_parser__external__list__of_nodes__push,
            lib_ruby_parser__external__list__of_nodes__remove,
            lib_ruby_parser__external__list__of_nodes__shrink_to_fit,
            lib_ruby_parser__external__list__of_nodes__as_ptr,
            lib_ruby_parser__external__list__of_nodes__get_len,
            lib_ruby_parser__external__list__of_nodes__get_capacity
        );
    }
    mod of_diagnostics {
        #[cfg(test)]
        fn new_item() -> crate::Diagnostic {
            crate::Diagnostic::new(
                crate::ErrorLevel::warning(),
                crate::DiagnosticMessage::new_alias_nth_ref(),
                crate::Loc::default(),
            )
        }

        gen_list_impl_for!(
            crate::Diagnostic,
            lib_ruby_parser__external__list__of_diagnostics__new,
            lib_ruby_parser__external__list__of_diagnostics__with_capacity,
            lib_ruby_parser__external__list__of_diagnostics__from_raw,
            lib_ruby_parser__external__list__of_diagnostics__push,
            lib_ruby_parser__external__list__of_diagnostics__remove,
            lib_ruby_parser__external__list__of_diagnostics__shrink_to_fit,
            lib_ruby_parser__external__list__of_diagnostics__as_ptr,
            lib_ruby_parser__external__list__of_diagnostics__get_len,
            lib_ruby_parser__external__list__of_diagnostics__get_capacity
        );
    }
    mod of_comments {
        #[cfg(test)]
        fn new_item() -> crate::source::Comment {
            crate::source::Comment::make(
                crate::Loc::new(1, 2),
                crate::source::CommentType::unknown(),
            )
        }

        gen_list_impl_for!(
            crate::source::Comment,
            lib_ruby_parser__external__list__of_comments__new,
            lib_ruby_parser__external__list__of_comments__with_capacity,
            lib_ruby_parser__external__list__of_comments__from_raw,
            lib_ruby_parser__external__list__of_comments__push,
            lib_ruby_parser__external__list__of_comments__remove,
            lib_ruby_parser__external__list__of_comments__shrink_to_fit,
            lib_ruby_parser__external__list__of_comments__as_ptr,
            lib_ruby_parser__external__list__of_comments__get_len,
            lib_ruby_parser__external__list__of_comments__get_capacity
        );
    }
    mod of_magic_comments {
        #[cfg(test)]
        fn new_item() -> crate::source::MagicComment {
            crate::source::MagicComment::new(
                crate::source::MagicCommentKind::encoding(),
                crate::Loc::default(),
                crate::Loc::default(),
            )
        }

        gen_list_impl_for!(
            crate::source::MagicComment,
            lib_ruby_parser__external__list__of_magic_comments__new,
            lib_ruby_parser__external__list__of_magic_comments__with_capacity,
            lib_ruby_parser__external__list__of_magic_comments__from_raw,
            lib_ruby_parser__external__list__of_magic_comments__push,
            lib_ruby_parser__external__list__of_magic_comments__remove,
            lib_ruby_parser__external__list__of_magic_comments__shrink_to_fit,
            lib_ruby_parser__external__list__of_magic_comments__as_ptr,
            lib_ruby_parser__external__list__of_magic_comments__get_len,
            lib_ruby_parser__external__list__of_magic_comments__get_capacity
        );
    }
    mod of_tokens {
        #[cfg(test)]
        fn new_item() -> crate::Token {
            crate::Token::new(
                crate::Lexer::tINTEGER,
                crate::Bytes::empty(),
                crate::Loc::default(),
                crate::LexState::default(),
                crate::LexState::default(),
            )
        }

        gen_list_impl_for!(
            crate::Token,
            lib_ruby_parser__external__list__of_tokens__new,
            lib_ruby_parser__external__list__of_tokens__with_capacity,
            lib_ruby_parser__external__list__of_tokens__from_raw,
            lib_ruby_parser__external__list__of_tokens__push,
            lib_ruby_parser__external__list__of_tokens__remove,
            lib_ruby_parser__external__list__of_tokens__shrink_to_fit,
            lib_ruby_parser__external__list__of_tokens__as_ptr,
            lib_ruby_parser__external__list__of_tokens__get_len,
            lib_ruby_parser__external__list__of_tokens__get_capacity
        );
    }
    mod of_source_lines {
        #[cfg(test)]
        fn new_item() -> crate::source::SourceLine {
            crate::source::SourceLine::new(1, 2, false)
        }

        gen_list_impl_for!(
            crate::source::SourceLine,
            lib_ruby_parser__external__list__of_source_lines__new,
            lib_ruby_parser__external__list__of_source_lines__with_capacity,
            lib_ruby_parser__external__list__of_source_lines__from_raw,
            lib_ruby_parser__external__list__of_source_lines__push,
            lib_ruby_parser__external__list__of_source_lines__remove,
            lib_ruby_parser__external__list__of_source_lines__shrink_to_fit,
            lib_ruby_parser__external__list__of_source_lines__as_ptr,
            lib_ruby_parser__external__list__of_source_lines__get_len,
            lib_ruby_parser__external__list__of_source_lines__get_capacity
        );
    }
    mod of_u8 {
        #[cfg(test)]
        fn new_item() -> u8 {
            42
        }

        gen_list_impl_for!(
            u8,
            lib_ruby_parser__external__list__of_bytes__new,
            lib_ruby_parser__external__list__of_bytes__with_capacity,
            lib_ruby_parser__external__list__of_bytes__from_raw,
            lib_ruby_parser__external__list__of_bytes__push,
            lib_ruby_parser__external__list__of_bytes__remove,
            lib_ruby_parser__external__list__of_bytes__shrink_to_fit,
            lib_ruby_parser__external__list__of_bytes__as_ptr,
            lib_ruby_parser__external__list__of_bytes__get_len,
            lib_ruby_parser__external__list__of_bytes__get_capacity
        );

        use super::ExternalSharedByteList;

        impl List<u8> {
            /// Equivalent of Vec::as_slice
            pub fn as_slice(&self) -> ExternalSharedByteList {
                ExternalSharedByteList::from_raw(self.as_ptr(), self.len())
            }
        }
    }
}

pub(crate) trait ListAPI<T: Clone> {
    fn take_first(self) -> T;
}
