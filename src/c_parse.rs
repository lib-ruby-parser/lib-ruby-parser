use crate::{
    containers::{List, Ptr, SharedList, StringPtr},
    debug_level,
    source::{CustomDecoder, CustomDecoderResult},
    token_rewriter::{TokenRewriter, TokenRewriterResult},
    ParserOptions, Token,
};

type ForeignCustomDecoderFn = extern "C" fn(StringPtr, List<u8>) -> CustomDecoderResult;

extern "C" fn dummy_decode(_encoding: StringPtr, input: List<u8>) -> CustomDecoderResult {
    CustomDecoderResult::Ok(input)
}

/// C-compatible custom decoder
#[repr(C)]
#[derive(Debug)]
pub struct ForeignCustomDecoder {
    f: ForeignCustomDecoderFn,
    dummy: bool,
}

impl ForeignCustomDecoder {
    /// Constructs a custom decoder with a given foreign function
    pub fn new(f: ForeignCustomDecoderFn) -> Self {
        Self { f, dummy: false }
    }

    fn none() -> Self {
        Self {
            f: dummy_decode,
            dummy: true,
        }
    }

    fn as_option(&self) -> Option<ForeignCustomDecoderFn> {
        if self.dummy {
            None
        } else {
            Some(self.f)
        }
    }
}

type ForeignTokenRewriterFn = extern "C" fn(Ptr<Token>, SharedList<u8>) -> TokenRewriterResult;

extern "C" fn dummy_rewrite(_token: Ptr<Token>, _input: SharedList<u8>) -> TokenRewriterResult {
    unreachable!()
}

/// C-compatible token rewriter struct
#[repr(C)]
#[derive(Debug)]
pub struct ForeignTokenRewriter {
    f: ForeignTokenRewriterFn,
    dummy: bool,
}

impl ForeignTokenRewriter {
    /// Constructs a token rewriter with a given foreign function
    pub fn new(f: ForeignTokenRewriterFn) -> Self {
        Self { f, dummy: false }
    }

    fn none() -> Self {
        Self {
            f: dummy_rewrite,
            dummy: true,
        }
    }

    fn as_option(&self) -> Option<ForeignTokenRewriterFn> {
        if self.dummy {
            None
        } else {
            Some(self.f)
        }
    }
}

/// Foreign parser options, can be casted to Rust ParserOptions
#[repr(C)]
#[derive(Debug)]
pub struct ForeignParserOptions {
    /// Equivalent of ParserOptions.buffer_name
    pub buffer_name: StringPtr,

    /// Equivalent of ParserOptions.debug
    pub debug: debug_level::Type,

    /// Equivalent of ParserOptions.decoder
    pub decoder: ForeignCustomDecoder,

    /// Equivalent of ParserOptions.token_rewriter
    pub token_rewriter: ForeignTokenRewriter,

    /// Equivalent of ParserOptions.record_tokens
    pub record_tokens: bool,
}

impl Default for ForeignParserOptions {
    fn default() -> Self {
        Self {
            buffer_name: StringPtr::from("(eval)"),
            debug: debug_level::NONE,
            decoder: ForeignCustomDecoder::none(),
            token_rewriter: ForeignTokenRewriter::none(),
            record_tokens: true,
        }
    }
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

        let decoder = if let Some(decoder) = decoder.as_option() {
            let rust_decode = move |encoding: StringPtr, input: List<u8>| decoder(encoding, input);
            CustomDecoder::new(Box::new(rust_decode))
        } else {
            CustomDecoder::none()
        };

        let token_rewriter = if let Some(token_rewriter) = token_rewriter.as_option() {
            let rust_rewrite =
                move |token: Ptr<Token>, input: SharedList<u8>| token_rewriter(token, input);
            TokenRewriter::new(Box::new(rust_rewrite))
        } else {
            TokenRewriter::none()
        };

        ParserOptions {
            buffer_name: String::from(buffer_name),
            debug,
            decoder,
            token_rewriter,
            record_tokens,
        }
    }
}
