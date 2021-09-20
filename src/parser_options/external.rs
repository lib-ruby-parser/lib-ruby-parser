crate::use_native_or_external!(Maybe);
use super::InternalParserOptions;
use crate::blobs::{Blob, HasBlob};
use crate::containers::ExternalStringPtr as StringPtr;
use crate::debug_level;
use crate::source::token_rewriter::TokenRewriter;
use crate::source::Decoder;

/// Configuration of the parser
#[repr(C)]
pub struct ParserOptions {
    pub(crate) blob: Blob<ParserOptions>,
}

extern "C" {
    fn lib_ruby_parser__external__parser_options__new(
        buffer_name: Blob<StringPtr>,
        debug: u8,
        decoder: Blob<Maybe<Decoder>>,
        token_rewriter: Blob<Maybe<TokenRewriter>>,
        record_tokens: bool,
    ) -> Blob<ParserOptions>;
    fn lib_ruby_parser__external__parser_options__drop(blob: *mut Blob<ParserOptions>);
    fn lib_ruby_parser__external__parser_options__into_internal(
        blob: Blob<ParserOptions>,
    ) -> InternalParserOptions;
    fn lib_ruby_parser__external__parser_options__get_buffer_name(
        blob: *const Blob<ParserOptions>,
    ) -> *const Blob<StringPtr>;
    fn lib_ruby_parser__external__parser_options__get_debug(blob: *const Blob<ParserOptions>)
        -> u8;
    fn lib_ruby_parser__external__parser_options__get_decoder(
        blob: *const Blob<ParserOptions>,
    ) -> *const Blob<Maybe<Decoder>>;
    fn lib_ruby_parser__external__parser_options__get_token_rewriter(
        blob: *const Blob<ParserOptions>,
    ) -> *const Blob<Maybe<TokenRewriter>>;
    fn lib_ruby_parser__external__parser_options__get_record_tokens(
        blob: *const Blob<ParserOptions>,
    ) -> bool;
}

impl Drop for ParserOptions {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__external__parser_options__drop(&mut self.blob) }
    }
}

impl ParserOptions {
    /// Constructs new ParserOptions
    pub fn new(
        buffer_name: StringPtr,
        debug: debug_level::Type,
        decoder: Maybe<Decoder>,
        token_rewriter: Maybe<TokenRewriter>,
        record_tokens: bool,
    ) -> Self {
        let blob = unsafe {
            lib_ruby_parser__external__parser_options__new(
                buffer_name.into_blob(),
                debug,
                decoder.into_blob(),
                token_rewriter.into_blob(),
                record_tokens,
            )
        };
        Self { blob }
    }

    pub(crate) fn buffer_name(&self) -> &StringPtr {
        unsafe {
            (lib_ruby_parser__external__parser_options__get_buffer_name(&self.blob)
                as *const StringPtr)
                .as_ref()
                .unwrap()
        }
    }
    pub(crate) fn debug(&self) -> debug_level::Type {
        unsafe { lib_ruby_parser__external__parser_options__get_debug(&self.blob) }
    }
    pub(crate) fn decoder(&self) -> &Maybe<Decoder> {
        unsafe {
            (lib_ruby_parser__external__parser_options__get_decoder(&self.blob)
                as *const Maybe<Decoder>)
                .as_ref()
                .unwrap()
        }
    }
    pub(crate) fn token_rewriter(&self) -> &Maybe<TokenRewriter> {
        unsafe {
            (lib_ruby_parser__external__parser_options__get_token_rewriter(&self.blob)
                as *const Maybe<TokenRewriter>)
                .as_ref()
                .unwrap()
        }
    }
    pub(crate) fn record_tokens(&self) -> bool {
        unsafe { lib_ruby_parser__external__parser_options__get_record_tokens(&self.blob) }
    }
}

impl std::fmt::Debug for ParserOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParserOptions")
            .field("buffer_name", self.buffer_name())
            .field("debug", &self.debug())
            .field("decoder", self.decoder())
            .field("token_rewriter", self.token_rewriter())
            .field("record_tokens", &self.record_tokens())
            .finish()
    }
}

impl From<ParserOptions> for InternalParserOptions {
    fn from(options: ParserOptions) -> Self {
        unsafe { lib_ruby_parser__external__parser_options__into_internal(options.into_blob()) }
    }
}
