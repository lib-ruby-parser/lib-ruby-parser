use lib_ruby_parser_nodes::LiquidTemplate;

pub(crate) fn codegen() {
    let contents = LiquidTemplate::new("codegen/rust/visitor.liquid").render();
    std::fs::write("src/traverse/visitor/visit_gen.rs", contents).unwrap();
}
