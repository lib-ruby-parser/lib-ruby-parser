use super::{DebugLevel, InputFile};
use lib_ruby_parser::source::Decoder;
use lib_ruby_parser::token_rewriter::TokenRewriter;
use lib_ruby_parser::{Parser, ParserOptions, ParserResult};

pub fn parse(input: InputFile, debug_level: DebugLevel, drop_tokens: bool) -> ParserResult {
    let options = ParserOptions::new(
        input.filepath.into(),
        debug_level.level,
        Decoder::none(),
        TokenRewriter::none(),
        !drop_tokens,
    );
    Parser::new(input.code, options).do_parse()
}
