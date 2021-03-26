use lib_ruby_parser_nodes::Node;

pub(crate) struct NodeMod<'a> {
    nodes: &'a [Node],
}

impl<'a> NodeMod<'a> {
    pub(crate) fn new(nodes: &'a [Node]) -> Self {
        Self { nodes }
    }

    pub(crate) fn write(&self) {
        let contents = self
            .nodes
            .iter()
            .map(|node| {
                format!(
                    "mod {mod_name};\npub use {mod_name}::{struct_name};\n",
                    mod_name = node.filename,
                    struct_name = node.struct_name
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        std::fs::write("src/nodes/types/mod.rs", &contents).unwrap();
    }
}
