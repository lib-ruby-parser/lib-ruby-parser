extern crate lib_ruby_parser_nodes;

pub(crate) mod helpers;
mod loc_name;
mod node_enum;
mod node_file;
mod node_mod;

pub(crate) fn codegen() {
    let nodes = lib_ruby_parser_nodes::nodes();

    std::fs::create_dir_all("src/nodes/types").unwrap();

    for node in nodes.iter() {
        node_file::codegen(node);
    }

    node_mod::codegen();
    node_enum::codegen();
    loc_name::codegen();
}
