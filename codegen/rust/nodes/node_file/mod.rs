#[cfg(feature = "compile-with-external-structures")]
mod external;
#[cfg(not(feature = "compile-with-external-structures"))]
mod native;

mod internal;
mod mod_file;
mod tests;

use crate::codegen::rust::nodes::helpers::filename;

pub(crate) fn codegen(node: &lib_ruby_parser_nodes::Node) {
    std::fs::create_dir_all(&format!("src/nodes/types/{}", filename(node))).unwrap();

    #[cfg(feature = "compile-with-external-structures")]
    external::codegen(node);

    #[cfg(not(feature = "compile-with-external-structures"))]
    native::codegen(node);

    internal::codegen(node);
    mod_file::codegen(node);
    tests::codegen(node);
}
