use lib_ruby_parser::{Parser, ParserOptions, Token};

#[allow(dead_code)]
pub fn lex(source: &[u8], filename: &str, debug: bool) -> Result<(Parser, Vec<Token>), String> {
    let options = ParserOptions {
        buffer_name: filename.to_owned(),
        debug,
        ..Default::default()
    };
    let mut parser = Parser::new(source, options).map_err(|e| e.to_string())?;
    let tokens = parser.do_parse().tokens;
    Ok((parser, tokens))
}
