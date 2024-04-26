use super::InputFile;
use lib_ruby_parser::{ByteArray, Parser, ParserOptions, ParserResult};
use lib_ruby_parser_ast::Blob;

pub(crate) fn parse<'b>(
    file: InputFile,
    blob: &'b Blob<'b>,
    drop_tokens: bool,
) -> ParserResult<'b> {
    let code = blob.push_bytes(file.code.as_slice());
    let filepath = ByteArray::new(blob);
    filepath.push_str(&file.filepath, blob);
    let options = ParserOptions {
        buffer_name: filepath.try_as_str().unwrap(),
        record_tokens: !drop_tokens,
        ..Default::default()
    };
    Parser::new(code, options, &blob).do_parse()
}
