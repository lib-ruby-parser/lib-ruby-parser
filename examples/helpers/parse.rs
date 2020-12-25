use lib_ruby_parser::{Parser, ParserOptions, ParserResult};

#[allow(dead_code)]
pub fn parse(source: &[u8], filename: &str, debug: bool) -> ParserResult {
    let options = ParserOptions {
        buffer_name: filename.to_owned(),
        debug,
        ..Default::default()
    };
    Parser::new(source, options).do_parse_with_state_validation()
}
