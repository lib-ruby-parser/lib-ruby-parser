use super::{DebugLevel, InputFile};
use lib_ruby_parser::{Parser, ParserOptions, ParserResult};

pub fn parse(input: InputFile, debug_level: DebugLevel, drop_tokens: bool) -> ParserResult {
    let options = ParserOptions::new(
        input.filepath.into(),
        debug_level.level,
        None,
        None,
        !drop_tokens,
    );
    Parser::new(input.code, options).do_parse()
}
