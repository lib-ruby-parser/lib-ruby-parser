use crate::blobs::{MaybePtrBlob, PtrBlob};

use crate::{Node, Token};

pub trait GetDropPtrFn {
    fn get_drop_ptr_fn() -> unsafe extern "C" fn(*mut PtrBlob);
}

macro_rules! ptr_impl {
    ($t:ty, $fn_name:ident) => {
        extern "C" {
            fn $fn_name(ptr: *mut PtrBlob);
        }

        impl GetDropPtrFn for $t {
            fn get_drop_ptr_fn() -> unsafe extern "C" fn(*mut PtrBlob) {
                $fn_name
            }
        }
    };
}

ptr_impl!(Node, lib_ruby_parser__external__ptr__of_node__drop);
ptr_impl!(Token, lib_ruby_parser__external__ptr__of_token__drop);

pub trait GetDropMaybePtrFn {
    fn get_drop_maybe_ptr_fn() -> unsafe extern "C" fn(*mut MaybePtrBlob);
}

macro_rules! maybe_ptr_impl {
    ($t:ty, $fn_name:ident) => {
        extern "C" {
            fn $fn_name(ptr: *mut MaybePtrBlob);
        }

        impl GetDropMaybePtrFn for $t {
            fn get_drop_maybe_ptr_fn() -> unsafe extern "C" fn(*mut MaybePtrBlob) {
                $fn_name
            }
        }
    };
}

maybe_ptr_impl!(Node, lib_ruby_parser__external__maybe_ptr__of_node__drop);
maybe_ptr_impl!(Token, lib_ruby_parser__external__maybe_ptr__of_token__drop);
