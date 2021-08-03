fn contents() -> String {
    let nodes = lib_ruby_parser_nodes::nodes();

    format!(
        "use crate::nodes::InnerNode;
use crate::nodes::*;

/// Generic combination of all known nodes.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
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
        variants = nodes.map(&variant).join(",\n    "),
        match_branches = nodes.map(&match_branch).join("\n            ")
    )
}

pub(crate) fn codegen() {
    std::fs::write("src/nodes/node_enum_gen.rs", contents()).unwrap();
}

fn variant(node: &lib_ruby_parser_nodes::Node) -> String {
    format!("{name}({name})", name = node.camelcase_name())
}

fn match_branch(node: &lib_ruby_parser_nodes::Node) -> String {
    format!(
        "Node::{name}(inner) => inner,",
        name = node.camelcase_name()
    )
}
