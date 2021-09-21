use super::InputFile;
use lib_ruby_parser::{Lexer, Token};

#[cfg(feature = "compile-with-external-structures")]
use lib_ruby_parser::containers::ExternalMaybe;
#[cfg(feature = "compile-with-external-structures")]
type Maybe<T> = ExternalMaybe<T>;
#[cfg(not(feature = "compile-with-external-structures"))]
type Maybe<T> = Option<T>;

#[allow(unused_imports)]
use lib_ruby_parser::containers::helpers::MaybeAPI;

#[allow(dead_code)]
pub(crate) fn tokenize(input: InputFile) -> Result<Vec<Token>, String> {
    print!("tokenizing {} ... ", input.filepath);
    let mut lexer = Lexer::new(input.code, input.filepath, Maybe::none());
    let tokens = lexer.tokenize_until_eof();
    Ok(tokens)
}
