use lib_ruby_parser_nodes::LiquidTemplate;

pub(crate) fn codegen() {
    let contents = LiquidTemplate::new("codegen/rust/messages.liquid").render();
    std::fs::write("src/error/messages/message_enum.rs", contents).unwrap();
}
