extern crate lib_ruby_parser_nodes;

mod comment;
mod get_loc_fn;
mod messages;
mod node_enum;
mod node_file;
mod node_mod;
mod reserved_words;
mod visitor;

use get_loc_fn::GetLocFn;
use messages::Messages;
use node_enum::NodeEnum;
use node_file::NodeFile;
use node_mod::NodeMod;
use reserved_words::ReservedWordsList;
use visitor::Visitor;

pub fn generate_nodes() {
    let nodes = lib_ruby_parser_nodes::nodes();
    let messages = lib_ruby_parser_nodes::messages();

    std::fs::create_dir_all("src/nodes/types").unwrap();

    for node in nodes.iter() {
        NodeFile::new(node).write();
    }

    NodeMod::new(&nodes).write();
    NodeEnum::new(&nodes).write();
    Visitor::new(&nodes).write();
    GetLocFn::new(&nodes).write();
    ReservedWordsList::new().write();
    Messages::new(&messages).write();
}
