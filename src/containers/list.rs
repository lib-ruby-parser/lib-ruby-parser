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

    pub trait ExternalListMember: std::fmt::Debug {
        fn get_new_fn() -> unsafe extern "C" fn() -> ListBlob;
        fn get_drop_fn() -> unsafe extern "C" fn(*mut ListBlob);
        fn get_with_capacity_fn() -> unsafe extern "C" fn(capacity: u64) -> ListBlob;
        fn get_from_raw_fn() -> unsafe extern "C" fn(ptr: *mut Self, size: u64) -> ListBlob;
        fn get_push_fn() -> unsafe extern "C" fn(blob: *mut ListBlob, item: Self);
        fn get_pop_fn() -> unsafe extern "C" fn(blob: *mut ListBlob) -> Self;
        fn get_remove_fn() -> unsafe extern "C" fn(blob: *mut ListBlob, index: u64) -> Self;
        fn get_shrink_to_fit_fn() -> unsafe extern "C" fn(blob: *mut ListBlob);
        fn get_as_ptr_fn() -> unsafe extern "C" fn(blob: *const ListBlob) -> *const Self;
        fn get_take_ptr_fn() -> unsafe extern "C" fn(blob: *mut ListBlob) -> *mut Self;
        fn get_len_fn() -> unsafe extern "C" fn(blob: *const ListBlob) -> u64;
        fn get_capacity_fn() -> unsafe extern "C" fn(blob: *const ListBlob) -> u64;
        fn get_reserve_fn() -> unsafe extern "C" fn(blob: *mut ListBlob, additional: u64);
    }

    /// C-compatible list
    #[repr(C)]
    pub struct List<T>
    where
        T: ExternalListMember,
    {
        pub(crate) blob: ListBlob,
        pub(crate) _t: std::marker::PhantomData<T>,
    }

    use crate::blobs::HasBlob;

    impl<T> List<T>
    where
        T: ExternalListMember,
    {
        /// Equivalent of Vec::new
        pub fn new() -> Self {
            let blob = unsafe { (T::get_new_fn())() };
            Self::from_blob(blob)
        }

        /// Equivalent of Vec::with_capacity
        pub fn with_capacity(capacity: usize) -> Self {
            let blob = unsafe { (T::get_with_capacity_fn())(capacity as u64) };
            Self::from_blob(blob)
        }

        pub(crate) fn from_raw(ptr: *mut T, size: usize) -> Self {
            let blob = unsafe { (T::get_from_raw_fn())(ptr, size as u64) };
            Self::from_blob(blob)
        }

        pub(crate) fn shrink_to_fit(&mut self) {
            unsafe { (T::get_shrink_to_fit_fn())(&mut self.blob) };
        }

        /// Equivalent of Vec::push
        pub fn push(&mut self, item: T) {
            unsafe { (T::get_push_fn())(&mut self.blob, item) };
        }

        /// Equivalent of Vec::push
        pub fn pop(&mut self) -> Option<T> {
            if self.is_empty() {
                None
            } else {
                let item = unsafe { (T::get_pop_fn())(&mut self.blob) };
                Some(item)
            }
        }

        /// Equivalent of Vec::rmeove
        pub fn remove(&mut self, index: usize) -> T {
            unsafe { (T::get_remove_fn())(&mut self.blob, index as u64) }
        }

        pub(crate) fn as_ptr(&self) -> *const T {
            unsafe { (T::get_as_ptr_fn())(&self.blob) }
        }

        pub(crate) fn take_ptr(&mut self) -> *mut T {
            unsafe { (T::get_take_ptr_fn())(&mut self.blob) }
        }

        /// Equivalent of Vec::len
        pub fn len(&self) -> usize {
            unsafe { (T::get_len_fn())(&self.blob) as usize }
        }

        /// Equivalent of Vec::is_empty
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// Equivalent of Vec::capacity
        pub fn capacity(&self) -> usize {
            unsafe { (T::get_capacity_fn())(&self.blob) as usize }
        }

        /// Equivalent of Vec::reserve
        pub fn reserve(&mut self, additional: usize) {
            unsafe { (T::get_reserve_fn())(&mut self.blob, additional as u64) };
        }

        /// Equivalent of Vec::append
        pub fn append(&mut self, other: &mut Self) {
            let other_len = other.len();
            let other_ptr = other.take_ptr();
            self.reserve(other_len);
            let other = unsafe { Vec::from_raw_parts(other_ptr, other_len, other_len) };
            for item in other {
                self.push(item);
            }
        }
    }

    macro_rules! list_count {
        () => {
            0usize
        };
        ( $x:expr $( ,$xs:expr )* $(,)? ) => {
            (1usize + list_count!( $($xs),* ))
        };
    }

    macro_rules! list {
        () => {
            $crate::containers::ExternalList::new()
        };
        ($elem:expr; $n:expr) => {
            compile_error!("list! macro doesn't support [item; N] format")
        };
        ( $x:expr $( ,$xs:expr )* $(,)? ) => {{
            let capacity = list_count!($x, $($xs),*);
            let mut list = $crate::containers::ExternalList::with_capacity(capacity);
            list.push($x);
            $( list.push($xs); )*
            list
        }};
    }

    pub(crate) use {list, list_count};

    impl<T> Drop for List<T>
    where
        T: ExternalListMember,
    {
        fn drop(&mut self) {
            unsafe { (T::get_drop_fn())(&mut self.blob) }
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

    impl<T: ExternalListMember> IntoIterator for List<T>
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

    impl<T> List<T>
    where
        T: ExternalListMember,
    {
        /// Equivalent of Vec::iter
        #[allow(dead_code)]
        fn iter(&self) -> std::slice::Iter<'_, T> {
            self.as_ref().iter()
        }
    }

    impl<T> Default for List<T>
    where
        T: ExternalListMember,
    {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> From<Vec<T>> for List<T>
    where
        T: ExternalListMember,
    {
        fn from(mut vec: Vec<T>) -> Self {
            vec.shrink_to_fit();
            let ptr = vec.as_mut_ptr();
            let len = vec.len();
            std::mem::forget(vec);
            Self::from_raw(ptr, len)
        }
    }

    impl<T, const N: usize> From<[T; N]> for List<T>
    where
        T: ExternalListMember,
    {
        fn from(array: [T; N]) -> Self {
            Self::from(Vec::<T>::from(array))
        }
    }

    impl<T, const N: usize> From<&[T; N]> for List<T>
    where
        T: ExternalListMember + Clone,
    {
        fn from(array: &[T; N]) -> Self {
            Self::from(Vec::<T>::from(array.to_owned()))
        }
    }

    impl<T> Clone for List<T>
    where
        T: ExternalListMember + Clone,
    {
        fn clone(&self) -> Self {
            let copied = self.as_ref().to_vec();
            Self::from(copied)
        }
    }

    impl<T> From<List<T>> for Vec<T>
    where
        T: ExternalListMember,
    {
        fn from(mut list: List<T>) -> Self {
            list.shrink_to_fit();
            let len = list.len();
            let ptr = list.take_ptr();
            unsafe { Vec::from_raw_parts(ptr, len, len) }
        }
    }

    impl<T> std::ops::Deref for List<T>
    where
        T: ExternalListMember,
    {
        type Target = [T];

        fn deref(&self) -> &[T] {
            unsafe { std::slice::from_raw_parts(self.as_ptr(), self.len()) }
        }
    }

    impl<T> std::fmt::Debug for List<T>
    where
        T: ExternalListMember + std::fmt::Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&**self, f)
        }
    }

    impl<T> PartialEq for List<T>
    where
        T: ExternalListMember + PartialEq,
    {
        fn eq(&self, other: &Self) -> bool {
            self.as_ref() == other.as_ref()
        }
    }

    impl<T> PartialEq<List<T>> for [T]
    where
        T: ExternalListMember + PartialEq,
    {
        fn eq(&self, other: &List<T>) -> bool {
            self == other.as_ref()
        }
    }

    impl<T> Eq for List<T> where T: ExternalListMember + Eq {}

    impl<T> PartialEq<&[T]> for List<T>
    where
        T: ExternalListMember + PartialEq,
    {
        fn eq(&self, other: &&[T]) -> bool {
            self.as_ref() == *other
        }
    }

    impl<T> PartialEq<[T]> for List<T>
    where
        T: ExternalListMember + PartialEq,
    {
        fn eq(&self, other: &[T]) -> bool {
            self.as_ref() == other
        }
    }

    impl<T> PartialEq<Vec<T>> for List<T>
    where
        T: ExternalListMember + PartialEq,
    {
        fn eq(&self, other: &Vec<T>) -> bool {
            self.as_ref() == other.as_slice()
        }
    }

    impl<T> PartialEq<List<T>> for Vec<T>
    where
        T: ExternalListMember + PartialEq,
    {
        fn eq(&self, other: &List<T>) -> bool {
            self.as_slice() == other.as_ref()
        }
    }

    impl<T, I> std::ops::Index<I> for List<T>
    where
        T: ExternalListMember,
        I: std::slice::SliceIndex<[T]>,
    {
        type Output = I::Output;

        fn index(&self, index: I) -> &Self::Output {
            std::ops::Index::index(&**self, index)
        }
    }

    impl<T> ListAPI<T> for List<T>
    where
        T: ExternalListMember + Clone,
    {
        fn take_first(self) -> T {
            if self.is_empty() {
                panic!("can't get the first item from an empty list")
            } else {
                unsafe { self.as_ptr().as_ref() }.unwrap().clone()
            }
        }
    }

    use super::ExternalSharedByteList;
    use super::ListAPI;

    macro_rules! gen_list_impl_for {
        (
            $t:ty,
            new = $new:ident,
            drop = $drop:ident,
            with_capacity = $with_capacity:ident,
            from_raw = $from_raw:ident,
            push = $push:ident,
            pop = $pop:ident,
            remove = $remove:ident,
            shrink_to_fit = $shrink_to_fit:ident,
            as_ptr = $as_ptr:ident,
            take_ptr = $take_ptr:ident,
            len = $len:ident,
            capacity = $capacity:ident,
            reserve = $reserve:ident
        ) => {
            use super::{ExternalListMember, ListBlob};

            extern "C" {
                fn $new() -> ListBlob;
                fn $drop(blob: *mut ListBlob);
                fn $with_capacity(capacity: u64) -> ListBlob;
                fn $from_raw(ptr: *mut $t, size: u64) -> ListBlob;
                fn $push(blob: *mut ListBlob, item: $t);
                fn $pop(blob: *mut ListBlob) -> $t;
                fn $shrink_to_fit(blob: *mut ListBlob);
                fn $remove(blob: *mut ListBlob, index: u64) -> $t;
                fn $as_ptr(blob: *const ListBlob) -> *const $t;
                fn $take_ptr(blob: *mut ListBlob) -> *mut $t;
                fn $len(blob: *const ListBlob) -> u64;
                fn $capacity(blob: *const ListBlob) -> u64;
                fn $reserve(blob: *mut ListBlob, additional: u64);
            }

            impl ExternalListMember for $t {
                fn get_new_fn() -> unsafe extern "C" fn() -> ListBlob {
                    $new
                }
                fn get_drop_fn() -> unsafe extern "C" fn(*mut ListBlob) {
                    $drop
                }
                fn get_with_capacity_fn() -> unsafe extern "C" fn(capacity: u64) -> ListBlob {
                    $with_capacity
                }
                fn get_from_raw_fn() -> unsafe extern "C" fn(ptr: *mut Self, size: u64) -> ListBlob
                {
                    $from_raw
                }
                fn get_push_fn() -> unsafe extern "C" fn(blob: *mut ListBlob, item: Self) {
                    $push
                }
                fn get_pop_fn() -> unsafe extern "C" fn(blob: *mut ListBlob) -> Self {
                    $pop
                }
                fn get_remove_fn() -> unsafe extern "C" fn(blob: *mut ListBlob, index: u64) -> Self
                {
                    $remove
                }
                fn get_shrink_to_fit_fn() -> unsafe extern "C" fn(blob: *mut ListBlob) {
                    $shrink_to_fit
                }
                fn get_as_ptr_fn() -> unsafe extern "C" fn(blob: *const ListBlob) -> *const Self {
                    $as_ptr
                }
                fn get_take_ptr_fn() -> unsafe extern "C" fn(blob: *mut ListBlob) -> *mut Self {
                    $take_ptr
                }
                fn get_len_fn() -> unsafe extern "C" fn(blob: *const ListBlob) -> u64 {
                    $len
                }
                fn get_capacity_fn() -> unsafe extern "C" fn(blob: *const ListBlob) -> u64 {
                    $capacity
                }
                fn get_reserve_fn() -> unsafe extern "C" fn(blob: *mut ListBlob, additional: u64) {
                    $reserve
                }
            }

            #[cfg(test)]
            mod tests {
                use super::new_item;
                use crate::containers::{helpers::ListAPI, ExternalList as List};
                use std::ops::Deref;

                #[test]
                fn test_new() {
                    let list = List::<$t>::new();
                    drop(list);
                }

                #[test]
                fn test_with_capacity() {
                    let list = List::<$t>::with_capacity(10);
                    drop(list);
                }

                fn list_with_items(n: usize) -> List<$t> {
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
                    let list = List::<$t>::with_capacity(10);
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
                    let list = List::<$t>::default();
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

                    list.push(new_item());
                    list.push(new_item());
                    let copy = list.clone();
                    assert_eq!(copy.len(), 2);
                    drop(copy);
                    drop(list);
                }

                #[test]
                fn test_vec_from_list() {
                    let mut list = list_with_items(3);
                    list.reserve(20);
                    list.shrink_to_fit();
                    assert_eq!(Vec::from(list_with_items(3)), vec_with_items(3));
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

                #[test]
                fn test_reserve() {
                    let mut list = list_with_items(5);

                    list.reserve(5);
                    // assert_eq!(list.capacity(), 10);

                    list.shrink_to_fit();
                    // assert_eq!(list.capacity(), 5);
                }

                #[test]
                fn test_append() {
                    let mut list1 = list_with_items(3);
                    let mut list2 = list_with_items(4);
                    list1.append(&mut list2);
                    assert_eq!(list1, list_with_items(7));
                    assert_eq!(list2, list_with_items(0));
                }

                #[test]
                fn test_shrink_to_fit() {
                    let mut list = list_with_items(0);
                    list.shrink_to_fit();
                    assert_eq!(list.capacity(), 0);

                    let mut list = list_with_items(5);
                    list.pop();
                    list.pop();
                    list.shrink_to_fit();
                    assert_eq!(list.capacity(), 3);
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
            new = lib_ruby_parser__external__list__of_nodes__new,
            drop = lib_ruby_parser__external__list__of_nodes__drop,
            with_capacity = lib_ruby_parser__external__list__of_nodes__with_capacity,
            from_raw = lib_ruby_parser__external__list__of_nodes__from_raw,
            push = lib_ruby_parser__external__list__of_nodes__push,
            pop = lib_ruby_parser__external__list__of_nodes__pop,
            remove = lib_ruby_parser__external__list__of_nodes__remove,
            shrink_to_fit = lib_ruby_parser__external__list__of_nodes__shrink_to_fit,
            as_ptr = lib_ruby_parser__external__list__of_nodes__as_ptr,
            take_ptr = lib_ruby_parser__external__list__of_nodes__take_ptr,
            len = lib_ruby_parser__external__list__of_nodes__get_len,
            capacity = lib_ruby_parser__external__list__of_nodes__get_capacity,
            reserve = lib_ruby_parser__external__list__of_nodes__reserve
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
            new = lib_ruby_parser__external__list__of_diagnostics__new,
            drop = lib_ruby_parser__external__list__of_diagnostics__drop,
            with_capacity = lib_ruby_parser__external__list__of_diagnostics__with_capacity,
            from_raw = lib_ruby_parser__external__list__of_diagnostics__from_raw,
            push = lib_ruby_parser__external__list__of_diagnostics__push,
            pop = lib_ruby_parser__external__list__of_diagnostics__pop,
            remove = lib_ruby_parser__external__list__of_diagnostics__remove,
            shrink_to_fit = lib_ruby_parser__external__list__of_diagnostics__shrink_to_fit,
            as_ptr = lib_ruby_parser__external__list__of_diagnostics__as_ptr,
            take_ptr = lib_ruby_parser__external__list__of_diagnostics__take_ptr,
            len = lib_ruby_parser__external__list__of_diagnostics__get_len,
            capacity = lib_ruby_parser__external__list__of_diagnostics__get_capacity,
            reserve = lib_ruby_parser__external__list__of_diagnostics__reserve
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
            new = lib_ruby_parser__external__list__of_comments__new,
            drop = lib_ruby_parser__external__list__of_comments__drop,
            with_capacity = lib_ruby_parser__external__list__of_comments__with_capacity,
            from_raw = lib_ruby_parser__external__list__of_comments__from_raw,
            push = lib_ruby_parser__external__list__of_comments__push,
            pop = lib_ruby_parser__external__list__of_comments__pop,
            remove = lib_ruby_parser__external__list__of_comments__remove,
            shrink_to_fit = lib_ruby_parser__external__list__of_comments__shrink_to_fit,
            as_ptr = lib_ruby_parser__external__list__of_comments__as_ptr,
            take_ptr = lib_ruby_parser__external__list__of_comments__take_ptr,
            len = lib_ruby_parser__external__list__of_comments__get_len,
            capacity = lib_ruby_parser__external__list__of_comments__get_capacity,
            reserve = lib_ruby_parser__external__list__of_comments__reserve
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
            new = lib_ruby_parser__external__list__of_magic_comments__new,
            drop = lib_ruby_parser__external__list__of_magic_comments__drop,
            with_capacity = lib_ruby_parser__external__list__of_magic_comments__with_capacity,
            from_raw = lib_ruby_parser__external__list__of_magic_comments__from_raw,
            push = lib_ruby_parser__external__list__of_magic_comments__push,
            pop = lib_ruby_parser__external__list__of_magic_comments__pop,
            remove = lib_ruby_parser__external__list__of_magic_comments__remove,
            shrink_to_fit = lib_ruby_parser__external__list__of_magic_comments__shrink_to_fit,
            as_ptr = lib_ruby_parser__external__list__of_magic_comments__as_ptr,
            take_ptr = lib_ruby_parser__external__list__of_magic_comments__take_ptr,
            len = lib_ruby_parser__external__list__of_magic_comments__get_len,
            capacity = lib_ruby_parser__external__list__of_magic_comments__get_capacity,
            reserve = lib_ruby_parser__external__list__of_magic_comments__reserve
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
            new = lib_ruby_parser__external__list__of_tokens__new,
            drop = lib_ruby_parser__external__list__of_tokens__drop,
            with_capacity = lib_ruby_parser__external__list__of_tokens__with_capacity,
            from_raw = lib_ruby_parser__external__list__of_tokens__from_raw,
            push = lib_ruby_parser__external__list__of_tokens__push,
            pop = lib_ruby_parser__external__list__of_tokens__pop,
            remove = lib_ruby_parser__external__list__of_tokens__remove,
            shrink_to_fit = lib_ruby_parser__external__list__of_tokens__shrink_to_fit,
            as_ptr = lib_ruby_parser__external__list__of_tokens__as_ptr,
            take_ptr = lib_ruby_parser__external__list__of_tokens__take_ptr,
            len = lib_ruby_parser__external__list__of_tokens__get_len,
            capacity = lib_ruby_parser__external__list__of_tokens__get_capacity,
            reserve = lib_ruby_parser__external__list__of_tokens__reserve
        );
    }
    mod of_source_lines {
        #[cfg(test)]
        fn new_item() -> crate::source::SourceLine {
            crate::source::SourceLine::new(1, 2, false)
        }

        gen_list_impl_for!(
            crate::source::SourceLine,
            new = lib_ruby_parser__external__list__of_source_lines__new,
            drop = lib_ruby_parser__external__list__of_source_lines__drop,
            with_capacity = lib_ruby_parser__external__list__of_source_lines__with_capacity,
            from_raw = lib_ruby_parser__external__list__of_source_lines__from_raw,
            push = lib_ruby_parser__external__list__of_source_lines__push,
            pop = lib_ruby_parser__external__list__of_source_lines__pop,
            remove = lib_ruby_parser__external__list__of_source_lines__remove,
            shrink_to_fit = lib_ruby_parser__external__list__of_source_lines__shrink_to_fit,
            as_ptr = lib_ruby_parser__external__list__of_source_lines__as_ptr,
            take_ptr = lib_ruby_parser__external__list__of_source_lines__take_ptr,
            len = lib_ruby_parser__external__list__of_source_lines__get_len,
            capacity = lib_ruby_parser__external__list__of_source_lines__get_capacity,
            reserve = lib_ruby_parser__external__list__of_source_lines__reserve
        );
    }
    mod of_u8 {
        #[cfg(test)]
        fn new_item() -> u8 {
            42
        }

        gen_list_impl_for!(
            u8,
            new = lib_ruby_parser__external__list__of_bytes__new,
            drop = lib_ruby_parser__external__list__of_bytes__drop,
            with_capacity = lib_ruby_parser__external__list__of_bytes__with_capacity,
            from_raw = lib_ruby_parser__external__list__of_bytes__from_raw,
            push = lib_ruby_parser__external__list__of_bytes__push,
            pop = lib_ruby_parser__external__list__of_bytes__pop,
            remove = lib_ruby_parser__external__list__of_bytes__remove,
            shrink_to_fit = lib_ruby_parser__external__list__of_bytes__shrink_to_fit,
            as_ptr = lib_ruby_parser__external__list__of_bytes__as_ptr,
            take_ptr = lib_ruby_parser__external__list__of_bytes__take_ptr,
            len = lib_ruby_parser__external__list__of_bytes__get_len,
            capacity = lib_ruby_parser__external__list__of_bytes__get_capacity,
            reserve = lib_ruby_parser__external__list__of_bytes__reserve
        );

        use super::ExternalSharedByteList;

        impl super::List<u8> {
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
