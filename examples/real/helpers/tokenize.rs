use super::InputFile;
use lib_ruby_parser::{Lexer, Token};

#[allow(dead_code)]
pub(crate) fn tokenize(input: InputFile) -> Result<Vec<Token>, String> {
    print!("tokenizing {} ... ", input.filepath);
    let mut lexer = Lexer::new(input.code, input.filepath, None);
    let tokens = lexer.tokenize_until_eof();
    Ok(tokens)
}
