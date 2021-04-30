use crate::containers::list::ListBlob;
use std::ffi::c_void;

pub type DropPtrFn = extern "C" fn(*mut c_void);
pub type DropInPlaceFn = extern "C" fn(ptr: *mut c_void);
pub type DropListBlobFn = unsafe extern "C" fn(blob: ListBlob, drop_item_in_place: DropInPlaceFn);

pub trait GetDropFn {
    fn get_drop_ptr_fn() -> DropPtrFn;
    fn get_drop_in_place_fn() -> DropInPlaceFn;
    fn get_drop_list_blob_fn() -> DropListBlobFn;
}

macro_rules! define_deleter_impl {
    ($t:ty, $drop_ptr_fn:ident, $drop_in_place_fn:ident, $drop_list_fn:ident) => {
        extern "C" fn $drop_ptr_fn(ptr: *mut c_void) {
            drop(unsafe { Box::from_raw(ptr as *mut $t) })
        }

        extern "C" fn $drop_in_place_fn(ptr: *mut c_void) {
            unsafe { std::ptr::drop_in_place(ptr as *mut $t) };
        }

        extern "C" {
            fn $drop_list_fn(blob: ListBlob, drop_item_in_place: DropInPlaceFn);
        }

        impl GetDropFn for $t {
            fn get_drop_ptr_fn() -> DropPtrFn {
                $drop_ptr_fn
            }

            fn get_drop_in_place_fn() -> DropInPlaceFn {
                $drop_in_place_fn
            }

            fn get_drop_list_blob_fn() -> DropListBlobFn {
                $drop_list_fn
            }
        }
    };
}

define_deleter_impl!(
    crate::Token,
    lib_ruby_parser_containers_drop_ptr_token,
    lib_ruby_parser_containers_drop_in_place_token,
    lib_ruby_parser_containers_node_list_blob_free
);
define_deleter_impl!(
    crate::Node,
    lib_ruby_parser_containers_drop_ptr_node,
    lib_ruby_parser_containers_drop_in_place_node,
    lib_ruby_parser_containers_diagnostic_list_blob_free
);
define_deleter_impl!(
    u8,
    lib_ruby_parser_containers_drop_ptr_u8,
    lib_ruby_parser_containers_drop_in_place_u8,
    lib_ruby_parser_containers_comment_list_blob_free
);
define_deleter_impl!(
    crate::Diagnostic,
    lib_ruby_parser_containers_drop_ptr_diagnostic,
    lib_ruby_parser_containers_drop_in_place_diagnostic,
    lib_ruby_parser_containers_magic_comment_list_blob_free
);
define_deleter_impl!(
    crate::source::Comment,
    lib_ruby_parser_containers_drop_ptr_comment,
    lib_ruby_parser_containers_drop_in_place_comment,
    lib_ruby_parser_containers_token_list_blob_free
);
define_deleter_impl!(
    crate::source::MagicComment,
    lib_ruby_parser_containers_drop_ptr_magic_comment,
    lib_ruby_parser_containers_drop_in_place_magic_comment,
    lib_ruby_parser_containers_source_line_list_blob_free
);
define_deleter_impl!(
    crate::source::SourceLine,
    lib_ruby_parser_containers_drop_ptr_source_line,
    lib_ruby_parser_containers_drop_in_place_source_line,
    lib_ruby_parser_containers_byte_list_blob_free
);
