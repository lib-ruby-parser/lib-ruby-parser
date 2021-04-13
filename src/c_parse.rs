use crate::{
    containers::{List, MaybePtr, Ptr, SharedList, StringPtr},
    debug_level,
    source::CustomDecoderResult,
    token_rewriter::{TokenRewriter, TokenRewriterResult},
    Parser, ParserOptions, ParserResult, Token,
};
/// Foreign parser options, can be casted to Rust ParserOptions
#[derive(Debug)]
#[repr(C)]
pub struct ForeignParserOptions {
    buffer_name: List<u8>,
    debug: debug_level::Type,
    decoder: *mut fn(encoding: StringPtr, input: List<u8>) -> CustomDecoderResult,
    token_rewriter: *mut fn(Ptr<Token>, SharedList<u8>) -> TokenRewriterResult,
    record_tokens: bool,
}

impl From<ForeignParserOptions> for ParserOptions {
    fn from(options: ForeignParserOptions) -> Self {
        let ForeignParserOptions {
            buffer_name,
            debug,
            decoder,
            token_rewriter,
            record_tokens,
        } = options;

        let token_rewriter = if token_rewriter.is_null() {
            TokenRewriter::none()
        } else {
            TokenRewriter::new(unsafe { *token_rewriter })
        };

        ParserOptions {
            buffer_name: String::from_utf8(buffer_name.into())
                .expect("Failed to convert buffer_name into UTF-8 string"),
            debug,
            decoder: MaybePtr::from_raw(decoder),
            token_rewriter,
            record_tokens,
        }
    }
}

/// C-compatible function that parses Ruby
#[no_mangle]
pub extern "C" fn parse(input: List<u8>, options: ForeignParserOptions) -> ParserResult {
    let options = ParserOptions::from(options);
    Parser::new(input, options).do_parse()
}

#[test]
fn test_parse() {
    let input = List::from("2 + 2");
    let options = ForeignParserOptions {
        buffer_name: List::from("(eval)"),
        debug: debug_level::NONE,
        decoder: std::ptr::null_mut(),
        token_rewriter: std::ptr::null_mut(),
        record_tokens: true,
    };
    println!("{:#?}", parse(input, options))
}
