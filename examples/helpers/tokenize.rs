use lib_ruby_parser::{Lexer, Token};

#[allow(dead_code)]
pub fn tokenize(source: &[u8], filename: &str, debug: bool) -> Result<Vec<Token>, String> {
    print!("tokenizing {} ... ", filename);
    let mut lexer = Lexer::new(source, filename, None);
    lexer.set_debug(debug);
    let tokens = lexer.tokenize_until_eof();
    println!("OK");
    Ok(tokens)
}
