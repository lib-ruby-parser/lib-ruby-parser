use lib_ruby_parser_nodes::{
    helpers::{camelcase_to_snakecase, escape_rust_keyword},
    reexports::liquid::value,
    LiquidTemplate,
};

pub(crate) fn codegen(node: &lib_ruby_parser_nodes::Node) {
    let contents = LiquidTemplate::new("codegen/rust/nodes/node_file.liquid")
        .with_global("node", value!(node.to_owned()))
        .render();
    let filename = escape_rust_keyword(&camelcase_to_snakecase(node.camelcase_name));
    let path = format!("src/nodes/types/{}.rs", filename);
    std::fs::write(&path, contents).unwrap();
}
