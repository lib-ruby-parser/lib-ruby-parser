use lib_ruby_parser::{debug_level, source::Input, Parser, ParserOptions, ParserResult, Token};

#[allow(dead_code)]
pub fn lex(source: &[u8], filename: &str, debug: debug_level::Type) -> (Vec<Token>, Input) {
    let options = ParserOptions {
        buffer_name: filename.to_owned(),
        debug,
        ..Default::default()
    };
    let parser = Parser::new(source, options);
    let ParserResult { tokens, input, .. } = parser.do_parse();
    (tokens, input)
}
