use lib_ruby_parser_nodes::LiquidTemplate;

pub(crate) fn codegen() {
    let contents = LiquidTemplate::new("codegen/rust/nodes/node_enum.liquid").render();
    std::fs::write("src/nodes/node_enum.rs", contents).unwrap()
}
