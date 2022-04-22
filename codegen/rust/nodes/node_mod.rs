use lib_ruby_parser_nodes::LiquidTemplate;

pub(crate) fn codegen() {
    let contents = LiquidTemplate::new("codegen/rust/nodes/node_mod.liquid").render();
    std::fs::write("src/nodes/types/mod.rs", contents).unwrap();
}
