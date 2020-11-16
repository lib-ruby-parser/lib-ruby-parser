use lib_ruby_parser::{source::CustomDecoder, Lexer, Token};

#[allow(dead_code)]
pub fn tokenize(source: &[u8], filename: &str, debug: bool) -> Result<Vec<Token>, String> {
    print!("tokenizing {} ... ", filename);
    let mut lexer = Lexer::new(source, filename, CustomDecoder::default());
    lexer.set_debug(debug);
    let tokens = lexer.tokenize_until_eof();
    println!("OK");
    Ok(tokens)
}
