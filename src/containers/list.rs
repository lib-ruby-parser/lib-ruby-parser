#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalSharedByteList;

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

    #[cfg(test)]
    mod tests {
        use super::List as GenericList;
        type List = GenericList<u8>;

        #[test]
        fn test_size() {
            assert_eq!(std::mem::size_of::<List>(), 24);
        }
    }
}

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod external {
    use crate::containers::get_drop_fn::GetDropFn;

    use crate::containers::size::LIST_SIZE;

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct ListBlob {
        blob: [u8; LIST_SIZE],
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
            let drop_item_in_place = T::get_drop_ptr_in_place_fn();
            let drop_list_blob = T::get_drop_list_blob_fn();
            unsafe { drop_list_blob(self.blob, drop_item_in_place) }
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

    impl<T: GetDropFn> From<ListBlob> for List<T> {
        fn from(blob: ListBlob) -> Self {
            unsafe { std::mem::transmute(blob) }
        }
    }

    impl<T: GetDropFn> From<List<T>> for ListBlob {
        fn from(list: List<T>) -> Self {
            unsafe { std::mem::transmute(list) }
        }
    }

    use super::ExternalSharedByteList;
    use super::TakeFirst;

    macro_rules! gen_list_impl_for {
        (
            $t:ty,
            $new:ident,
            $with_capacity:ident,
            $from_raw:ident,
            $shrink_to_fit:ident,
            $push:ident,
            $remove:ident,
            $as_ptr:ident,
            $len:ident,
            $capacity:ident
        ) => {
            use super::{List, ListBlob, TakeFirst};

            #[repr(C)]
            struct RemoveResult {
                new_blob: ListBlob,
                removed_item: $t,
            }

            extern "C" {
                fn $new() -> ListBlob;
                fn $with_capacity(capacity: u64) -> ListBlob;
                fn $from_raw(ptr: *mut $t, size: u64) -> ListBlob;
                fn $shrink_to_fit(blob: ListBlob) -> ListBlob;
                fn $push(blob: ListBlob, item: $t) -> ListBlob;
                fn $remove(blob: ListBlob, index: u64) -> RemoveResult;
                fn $as_ptr(blob: ListBlob) -> *mut $t;
                fn $len(blob: ListBlob) -> u64;
                fn $capacity(blob: ListBlob) -> u64;
            }

            impl List<$t> {
                /// Equivalent of Vec::new
                pub fn new() -> Self {
                    let blob = unsafe { $new() };
                    Self {
                        blob,
                        _t: std::marker::PhantomData,
                    }
                }

                /// Equivalent of Vec::with_capacity
                pub fn with_capacity(capacity: usize) -> Self {
                    let blob = unsafe { $with_capacity(capacity as u64) };
                    Self {
                        blob,
                        _t: std::marker::PhantomData,
                    }
                }

                pub(crate) fn from_raw(ptr: *mut $t, size: usize) -> Self {
                    let blob = unsafe { $from_raw(ptr, size as u64) };
                    Self {
                        blob,
                        _t: std::marker::PhantomData,
                    }
                }

                pub(crate) fn shrink_to_fit(&mut self) {
                    self.blob = unsafe { $shrink_to_fit(self.blob) };
                }

                /// Equivalent of Vec::push
                pub fn push(&mut self, item: $t) {
                    self.blob = unsafe { $push(self.blob, item) };
                }

                /// Equivalent of Vec::rmeove
                pub fn remove(&mut self, index: usize) -> $t {
                    let RemoveResult {
                        new_blob,
                        removed_item,
                    } = unsafe { $remove(self.blob, index as u64) };
                    self.blob = new_blob;
                    removed_item
                }

                pub(crate) fn as_ptr(&self) -> *mut $t {
                    unsafe { $as_ptr(self.blob) }
                }

                /// Equivalent of Vec::len
                pub fn len(&self) -> usize {
                    unsafe { $len(self.blob) as usize }
                }

                /// Equivalent of Vec::capacity
                pub fn capacity(&self) -> usize {
                    unsafe { $capacity(self.blob) as usize }
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
                    let ptr = list.as_ptr();
                    let len = list.len();
                    list.blob = unsafe { $new() };
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
                    unsafe { std::slice::from_raw_parts_mut(self.as_ptr(), self.len()) }
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

            impl TakeFirst<$t> for List<$t> {
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
                use super::{make_one, List as GenericList, TakeFirst};
                use std::ops::{Deref, DerefMut};
                type List = GenericList<$t>;

                #[test]
                fn test_size() {
                    assert_eq!(std::mem::size_of::<List>(), 24);
                }

                #[test]
                fn test_new() {
                    let _ = List::new();
                }

                #[test]
                fn test_with_capacity() {
                    let _ = List::with_capacity(10);
                }

                fn list_with_items(n: usize) -> List {
                    let mut list = List::new();
                    for _ in 0..n {
                        list.push(make_one());
                    }
                    list
                }

                fn vec_with_items(n: usize) -> Vec<$t> {
                    let mut list = vec![];
                    for _ in 0..n {
                        list.push(make_one());
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
                    list.push(make_one());
                }

                #[test]
                fn test_remove() {
                    let mut list = list_with_items(10);

                    assert_eq!(list.remove(0), make_one());
                    assert_eq!(list.len(), 9);

                    assert_eq!(list.remove(3), make_one());
                    assert_eq!(list.len(), 8);
                }

                #[test]
                fn test_as_ptr() {
                    let mut list = List::new();
                    assert_eq!(list.as_ptr(), std::ptr::null_mut());

                    list.push(make_one());
                    assert_eq!(unsafe { &*list.as_ptr() }, &make_one());
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

                    list.push(make_one());
                    list.push(make_one());
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

                    list.push(make_one());
                    vec.push(make_one());
                    assert_eq!(list.deref(), &vec);
                }

                #[test]
                fn test_deref_mut() {
                    let mut list = List::new();
                    let mut vec = vec![];
                    assert_eq!(list.deref_mut(), &vec);

                    list.push(make_one());
                    vec.push(make_one());
                    assert_eq!(list.deref_mut(), &vec);
                }

                #[test]
                fn test_fmt() {
                    let mut list = List::new();
                    list.push(make_one());

                    let vec = vec![make_one()];
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
                    list.push(make_one());

                    assert_eq!(list[0], make_one());
                }

                #[test]
                fn test_take_first() {
                    let mut list = List::new();
                    list.push(make_one());

                    assert_eq!(list.take_first(), make_one());
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
        fn make_one() -> crate::Node {
            crate::Node::True(crate::nodes::True {
                expression_l: crate::Loc::default(),
            })
        }

        gen_list_impl_for!(
            crate::Node,
            lib_ruby_parser_containers_node_list_blob_new,
            lib_ruby_parser_containers_node_list_blob_with_capacity,
            lib_ruby_parser_containers_node_list_blob_from_raw,
            lib_ruby_parser_containers_node_list_blob_shrink_to_fit,
            lib_ruby_parser_containers_node_list_blob_push,
            lib_ruby_parser_containers_node_list_blob_remove,
            lib_ruby_parser_containers_node_list_blob_as_ptr,
            lib_ruby_parser_containers_node_list_blob_len,
            lib_ruby_parser_containers_node_list_blob_capacity
        );
    }
    mod of_diagnostics {
        #[cfg(test)]
        fn make_one() -> crate::Diagnostic {
            crate::Diagnostic {
                level: crate::ErrorLevel::Warning,
                message: crate::DiagnosticMessage::AliasNthRef,
                loc: crate::Loc::default(),
            }
        }

        gen_list_impl_for!(
            crate::Diagnostic,
            lib_ruby_parser_containers_diagnostic_list_blob_new,
            lib_ruby_parser_containers_diagnostic_list_blob_with_capacity,
            lib_ruby_parser_containers_diagnostic_list_blob_from_raw,
            lib_ruby_parser_containers_diagnostic_list_blob_shrink_to_fit,
            lib_ruby_parser_containers_diagnostic_list_blob_push,
            lib_ruby_parser_containers_diagnostic_list_blob_remove,
            lib_ruby_parser_containers_diagnostic_list_blob_as_ptr,
            lib_ruby_parser_containers_diagnostic_list_blob_len,
            lib_ruby_parser_containers_diagnostic_list_blob_capacity
        );
    }
    mod of_comments {
        #[cfg(test)]
        fn make_one() -> crate::source::Comment {
            crate::source::Comment {
                location: crate::Loc::default(),
                kind: crate::source::CommentType::Inline,
            }
        }

        gen_list_impl_for!(
            crate::source::Comment,
            lib_ruby_parser_containers_comment_list_blob_new,
            lib_ruby_parser_containers_comment_list_blob_with_capacity,
            lib_ruby_parser_containers_comment_list_blob_from_raw,
            lib_ruby_parser_containers_comment_list_blob_shrink_to_fit,
            lib_ruby_parser_containers_comment_list_blob_push,
            lib_ruby_parser_containers_comment_list_blob_remove,
            lib_ruby_parser_containers_comment_list_blob_as_ptr,
            lib_ruby_parser_containers_comment_list_blob_len,
            lib_ruby_parser_containers_comment_list_blob_capacity
        );
    }
    mod of_magic_comments {
        #[cfg(test)]
        fn make_one() -> crate::source::MagicComment {
            crate::source::MagicComment {
                kind: crate::source::MagicCommentKind::Encoding,
                key_l: crate::Loc::default(),
                value_l: crate::Loc::default(),
            }
        }

        gen_list_impl_for!(
            crate::source::MagicComment,
            lib_ruby_parser_containers_magic_comment_list_blob_new,
            lib_ruby_parser_containers_magic_comment_list_blob_with_capacity,
            lib_ruby_parser_containers_magic_comment_list_blob_from_raw,
            lib_ruby_parser_containers_magic_comment_list_blob_shrink_to_fit,
            lib_ruby_parser_containers_magic_comment_list_blob_push,
            lib_ruby_parser_containers_magic_comment_list_blob_remove,
            lib_ruby_parser_containers_magic_comment_list_blob_as_ptr,
            lib_ruby_parser_containers_magic_comment_list_blob_len,
            lib_ruby_parser_containers_magic_comment_list_blob_capacity
        );
    }
    mod of_tokens {
        #[cfg(test)]
        fn make_one() -> crate::Token {
            use crate::bytes::BytesTrait;
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
            lib_ruby_parser_containers_token_list_blob_new,
            lib_ruby_parser_containers_token_list_blob_with_capacity,
            lib_ruby_parser_containers_token_list_blob_from_raw,
            lib_ruby_parser_containers_token_list_blob_shrink_to_fit,
            lib_ruby_parser_containers_token_list_blob_push,
            lib_ruby_parser_containers_token_list_blob_remove,
            lib_ruby_parser_containers_token_list_blob_as_ptr,
            lib_ruby_parser_containers_token_list_blob_len,
            lib_ruby_parser_containers_token_list_blob_capacity
        );
    }
    mod of_source_lines {
        #[cfg(test)]
        fn make_one() -> crate::source::SourceLine {
            use crate::source::SourceLineTrait;

            crate::source::SourceLine::new(1, 2, false)
        }

        gen_list_impl_for!(
            crate::source::SourceLine,
            lib_ruby_parser_containers_source_line_list_blob_new,
            lib_ruby_parser_containers_source_line_list_blob_with_capacity,
            lib_ruby_parser_containers_source_line_list_blob_from_raw,
            lib_ruby_parser_containers_source_line_list_blob_shrink_to_fit,
            lib_ruby_parser_containers_source_line_list_blob_push,
            lib_ruby_parser_containers_source_line_list_blob_remove,
            lib_ruby_parser_containers_source_line_list_blob_as_ptr,
            lib_ruby_parser_containers_source_line_list_blob_len,
            lib_ruby_parser_containers_source_line_list_blob_capacity
        );
    }
    mod of_u8 {
        #[cfg(test)]
        fn make_one() -> u8 {
            42
        }

        gen_list_impl_for!(
            u8,
            lib_ruby_parser_containers_byte_list_blob_new,
            lib_ruby_parser_containers_byte_list_blob_with_capacity,
            lib_ruby_parser_containers_byte_list_blob_from_raw,
            lib_ruby_parser_containers_byte_list_blob_shrink_to_fit,
            lib_ruby_parser_containers_byte_list_blob_push,
            lib_ruby_parser_containers_byte_list_blob_remove,
            lib_ruby_parser_containers_byte_list_blob_as_ptr,
            lib_ruby_parser_containers_byte_list_blob_len,
            lib_ruby_parser_containers_byte_list_blob_capacity
        );

        use super::ExternalSharedByteList;

        impl List<u8> {
            /// Equivalent of Vec::as_slice
            pub fn as_slice(&self) -> ExternalSharedByteList {
                ExternalSharedByteList::from_raw(self.as_ptr(), self.len())
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::ListBlob;

        #[test]
        fn test_size() {
            assert_eq!(std::mem::size_of::<ListBlob>(), 24);
        }
    }
}

pub(crate) trait TakeFirst<T: Clone> {
    fn take_first(self) -> T;
}
