use super::{DebugLevel, InputFile};
use lib_ruby_parser::{
    source::maybe_token_rewriter::{MaybeTokenRewriter, MaybeTokenRewriterAPI},
    source::{MaybeDecoder, MaybeDecoderAPI},
    Parser, ParserOptions, ParserResult,
};

#[allow(dead_code)]
pub(crate) fn parse(input: InputFile, debug_level: DebugLevel, drop_tokens: bool) -> ParserResult {
    let options = ParserOptions::new(
        input.filepath.into(),
        debug_level.level,
        MaybeDecoder::new_none(),
        MaybeTokenRewriter::new_none(),
        !drop_tokens,
    );
    Parser::new(input.code, options).do_parse()
}
