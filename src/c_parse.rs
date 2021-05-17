use crate::{
    containers::{List, Ptr, SharedByteList, StringPtr},
    debug_level,
    source::{CustomDecoder, CustomDecoderResult},
    token_rewriter::{TokenRewriter, TokenRewriterResult},
    ParserOptions, Token,
};

use std::ffi::c_void;

type ForeignCustomDecoderFn =
    extern "C" fn(StringPtr, List<u8>, *mut c_void) -> CustomDecoderResult;

extern "C" fn dummy_decode(
    _encoding: StringPtr,
    input: List<u8>,
    _state: *mut c_void,
) -> CustomDecoderResult {
    CustomDecoderResult::Ok(input)
}

/// C-compatible custom decoder
#[repr(C)]
#[derive(Debug)]
pub struct ForeignCustomDecoder {
    /// Foreign function that does decoding
    pub f: ForeignCustomDecoderFn,

    /// Indicator that decoder is dummy
    pub dummy: bool,

    /// Shared state that is passed to external function
    pub state: *mut c_void,
}

impl ForeignCustomDecoder {
    /// Constructs a custom decoder with a given foreign function
    pub fn new(f: ForeignCustomDecoderFn) -> Self {
        Self {
            f,
            dummy: false,
            state: std::ptr::null_mut(),
        }
    }

    fn none() -> Self {
        Self {
            f: dummy_decode,
            dummy: true,
            state: std::ptr::null_mut(),
        }
    }

    fn call(&self, encoding: StringPtr, input: List<u8>) -> CustomDecoderResult {
        if self.dummy {
            panic!("Can't run dummy decoder")
        } else {
            let f = self.f;
            f(encoding, input, self.state)
        }
    }
}

type ForeignTokenRewriterFn =
    extern "C" fn(Ptr<Token>, SharedByteList, *mut c_void) -> TokenRewriterResult;

extern "C" fn dummy_rewrite(
    _token: Ptr<Token>,
    _input: SharedByteList,
    _state: *mut c_void,
) -> TokenRewriterResult {
    unreachable!()
}

/// C-compatible token rewriter struct
#[repr(C)]
#[derive(Debug)]
pub struct ForeignTokenRewriter {
    /// External function that rewrites tokens
    pub f: ForeignTokenRewriterFn,

    /// Indicator that token rewriter is dummy
    pub dummy: bool,

    /// Shared state that is passed to external function
    pub state: *mut c_void,
}

impl ForeignTokenRewriter {
    /// Constructs a token rewriter with a given foreign function
    pub fn new(f: ForeignTokenRewriterFn) -> Self {
        Self {
            f,
            dummy: false,
            state: std::ptr::null_mut(),
        }
    }

    fn none() -> Self {
        Self {
            f: dummy_rewrite,
            dummy: true,
            state: std::ptr::null_mut(),
        }
    }

    fn call(&self, token: Ptr<Token>, input: SharedByteList) -> TokenRewriterResult {
        if self.dummy {
            panic!("Can't run dummy token rewriter")
        } else {
            let f = self.f;
            f(token, input, self.state)
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

        let decoder = if decoder.dummy {
            CustomDecoder::none()
        } else {
            CustomDecoder::new(Box::new(move |encoding: StringPtr, input: List<u8>| {
                decoder.call(encoding, input)
            }))
        };

        let token_rewriter = if token_rewriter.dummy {
            TokenRewriter::none()
        } else {
            TokenRewriter::new(Box::new(move |token: Ptr<Token>, input: SharedByteList| {
                token_rewriter.call(token, input)
            }))
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
