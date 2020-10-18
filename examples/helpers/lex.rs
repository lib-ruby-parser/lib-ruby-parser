use ruby_parser::{Lexer, Parser, Token};

#[allow(dead_code)]
pub fn lex(source: &Vec<u8>, filename: &str, debug: bool) -> Result<(Parser, Vec<Token>), String> {
    let lexer = Lexer::new(&source, None).map_err(|e| e.to_string())?;
    let mut parser = Parser::new(lexer);
    parser.set_debug(debug);
    parser.yylexer.buffer.name = filename.to_string();
    match parser.lex() {
        Some(tokens) => Ok((parser, tokens)),
        None => Err("no tokens".to_owned()),
    }
}
