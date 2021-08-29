#[cfg(feature = "lib-ruby-parser-bindings")]
mod external;
mod native;

mod internal;
mod mod_file;
mod tests;

use crate::codegen::rust::nodes::helpers::filename;

pub(crate) fn codegen(node: &lib_ruby_parser_nodes::Node) {
    std::fs::create_dir_all(&format!("src/nodes/types/{}", filename(node))).unwrap();

    #[cfg(feature = "lib-ruby-parser-bindings")]
    external::codegen(node);
    native::codegen(node);

    internal::codegen(node);
    mod_file::codegen(node);
    tests::codegen(node);
}
