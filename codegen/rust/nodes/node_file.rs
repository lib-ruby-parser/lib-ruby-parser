use crate::codegen::rust::nodes::helpers::filename;
use lib_ruby_parser_nodes::{reexports::liquid::value, LiquidTemplate};

pub(crate) fn codegen(node: &lib_ruby_parser_nodes::Node) {
    let contents = LiquidTemplate::new("codegen/rust/nodes/node_file.liquid")
        .with_global("node", value!(node.to_owned()))
        .render();
    let dir = filename(node);
    let path = format!("src/nodes/types/{}.rs", dir);
    std::fs::write(&path, contents).unwrap();
}
