use lib_ruby_parser::{debug_level, Lexer, Token};

pub fn tokenize(
    source: &[u8],
    filename: &str,
    debug: debug_level::Type,
) -> Result<Vec<Token>, String> {
    print!("tokenizing {} ... ", filename);
    let mut lexer = Lexer::new(source, filename, None);
    lexer.set_debug(debug);
    let tokens = lexer.tokenize_until_eof();
    println!("OK");
    Ok(tokens)
}
