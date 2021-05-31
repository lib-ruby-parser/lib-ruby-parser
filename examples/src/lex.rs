use lib_ruby_parser::{
    debug_level, source::DecodedInput, Parser, ParserOptions, ParserResult, Token,
};

pub fn lex(source: &[u8], filename: &str, debug: debug_level::Type) -> (Vec<Token>, DecodedInput) {
    let options = ParserOptions {
        buffer_name: filename.to_string(),
        debug,
        ..Default::default()
    };
    let parser = Parser::new(source, options);
    let ParserResult { tokens, input, .. } = parser.do_parse();
    (tokens, input)
}
