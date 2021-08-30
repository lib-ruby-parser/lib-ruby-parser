use crate::blobs::{ListBlob, MaybePtrBlob, PtrBlob};

use crate::{
    source::{Comment, MagicComment, SourceLine},
    Diagnostic, Node, Token,
};

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

pub trait GetDropListFn {
    fn get_drop_list_fn() -> unsafe extern "C" fn(*mut ListBlob);
}

macro_rules! list_impl {
    ($t:ty, $fn_name:ident) => {
        extern "C" {
            fn $fn_name(ptr: *mut ListBlob);
        }

        impl GetDropListFn for $t {
            fn get_drop_list_fn() -> unsafe extern "C" fn(*mut ListBlob) {
                $fn_name
            }
        }
    };
}

list_impl!(Token, lib_ruby_parser__external__list__of_tokens__drop);
list_impl!(Node, lib_ruby_parser__external__list__of_nodes__drop);
list_impl!(u8, lib_ruby_parser__external__list__of_bytes__drop);
list_impl!(
    Diagnostic,
    lib_ruby_parser__external__list__of_diagnostics__drop
);
list_impl!(Comment, lib_ruby_parser__external__list__of_comments__drop);
list_impl!(
    MagicComment,
    lib_ruby_parser__external__list__of_magic_comments__drop
);
list_impl!(
    SourceLine,
    lib_ruby_parser__external__list__of_source_lines__drop
);
