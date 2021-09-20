use crate::blobs::{MaybePtrBlob, PtrBlob};

use crate::{Node, Token};

pub trait GetDropPtrFn {
    fn get_drop_ptr_fn() -> unsafe extern "C" fn(*mut PtrBlob);
    fn get_drop_maybe_ptr_fn() -> unsafe extern "C" fn(*mut MaybePtrBlob);
}

macro_rules! ptr_impl {
    ($t:ty, $drop_ptr_fn:ident, $drop_maybe_ptr_fn:ident) => {
        extern "C" {
            fn $drop_ptr_fn(ptr: *mut PtrBlob);
            fn $drop_maybe_ptr_fn(ptr: *mut MaybePtrBlob);
        }

        impl GetDropPtrFn for $t {
            fn get_drop_ptr_fn() -> unsafe extern "C" fn(*mut PtrBlob) {
                $drop_ptr_fn
            }

            fn get_drop_maybe_ptr_fn() -> unsafe extern "C" fn(*mut MaybePtrBlob) {
                $drop_maybe_ptr_fn
            }
        }
    };
}

ptr_impl!(
    Node,
    lib_ruby_parser__external__ptr__of_node__drop,
    lib_ruby_parser__external__maybe_ptr__of_node__drop
);
ptr_impl!(
    Token,
    lib_ruby_parser__external__ptr__of_token__drop,
    lib_ruby_parser__external__maybe_ptr__of_token__drop
);
