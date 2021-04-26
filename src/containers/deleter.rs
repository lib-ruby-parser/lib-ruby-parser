use std::ffi::c_void;

pub type Deleter = extern "C" fn(*mut c_void);

pub trait GetDeleter {
    fn get_deleter(&self) -> Deleter;
}

extern "C" fn lib_ruby_parser_containers_ptr_delete_token(ptr: *mut c_void) {
    drop(unsafe { Box::from_raw(ptr as *mut crate::Token) })
}

impl GetDeleter for crate::Token {
    fn get_deleter(&self) -> Deleter {
        lib_ruby_parser_containers_ptr_delete_token
    }
}

extern "C" fn lib_ruby_parser_containers_ptr_delete_node(ptr: *mut c_void) {
    drop(unsafe { Box::from_raw(ptr as *mut crate::Node) })
}

impl GetDeleter for crate::Node {
    fn get_deleter(&self) -> Deleter {
        lib_ruby_parser_containers_ptr_delete_node
    }
}
