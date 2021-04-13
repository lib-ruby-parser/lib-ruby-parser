use lib_ruby_parser::{
    containers::List, debug_level, source::Input, Parser, ParserOptions, ParserResult, Token,
};

pub fn lex(source: &[u8], filename: &str, debug: debug_level::Type) -> (List<Token>, Input) {
    let options = ParserOptions {
        buffer_name: filename.to_string(),
        debug,
        ..Default::default()
    };
    let parser = Parser::new(source, options);
    let ParserResult { tokens, input, .. } = parser.do_parse();
    (tokens, input)
}
