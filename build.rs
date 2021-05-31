mod gen;
use gen::{generate_nodes, generate_parser_y};

#[cfg(any(
    feature = "link-external-c-structures",
    feature = "link-external-cpp-structures"
))]
fn generate_external_structure_defs() {
    println!("cargo:rerun-if-env-changed=LIB_RUBY_PARSER_PTR_SIZE");
    let ptr_size = env!("LIB_RUBY_PARSER_PTR_SIZE");

    println!("cargo:rerun-if-env-changed=LIB_RUBY_PARSER_MAYBE_PTR_SIZE");
    let maybe_ptr_size = env!("LIB_RUBY_PARSER_MAYBE_PTR_SIZE");

    println!("cargo:rerun-if-env-changed=LIB_RUBY_PARSER_LIST_SIZE");
    let list_size = env!("LIB_RUBY_PARSER_LIST_SIZE");

    println!("cargo:rerun-if-env-changed=LIB_RUBY_PARSER_STRING_PTR_SIZE");
    let string_ptr_size = env!("LIB_RUBY_PARSER_STRING_PTR_SIZE");

    println!("cargo:rerun-if-env-changed=LIB_RUBY_PARSER_SHARED_BYTE_LIST_SIZE");
    let shared_byte_list_size = env!("LIB_RUBY_PARSER_SHARED_BYTE_LIST_SIZE");

    let contents = format!(
        "pub(crate) const PTR_SIZE: usize = {ptr_size};
pub(crate) const MAYBE_PTR_SIZE: usize = {maybe_ptr_size};
pub(crate) const LIST_SIZE: usize = {list_size};
pub(crate) const STRING_PTR_SIZE: usize = {string_ptr_size};
pub(crate) const SHARED_BYTE_LIST_SIZE: usize = {shared_byte_list_size};
",
        ptr_size = ptr_size,
        maybe_ptr_size = maybe_ptr_size,
        list_size = list_size,
        string_ptr_size = string_ptr_size,
        shared_byte_list_size = shared_byte_list_size
    );

    std::fs::write("src/containers/size.rs", contents).unwrap();
}

#[cfg(not(any(
    feature = "link-external-c-structures",
    feature = "link-external-cpp-structures"
)))]
fn generate_external_structure_defs() {
    // noop
}

#[cfg(feature = "link-external-c-structures")]
fn link_with_external_structures() {
    println!("cargo:rustc-link-search=external");
    println!("cargo:rustc-link-lib=static=structures-c");
    println!("cargo:rerun-if-changed=external/libstructures-c.a");
}

#[cfg(feature = "link-external-cpp-structures")]
fn link_with_external_structures() {
    println!("cargo:rustc-link-search=external");
    println!("cargo:rustc-link-lib=static=structures-cpp");
    println!("cargo:rerun-if-changed=external/libstructures-cpp.a");

    println!("cargo:rustc-link-lib=dylib=c++");
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}

#[cfg(not(any(
    feature = "link-external-c-structures",
    feature = "link-external-cpp-structures"
)))]
fn link_with_external_structures() {
    // noop
}

fn main() {
    generate_parser_y();
    generate_nodes();

    generate_external_structure_defs();
    link_with_external_structures();
}
