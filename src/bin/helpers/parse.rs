use super::InputFile;
use lib_ruby_parser::{Parser, ParserOptions, ParserResult};

pub(crate) fn parse(input: InputFile, mem: &mut [usize], drop_tokens: bool) -> ParserResult {
    let options = ParserOptions {
        buffer_name: input.filepath,
        record_tokens: !drop_tokens,
        ..Default::default()
    };
    let blob = lib_ruby_parser_ast_arena::Blob::from(mem);
    Parser::new(&input.code, options, &blob).do_parse()
}
