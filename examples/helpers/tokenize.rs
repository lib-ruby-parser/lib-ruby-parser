use ruby_parser::{Lexer, Token};

#[allow(dead_code)]
pub fn tokenize(source: &Vec<u8>, filename: &str, debug: bool) -> Result<Vec<Token>, ()> {
    print!("tokenizing {} ... ", filename);
    let mut lexer = Lexer::new(&source, None).unwrap();
    lexer.set_debug(debug);
    let tokens = lexer.tokenize_until_eof();
    println!("OK");
    Ok(tokens)
}
