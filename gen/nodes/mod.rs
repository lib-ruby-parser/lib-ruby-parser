extern crate lib_ruby_parser_nodes;

mod get_loc_fn;
mod node_enum;
mod node_file;
mod node_mod;
mod visitor;

use get_loc_fn::GetLocFn;
use node_enum::NodeEnum;
use node_file::NodeFile;
use node_mod::NodeMod;
use visitor::Visitor;

pub fn generate_nodes() {
    let nodes = lib_ruby_parser_nodes::nodes().unwrap();

    std::fs::create_dir_all("src/nodes/types").unwrap();

    for node in nodes.iter() {
        NodeFile::new(node).write();
    }

    NodeMod::new(&nodes).write();
    NodeEnum::new(&nodes).write();
    Visitor::new(&nodes).write();
    GetLocFn::new(&nodes).write();
}
