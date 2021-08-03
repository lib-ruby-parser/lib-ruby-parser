mod external;
mod mod_file;
mod native;

pub(crate) fn codegen(node: &lib_ruby_parser_nodes::Node) {
    std::fs::create_dir_all(&format!("src/nodes/types/{}", node.filename)).unwrap();

    external::codegen(node);
    native::codegen(node);
    mod_file::codegen(node);
}
