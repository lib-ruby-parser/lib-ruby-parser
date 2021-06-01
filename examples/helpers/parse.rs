use super::{DebugLevel, InputFile};
use lib_ruby_parser::{Parser, ParserOptions, ParserResult};

pub fn parse(input: InputFile, debug_level: DebugLevel, drop_tokens: bool) -> ParserResult {
    let options = ParserOptions {
        buffer_name: input.filepath,
        debug: debug_level.level,
        record_tokens: !drop_tokens,
        ..Default::default()
    };
    Parser::new(input.code, options).do_parse()
}
