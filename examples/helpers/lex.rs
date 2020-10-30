use ruby_parser::{Parser, ParserOptions, Token};

#[allow(dead_code)]
pub fn lex(source: &Vec<u8>, filename: &str, debug: bool) -> Result<(Parser, Vec<Token>), String> {
    let options = ParserOptions {
        buffer_name: filename,
        debug,
        ..Default::default()
    };
    let mut parser = Parser::new(source, options).map_err(|e| e.to_string())?;
    let tokens = parser.lex();
    Ok((parser, tokens))
}
