use lib_ruby_parser_nodes::Node;

pub struct NodeEnum<'a> {
    nodes: &'a [Node],
}

impl<'a> NodeEnum<'a> {
    pub fn new(nodes: &'a [Node]) -> Self {
        Self { nodes }
    }

    pub fn write(&self) {
        let contents = format!(
            "use crate::nodes::InnerNode;
use crate::nodes::*;

/// Generic combination of all known nodes.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq)]
pub enum Node {{
    {variants}
}}

impl Node {{
    pub(crate) fn inner_ref(&self) -> &dyn InnerNode {{
        match &self {{
            {match_branches}
        }}
    }}
}}
",
            variants = self.variants().join(",\n    "),
            match_branches = self.match_branches().join("\n            ")
        );

        std::fs::write("src/nodes/node_enum_gen.rs", contents).unwrap();
    }

    fn variants(&self) -> Vec<String> {
        self.nodes
            .iter()
            .map(|node| format!("{name}({name})", name = node.struct_name))
            .collect()
    }

    fn match_branches(&self) -> Vec<String> {
        self.nodes
            .iter()
            .map(|node| format!("Node::{name}(inner) => inner,", name = node.struct_name))
            .collect()
    }
}
