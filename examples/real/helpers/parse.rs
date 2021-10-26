use super::InputFile;
use lib_ruby_parser::{Parser, ParserOptions, ParserResult};

#[allow(dead_code)]
pub(crate) fn parse(input: InputFile, drop_tokens: bool) -> ParserResult {
    let options = ParserOptions::new(input.filepath.into(), None, None, !drop_tokens);
    Parser::new(input.code, options).do_parse()
}
