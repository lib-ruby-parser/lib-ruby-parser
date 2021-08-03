#[cfg(feature = "lib-ruby-parser-nodes")]
extern crate lib_ruby_parser_nodes;

#[cfg(feature = "lib-ruby-parser-nodes")]
mod get_loc_fn;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod node_enum;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod node_file;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod node_mod;

#[cfg(feature = "lib-ruby-parser-nodes")]
pub(crate) fn codegen() {
    use node_file::NodeFile;

    let nodes = lib_ruby_parser_nodes::nodes();

    std::fs::create_dir_all("src/nodes/types").unwrap();

    for node in nodes.0.iter() {
        NodeFile::new(node).write();
    }

    node_mod::codegen();
    node_enum::codegen();
    get_loc_fn::codegen();
}

#[cfg(not(feature = "lib-ruby-parser-nodes"))]
pub(crate) fn codegen() {
    println!("Skipping generating node-based Rust source files")
}
