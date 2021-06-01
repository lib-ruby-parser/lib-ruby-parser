use super::{DebugLevel, InputFile};
use lib_ruby_parser::{source::CustomDecoder, Lexer, Token};

pub fn tokenize(input: InputFile, debug_level: DebugLevel) -> Result<Vec<Token>, String> {
    print!("tokenizing {} ... ", input.filepath);
    let mut lexer = Lexer::new(input.code, input.filepath, CustomDecoder::none());
    lexer.set_debug(debug_level.level);
    let tokens = lexer.tokenize_until_eof();
    Ok(tokens)
}
