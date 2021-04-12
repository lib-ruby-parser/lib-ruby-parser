use lib_ruby_parser::{debug_level, Parser, ParserOptions, ParserResult};

pub fn parse(
    source: &[u8],
    filename: &str,
    debug: debug_level::Type,
    drop_tokens: bool,
) -> ParserResult {
    let options = ParserOptions {
        buffer_name: filename.to_string(),
        debug,
        record_tokens: !drop_tokens,
        ..Default::default()
    };
    Parser::new(source, options).do_parse()
}
