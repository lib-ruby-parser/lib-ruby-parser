#[cfg(not(feature = "compile-with-external-structures"))]
pub(crate) mod native {
    type Maybe<T> = Option<T>;

    use super::MaybeAPI;
    impl<T> MaybeAPI<T> for Maybe<T> {
        fn some(value: T) -> Self {
            Some(value)
        }

        fn none() -> Self {
            None
        }
    }
}

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod external {
    use super::MaybeAPI;
    use crate::blobs::{Blob, HasBlob};
    use crate::containers::ExternalPtr as Ptr;
    use crate::{Node, Token};

    pub trait MaybeValue: HasBlob {
        type MaybeBlob;

        fn new_some_fn() -> unsafe extern "C" fn(value: Blob<Self>) -> Self::MaybeBlob;
        fn new_none_fn() -> unsafe extern "C" fn() -> Self::MaybeBlob;
        fn drop_fn() -> unsafe extern "C" fn(blob: *mut Self::MaybeBlob);

        fn is_some_fn() -> unsafe extern "C" fn(blob: *const Self::MaybeBlob) -> bool;
        fn is_none_fn() -> unsafe extern "C" fn(blob: *const Self::MaybeBlob) -> bool;

        fn as_value_fn() -> unsafe extern "C" fn(blob: *const Self::MaybeBlob) -> *const Blob<Self>;
        fn into_value_fn() -> unsafe extern "C" fn(blob: Self::MaybeBlob) -> Blob<Self>;
    }

    /// Generic external Option<T>
    #[repr(C)]
    pub struct Maybe<T>
    where
        T: MaybeValue,
    {
        pub(crate) blob: T::MaybeBlob,
    }

    impl<T> Drop for Maybe<T>
    where
        T: MaybeValue,
    {
        fn drop(&mut self) {
            let f = T::drop_fn();
            unsafe { f(&mut self.blob) }
        }
    }

    impl<T> MaybeAPI<T> for Maybe<T>
    where
        T: MaybeValue,
    {
        fn some(value: T) -> Self {
            let f = T::new_some_fn();
            let blob = unsafe { f(value.into_blob()) };
            Self { blob }
        }

        fn none() -> Self {
            let f = T::new_none_fn();
            let blob = unsafe { f() };
            Self { blob }
        }
    }

    impl<T> Maybe<T>
    where
        T: MaybeValue,
    {
        /// Equivalent of Option::is_some
        pub fn is_some(&self) -> bool {
            let f = T::is_some_fn();
            unsafe { f(&self.blob) }
        }

        /// Equivalent of Option::is_none
        pub fn is_none(&self) -> bool {
            let f = T::is_none_fn();
            unsafe { f(&self.blob) }
        }

        /// Equivalent of Option::as_ref
        pub fn as_ref(&self) -> Option<&T> {
            if self.is_none() {
                return None;
            }
            let f = T::as_value_fn();
            unsafe { (f(&self.blob) as *const T).as_ref() }
        }
    }

    impl<T> Maybe<T>
    where
        T: MaybeValue,
        Self: HasBlob<Blob = T::MaybeBlob>,
    {
        /// Equivalent of Option::unwrap
        pub fn unwrap(self) -> T {
            if self.is_some() {
                let f = T::into_value_fn();
                let t_blob = unsafe { f(self.into_blob()) };
                T::from_blob(t_blob)
            } else {
                panic!("called `Maybe::unwrap()` on a `None` value")
            }
        }
    }

    impl<T> Maybe<T>
    where
        T: MaybeValue,
        Self: HasBlob<Blob = T::MaybeBlob>,
    {
        /// Equivalent of Option::unwrap_or_else
        pub fn unwrap_or_else<F>(self, f: F) -> T
        where
            F: FnOnce() -> T,
        {
            if self.is_some() {
                self.unwrap()
            } else {
                f()
            }
        }

        /// Equivalent of Option::or_else
        pub fn or_else<F>(self, f: F) -> Self
        where
            F: FnOnce() -> Self,
        {
            if self.is_some() {
                self
            } else {
                f()
            }
        }

        /// Equivalent of Option::expect
        pub fn expect(self, msg: &str) -> T {
            if self.is_some() {
                self.unwrap()
            } else {
                panic!("{}", msg)
            }
        }

        /// Equivalent of Option::map
        pub fn map<U, F>(self, f: F) -> Maybe<U>
        where
            U: MaybeValue,
            F: FnOnce(T) -> U,
        {
            if self.is_some() {
                Maybe::some(f(self.unwrap()))
            } else {
                Maybe::none()
            }
        }
    }

    impl<T> Clone for Maybe<T>
    where
        T: MaybeValue + Clone,
    {
        fn clone(&self) -> Self {
            if self.is_some() {
                Self::some(self.as_ref().unwrap().clone())
            } else {
                Self::none()
            }
        }
    }

    impl<T> Default for Maybe<T>
    where
        T: MaybeValue,
    {
        fn default() -> Self {
            Self::none()
        }
    }

    impl<T> std::fmt::Debug for Maybe<T>
    where
        T: MaybeValue + std::fmt::Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.as_ref())
        }
    }

    impl<T> PartialEq for Maybe<T>
    where
        T: MaybeValue + PartialEq,
    {
        fn eq(&self, other: &Self) -> bool {
            self.as_ref() == other.as_ref()
        }
    }

    impl<T> From<Option<T>> for Maybe<T>
    where
        T: MaybeValue,
    {
        fn from(option: Option<T>) -> Self {
            match option {
                Some(value) => Self::some(value),
                None => Self::none(),
            }
        }
    }

    impl From<Option<Box<Node>>> for Maybe<Ptr<Node>> {
        fn from(option: Option<Box<Node>>) -> Self {
            match option {
                Some(ptr) => Self::some(Ptr::new(*ptr)),
                None => Self::none(),
            }
        }
    }

    macro_rules! define_impl {
        (
            t = $t:ty,
            maybe_blob = $maybe_blob:ty,
            new_some = $new_some:ident,
            new_none = $new_none:ident,
            drop = $drop:ident,
            is_some = $is_some:ident,
            is_none = $is_none:ident,
            as_value = $as_value:ident,
            into_value = $into_value:ident
        ) => {
            extern "C" {
                fn $new_some(value: Blob<$t>) -> $maybe_blob;
                fn $new_none() -> $maybe_blob;
                fn $drop(blob: *mut $maybe_blob);
                fn $is_some(blob: *const $maybe_blob) -> bool;
                fn $is_none(blob: *const $maybe_blob) -> bool;
                fn $as_value(blob: *const $maybe_blob) -> *const Blob<$t>;
                fn $into_value(blob: $maybe_blob) -> Blob<$t>;
            }

            impl MaybeValue for $t {
                type MaybeBlob = $maybe_blob;

                fn new_some_fn() -> unsafe extern "C" fn(value: Blob<Self>) -> Self::MaybeBlob {
                    $new_some
                }

                fn new_none_fn() -> unsafe extern "C" fn() -> Self::MaybeBlob {
                    $new_none
                }

                fn drop_fn() -> unsafe extern "C" fn(blob: *mut Self::MaybeBlob) {
                    $drop
                }

                fn is_some_fn() -> unsafe extern "C" fn(blob: *const Self::MaybeBlob) -> bool {
                    $is_some
                }

                fn is_none_fn() -> unsafe extern "C" fn(blob: *const Self::MaybeBlob) -> bool {
                    $is_none
                }

                fn as_value_fn(
                ) -> unsafe extern "C" fn(blob: *const Self::MaybeBlob) -> *const Blob<Self> {
                    $as_value
                }

                fn into_value_fn() -> unsafe extern "C" fn(blob: Self::MaybeBlob) -> Blob<Self> {
                    $into_value
                }
            }
        };
    }

    define_impl!(
        t = crate::Loc,
        maybe_blob = crate::blobs::MaybeLocBlob,
        new_some = lib_ruby_parser__external__maybe__loc__new_some,
        new_none = lib_ruby_parser__external__maybe__loc__new_none,
        drop = lib_ruby_parser__external__maybe__loc__drop,
        is_some = lib_ruby_parser__external__maybe__loc__is_some,
        is_none = lib_ruby_parser__external__maybe__loc__is_none,
        as_value = lib_ruby_parser__external__maybe__loc__as_value,
        into_value = lib_ruby_parser__external__maybe__loc__into_value
    );
    // Ptr is generic
    use crate::blobs::{MaybePtrBlob, PtrBlob};
    extern "C" {
        fn lib_ruby_parser__external__maybe__ptr__new_some(value: PtrBlob) -> MaybePtrBlob;
        fn lib_ruby_parser__external__maybe__ptr__new_none() -> MaybePtrBlob;
        fn lib_ruby_parser__external__maybe__ptr__of_node__drop(blob: *mut MaybePtrBlob);
        fn lib_ruby_parser__external__maybe__ptr__of_token__drop(blob: *mut MaybePtrBlob);
        fn lib_ruby_parser__external__maybe__ptr__is_some(blob: *const MaybePtrBlob) -> bool;
        fn lib_ruby_parser__external__maybe__ptr__is_none(blob: *const MaybePtrBlob) -> bool;
        fn lib_ruby_parser__external__maybe__ptr__as_value(
            blob: *const MaybePtrBlob,
        ) -> *const PtrBlob;
        fn lib_ruby_parser__external__maybe__ptr__into_value(blob: MaybePtrBlob) -> PtrBlob;
    }

    impl MaybeValue for Ptr<Node> {
        type MaybeBlob = MaybePtrBlob;

        fn new_some_fn() -> unsafe extern "C" fn(value: Blob<Self>) -> Self::MaybeBlob {
            lib_ruby_parser__external__maybe__ptr__new_some
        }

        fn new_none_fn() -> unsafe extern "C" fn() -> Self::MaybeBlob {
            lib_ruby_parser__external__maybe__ptr__new_none
        }

        fn drop_fn() -> unsafe extern "C" fn(blob: *mut Self::MaybeBlob) {
            lib_ruby_parser__external__maybe__ptr__of_node__drop
        }

        fn is_some_fn() -> unsafe extern "C" fn(blob: *const Self::MaybeBlob) -> bool {
            lib_ruby_parser__external__maybe__ptr__is_some
        }

        fn is_none_fn() -> unsafe extern "C" fn(blob: *const Self::MaybeBlob) -> bool {
            lib_ruby_parser__external__maybe__ptr__is_none
        }

        fn as_value_fn() -> unsafe extern "C" fn(blob: *const Self::MaybeBlob) -> *const Blob<Self>
        {
            lib_ruby_parser__external__maybe__ptr__as_value
        }

        fn into_value_fn() -> unsafe extern "C" fn(blob: Self::MaybeBlob) -> Blob<Self> {
            lib_ruby_parser__external__maybe__ptr__into_value
        }
    }
    impl MaybeValue for Ptr<Token> {
        type MaybeBlob = MaybePtrBlob;

        fn new_some_fn() -> unsafe extern "C" fn(value: Blob<Self>) -> Self::MaybeBlob {
            lib_ruby_parser__external__maybe__ptr__new_some
        }

        fn new_none_fn() -> unsafe extern "C" fn() -> Self::MaybeBlob {
            lib_ruby_parser__external__maybe__ptr__new_none
        }

        fn drop_fn() -> unsafe extern "C" fn(blob: *mut Self::MaybeBlob) {
            lib_ruby_parser__external__maybe__ptr__of_token__drop
        }

        fn is_some_fn() -> unsafe extern "C" fn(blob: *const Self::MaybeBlob) -> bool {
            lib_ruby_parser__external__maybe__ptr__is_some
        }

        fn is_none_fn() -> unsafe extern "C" fn(blob: *const Self::MaybeBlob) -> bool {
            lib_ruby_parser__external__maybe__ptr__is_none
        }

        fn as_value_fn() -> unsafe extern "C" fn(blob: *const Self::MaybeBlob) -> *const Blob<Self>
        {
            lib_ruby_parser__external__maybe__ptr__as_value
        }

        fn into_value_fn() -> unsafe extern "C" fn(blob: Self::MaybeBlob) -> Blob<Self> {
            lib_ruby_parser__external__maybe__ptr__into_value
        }
    }
    define_impl!(
        t = crate::containers::ExternalStringPtr,
        maybe_blob = crate::blobs::MaybeStringPtrBlob,
        new_some = lib_ruby_parser__external__maybe__string_ptr__new_some,
        new_none = lib_ruby_parser__external__maybe__string_ptr__new_none,
        drop = lib_ruby_parser__external__maybe__string_ptr__drop,
        is_some = lib_ruby_parser__external__maybe__string_ptr__is_some,
        is_none = lib_ruby_parser__external__maybe__string_ptr__is_none,
        as_value = lib_ruby_parser__external__maybe__string_ptr__as_value,
        into_value = lib_ruby_parser__external__maybe__string_ptr__into_value
    );
    define_impl!(
        t = crate::source::Decoder,
        maybe_blob = crate::blobs::MaybeDecoderBlob,
        new_some = lib_ruby_parser__external__maybe__decoder__new_some,
        new_none = lib_ruby_parser__external__maybe__decoder__new_none,
        drop = lib_ruby_parser__external__maybe__decoder__drop,
        is_some = lib_ruby_parser__external__maybe__decoder__is_some,
        is_none = lib_ruby_parser__external__maybe__decoder__is_none,
        as_value = lib_ruby_parser__external__maybe__decoder__as_value,
        into_value = lib_ruby_parser__external__maybe__decoder__into_value
    );
    define_impl!(
        t = crate::source::token_rewriter::TokenRewriter,
        maybe_blob = crate::blobs::MaybeTokenRewriterBlob,
        new_some = lib_ruby_parser__external__maybe__token_rewriter__new_some,
        new_none = lib_ruby_parser__external__maybe__token_rewriter__new_none,
        drop = lib_ruby_parser__external__maybe__token_rewriter__drop,
        is_some = lib_ruby_parser__external__maybe__token_rewriter__is_some,
        is_none = lib_ruby_parser__external__maybe__token_rewriter__is_none,
        as_value = lib_ruby_parser__external__maybe__token_rewriter__as_value,
        into_value = lib_ruby_parser__external__maybe__token_rewriter__into_value
    );

    #[cfg(test)]
    mod tests {
        use super::{Maybe, MaybeAPI, Ptr};

        #[test]
        fn test_maybe_loc() {
            use crate::Loc;

            fn make() -> Loc {
                Loc::new(1, 2)
            }

            let some_loc = Maybe::some(make());
            assert!(some_loc.is_some());
            assert!(!some_loc.is_none());
            assert_eq!(some_loc.clone(), Maybe::some(make()));
            assert_eq!(some_loc.as_ref(), Some(&make()));
            assert_eq!(some_loc.unwrap(), make());

            let none_loc = Maybe::<Loc>::none();
            assert!(none_loc.is_none());
            assert!(!none_loc.is_some());
            assert_eq!(none_loc.clone(), Maybe::none());
            assert_eq!(none_loc.as_ref(), None);
        }

        #[test]
        fn test_maybe_ptr_token() {
            use crate::Token;

            fn make() -> Ptr<Token> {
                Ptr::new(Token::new(
                    280,
                    crate::Bytes::new(crate::containers::ExternalList::from(vec![97, 98, 99])),
                    crate::Loc::new(3, 4),
                    crate::LexState { value: 1 },
                    crate::LexState { value: 2 },
                ))
            }

            let some_token_ptr = Maybe::some(make());
            println!("{:?}", some_token_ptr);
            assert!(some_token_ptr.is_some());
            assert!(!some_token_ptr.is_none());
            assert_eq!(some_token_ptr.clone(), Maybe::some(make()));
            assert_eq!(some_token_ptr.as_ref(), Some(&make()));
            assert_eq!(some_token_ptr.unwrap(), make());

            let none_token_ptr = Maybe::<Ptr<Token>>::none();
            assert!(none_token_ptr.is_none());
            assert!(!none_token_ptr.is_some());
            assert_eq!(none_token_ptr.clone(), Maybe::none());
            assert_eq!(none_token_ptr.as_ref(), None);
        }

        #[test]
        fn test_maybe_ptr_node() {
            use crate::Node;

            fn make() -> Ptr<Node> {
                Ptr::new(Node::new_file(crate::Loc::new(1, 2)))
            }

            let some_node_ptr = Maybe::some(make());
            println!("{:?}", some_node_ptr);
            assert!(some_node_ptr.is_some());
            assert!(!some_node_ptr.is_none());
            assert_eq!(some_node_ptr.clone(), Maybe::some(make()));
            assert_eq!(some_node_ptr.as_ref(), Some(&make()));
            assert_eq!(some_node_ptr.unwrap(), make());

            let none_node_ptr = Maybe::<Ptr<Node>>::none();
            assert!(none_node_ptr.is_none());
            assert!(!none_node_ptr.is_some());
            assert_eq!(none_node_ptr.clone(), Maybe::none());
            assert_eq!(none_node_ptr.as_ref(), None);
        }

        #[test]
        fn test_maybe_string_ptr() {
            use crate::containers::ExternalStringPtr as StringPtr;

            fn make() -> StringPtr {
                StringPtr::from("foobar")
            }

            let some_string_ptr = Maybe::some(make());
            assert!(some_string_ptr.is_some());
            assert!(!some_string_ptr.is_none());
            assert_eq!(some_string_ptr.clone(), Maybe::some(make()));
            assert_eq!(some_string_ptr.as_ref(), Some(&make()));
            assert_eq!(some_string_ptr.unwrap(), make());

            let none_string_ptr = Maybe::<StringPtr>::none();
            assert!(none_string_ptr.is_none());
            assert!(!none_string_ptr.is_some());
            assert_eq!(none_string_ptr.clone(), Maybe::none());
            assert_eq!(none_string_ptr.as_ref(), None);
        }

        #[test]
        fn test_maybe_decoder() {
            use crate::source::decoder::shared::dummy_decoder::*;
            use crate::source::{Decoder, DecoderResult};

            fn make() -> Decoder {
                ok_decoder()
            }

            let some_decoder = Maybe::some(make());
            assert!(some_decoder.is_some());
            assert!(!some_decoder.is_none());
            assert_eq!(
                call_dummy_decoder(some_decoder.unwrap()),
                DecoderResult::new_ok(decoded_output())
            );
        }
    }
}

pub trait MaybeAPI<T> {
    /// Some(T) constructor
    fn some(value: T) -> Self
    where
        Self: Sized;

    /// None constructor
    fn none() -> Self
    where
        Self: Sized;
}
