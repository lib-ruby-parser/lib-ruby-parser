#[cfg(feature = "compile-with-external-structures")]
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

    println!("cargo:rerun-if-env-changed=LIB_RUBY_PARSER_BYTES_SIZE");
    let bytes_size = env!("LIB_RUBY_PARSER_BYTES_SIZE");

    println!("cargo:rerun-if-env-changed=LIB_RUBY_PARSER_TOKEN_SIZE");
    let token_size = env!("LIB_RUBY_PARSER_TOKEN_SIZE");

    println!("cargo:rerun-if-env-changed=LIB_RUBY_PARSER_SOURCE_LINE_SIZE");
    let source_line_size = env!("LIB_RUBY_PARSER_SOURCE_LINE_SIZE");

    println!("cargo:rerun-if-env-changed=LIB_RUBY_PARSER_ERROR_LEVEL_SIZE");
    let error_level_size = env!("LIB_RUBY_PARSER_ERROR_LEVEL_SIZE");

    println!("cargo:rerun-if-env-changed=LIB_RUBY_PARSER_LOC_SIZE");
    let loc_size = env!("LIB_RUBY_PARSER_LOC_SIZE");

    println!("cargo:rerun-if-env-changed=LIB_RUBY_PARSER_COMMENT_TYPE_SIZE");
    let comment_type_size = env!("LIB_RUBY_PARSER_COMMENT_TYPE_SIZE");

    let contents = format!(
        "pub(crate) const PTR_SIZE: usize = {ptr_size};
pub(crate) const MAYBE_PTR_SIZE: usize = {maybe_ptr_size};
pub(crate) const LIST_SIZE: usize = {list_size};
pub(crate) const STRING_PTR_SIZE: usize = {string_ptr_size};
pub(crate) const SHARED_BYTE_LIST_SIZE: usize = {shared_byte_list_size};
pub(crate) const BYTES_SIZE: usize = {bytes_size};
pub(crate) const TOKEN_SIZE: usize = {token_size};
pub(crate) const SOURCE_LINE_SIZE: usize = {source_line_size};
pub(crate) const ERROR_LEVEL_SIZE: usize = {error_level_size};
pub(crate) const LOC_SIZE: usize = {loc_size};
pub(crate) const COMMENT_TYPE_SIZE: usize = {comment_type_size};
",
        ptr_size = ptr_size,
        maybe_ptr_size = maybe_ptr_size,
        list_size = list_size,
        string_ptr_size = string_ptr_size,
        shared_byte_list_size = shared_byte_list_size,
        bytes_size = bytes_size,
        token_size = token_size,
        source_line_size = source_line_size,
        error_level_size = error_level_size,
        loc_size = loc_size,
        comment_type_size = comment_type_size
    );

    println!("Generating sizes.rs");
    std::fs::write("src/containers/size.rs", contents).unwrap();
}

#[cfg(not(feature = "compile-with-external-structures"))]
pub(crate) fn generate_size_rs() {
    println!("Skipping generating size.rs")
}
