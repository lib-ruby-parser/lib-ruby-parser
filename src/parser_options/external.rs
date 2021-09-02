use super::InternalParserOptions;
use crate::blobs::ParserOptionsBlob;
use crate::blobs::StringPtrBlob;
use crate::blobs::{MaybeDecoderBlob, MaybeTokenRewriterBlob};
use crate::containers::ExternalStringPtr as StringPtr;
use crate::containers::IntoBlob;
use crate::debug_level;
use crate::source::maybe_token_rewriter::MaybeTokenRewriter;
use crate::source::MaybeDecoder;

/// Configuration of the parser
#[repr(C)]
pub struct ParserOptions {
    pub(crate) blob: ParserOptionsBlob,
}

extern "C" {
    fn lib_ruby_parser__external__parser_options__new(
        buffer_name: StringPtrBlob,
        debug: u8,
        decoder: MaybeDecoderBlob,
        token_rewriter: MaybeTokenRewriterBlob,
        record_tokens: bool,
    ) -> ParserOptionsBlob;
    fn lib_ruby_parser__external__parser_options__drop(blob: *mut ParserOptionsBlob);
    fn lib_ruby_parser__external__parser_options__into_internal(
        blob: ParserOptionsBlob,
    ) -> InternalParserOptions;
    fn lib_ruby_parser__external__parser_options__get_buffer_name(
        blob: *const ParserOptionsBlob,
    ) -> *const StringPtrBlob;
    fn lib_ruby_parser__external__parser_options__get_debug(blob: *const ParserOptionsBlob) -> u8;
    fn lib_ruby_parser__external__parser_options__get_decoder(
        blob: *const ParserOptionsBlob,
    ) -> *const MaybeDecoderBlob;
    fn lib_ruby_parser__external__parser_options__get_token_rewriter(
        blob: *const ParserOptionsBlob,
    ) -> *const MaybeTokenRewriterBlob;
    fn lib_ruby_parser__external__parser_options__get_record_tokens(
        blob: *const ParserOptionsBlob,
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
        decoder: MaybeDecoder,
        token_rewriter: MaybeTokenRewriter,
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

    /// Creates ParserOptions from ParserOptionsBlob
    pub fn from_blob(blob: ParserOptionsBlob) -> Self {
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
    pub(crate) fn decoder(&self) -> &MaybeDecoder {
        unsafe {
            (lib_ruby_parser__external__parser_options__get_decoder(&self.blob)
                as *const MaybeDecoder)
                .as_ref()
                .unwrap()
        }
    }
    pub(crate) fn token_rewriter(&self) -> &MaybeTokenRewriter {
        unsafe {
            (lib_ruby_parser__external__parser_options__get_token_rewriter(&self.blob)
                as *const MaybeTokenRewriter)
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
