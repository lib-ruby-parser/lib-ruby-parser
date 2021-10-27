use super::InputFile;
use lib_ruby_parser::{Parser, ParserOptions, ParserResult};

#[allow(dead_code)]
pub(crate) fn parse(input: InputFile, drop_tokens: bool) -> ParserResult {
    let options = ParserOptions {
        buffer_name: input.filepath.into(),
        record_tokens: !drop_tokens,
        ..Default::default()
    };
    Parser::new(input.code, options).do_parse()
}
