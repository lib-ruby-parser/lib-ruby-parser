use super::InputFile;
use lib_ruby_parser::{Parser, ParserOptions, ParserResult};
use lib_ruby_parser_ast_arena::Blob;

pub(crate) fn parse<'b>(
    file: InputFile,
    blob: &'b Blob<'b>,
    drop_tokens: bool,
) -> ParserResult<'b> {
    let code = blob.push_bytes(file.code.as_slice());
    let filepath = blob.push_str(&file.filepath);
    let options = ParserOptions {
        buffer_name: filepath,
        record_tokens: !drop_tokens,
        ..Default::default()
    };
    Parser::new(code, options, &blob).do_parse()
}
