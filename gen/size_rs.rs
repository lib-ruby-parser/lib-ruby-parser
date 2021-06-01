#[cfg(any(
    feature = "link-with-external-c-structures",
    feature = "link-with-external-cpp-structures"
))]
pub(crate) fn generate_size_rs() {
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
    feature = "link-with-external-c-structures",
    feature = "link-with-external-cpp-structures"
)))]
pub(crate) fn generate_size_rs() {
    println!("Skipping generating size.rs")
}
