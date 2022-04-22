use lib_ruby_parser_nodes::LiquidTemplate;

pub(crate) fn codegen() {
    let contents = LiquidTemplate::new("codegen/rust/finder.liquid").render();
    std::fs::write("src/traverse/finder/finder_gen.rs", contents).unwrap();
}
