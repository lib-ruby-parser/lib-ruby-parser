use super::{DebugLevel, InputFile};
use lib_ruby_parser::{Parser, ParserOptions, ParserResult};

#[cfg(feature = "compile-with-external-structures")]
use lib_ruby_parser::containers::ExternalMaybe;
#[cfg(feature = "compile-with-external-structures")]
type Maybe<T> = ExternalMaybe<T>;
#[cfg(not(feature = "compile-with-external-structures"))]
type Maybe<T> = Option<T>;

#[allow(unused_imports)]
use lib_ruby_parser::containers::helpers::MaybeAPI;

#[allow(dead_code)]
pub(crate) fn parse(input: InputFile, debug_level: DebugLevel, drop_tokens: bool) -> ParserResult {
    let options = ParserOptions::new(
        input.filepath.into(),
        debug_level.level,
        Maybe::none(),
        Maybe::none(),
        !drop_tokens,
    );
    Parser::new(input.code, options).do_parse()
}
