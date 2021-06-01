#[cfg(feature = "lib-ruby-parser-nodes")]
extern crate lib_ruby_parser_nodes;

#[cfg(feature = "lib-ruby-parser-nodes")]
mod comment;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod get_loc_fn;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod messages;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod node_enum;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod node_file;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod node_mod;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod reserved_words;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod visitor;

#[cfg(feature = "lib-ruby-parser-nodes")]
pub(crate) fn generate_nodes() {
    use get_loc_fn::GetLocFn;
    use messages::Messages;
    use node_enum::NodeEnum;
    use node_file::NodeFile;
    use node_mod::NodeMod;
    use reserved_words::ReservedWordsList;
    use visitor::Visitor;

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

#[cfg(not(feature = "lib-ruby-parser-nodes"))]
pub(crate) fn generate_nodes() {
    println!("Skipping generating node-based Rust source files")
}
