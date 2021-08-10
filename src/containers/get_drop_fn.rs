use crate::containers::list::external::ListBlob;
use crate::containers::{MaybePtrBlob, PtrBlob};

pub trait GetFreePtrFn {
    fn get_free_ptr_fn() -> unsafe extern "C" fn(*mut PtrBlob);
}

pub trait GetFreeMaybePtrFn {
    fn get_maybe_free_ptr_fn() -> unsafe extern "C" fn(*mut MaybePtrBlob);
}

pub trait GetDropListInPlaceFn {
    fn get_drop_list_in_place_fn() -> unsafe extern "C" fn(*mut ListBlob);
}

extern "C" {
    fn lib_ruby_parser__internal__containers__ptr__of_node__free(ptr: *mut PtrBlob);
    fn lib_ruby_parser__internal__containers__ptr__of_token__free(ptr: *mut PtrBlob);
    fn lib_ruby_parser__internal__containers__maybe_ptr__of_node__free(ptr: *mut MaybePtrBlob);
    fn lib_ruby_parser__internal__containers__maybe_ptr__of_token__free(ptr: *mut MaybePtrBlob);
}
impl GetFreePtrFn for crate::Node {
    fn get_free_ptr_fn() -> unsafe extern "C" fn(*mut PtrBlob) {
        lib_ruby_parser__internal__containers__ptr__of_node__free
    }
}
impl GetFreeMaybePtrFn for crate::Node {
    fn get_maybe_free_ptr_fn() -> unsafe extern "C" fn(*mut MaybePtrBlob) {
        lib_ruby_parser__internal__containers__maybe_ptr__of_node__free
    }
}

impl GetFreePtrFn for crate::Token {
    fn get_free_ptr_fn() -> unsafe extern "C" fn(*mut PtrBlob) {
        lib_ruby_parser__internal__containers__ptr__of_token__free
    }
}
impl GetFreeMaybePtrFn for crate::Token {
    fn get_maybe_free_ptr_fn() -> unsafe extern "C" fn(*mut MaybePtrBlob) {
        lib_ruby_parser__internal__containers__maybe_ptr__of_token__free
    }
}

macro_rules! define_list_deleter_impl {
    ($t:ty, $fn_name:ident) => {
        extern "C" {
            fn $fn_name(ptr: *mut ListBlob);
        }

        impl GetDropListInPlaceFn for $t {
            fn get_drop_list_in_place_fn() -> unsafe extern "C" fn(*mut ListBlob) {
                $fn_name
            }
        }
    };
}

define_list_deleter_impl!(
    crate::Token,
    lib_ruby_parser__internal__containers__list__of_tokens__drop
);
define_list_deleter_impl!(
    crate::Node,
    lib_ruby_parser__internal__containers__list__of_nodes__drop
);
define_list_deleter_impl!(
    u8,
    lib_ruby_parser__internal__containers__list__of_bytes__drop
);
define_list_deleter_impl!(
    crate::Diagnostic,
    lib_ruby_parser__internal__containers__list__of_diagnostics__drop
);
define_list_deleter_impl!(
    crate::source::Comment,
    lib_ruby_parser__internal__containers__list__of_comments__drop
);
define_list_deleter_impl!(
    crate::source::MagicComment,
    lib_ruby_parser__internal__containers__list__of_magic_comments__drop
);
define_list_deleter_impl!(
    crate::source::SourceLine,
    lib_ruby_parser__internal__containers__list__of_source_lines__drop
);
