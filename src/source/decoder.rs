use lib_ruby_parser_ast_arena::Blob;

/// An enum with all possible kinds of errors that can be returned
/// from a decoder
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum InputError<'b> {
    /// Emitted when no custom decoder provided but input has custom encoding.
    ///
    /// You can return this error from your custom decoder if you don't support given encoding.
    UnsupportedEncoding(&'b str),

    /// Generic error that can be emitted from a custom decoder
    DecodingError(&'b str),
}

impl std::fmt::Display for InputError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for InputError<'_> {}

/// Result that is returned from decoding function
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DecoderResult<'b> {
    /// Ok + decoded bytes
    Ok(&'b [u8]),

    /// Err + reason
    Err(InputError<'b>),
}

impl<'b> DecoderResult<'b> {
    pub(crate) fn into_result(self) -> Result<&'b [u8], InputError<'b>> {
        match self {
            Self::Ok(value) => Ok(value),
            Self::Err(err) => Err(err),
        }
    }
}

/// Decoder is what is used if input source has encoding
/// that is not supported out of the box.
///
/// Supported encoding are:
/// 1. UTF-8
/// 2. ASCII-8BIT (or BINARY, it's an alias)
///
/// So if your source looks like this:
///
/// ```text
/// # encoding: koi8-r
/// \xFF = 42
/// ```
///
/// you need to provide a decoder that converts this byte sequence
/// into UTF-8 bytes.
///
/// Decoding function
///
/// Takes encoding name and initial input as arguments
/// and returns `Ok(decoded)` vector of bytes or `Err(error)` that will be returned
/// in the `ParserResult::diagnostics` vector.
pub type DecoderFn<'b> = dyn Fn(&'b str, &'b [u8], &'b Blob<'b>) -> DecoderResult<'b>;

/// Custom decoder, a wrapper around a function
pub struct Decoder<'b> {
    f: Box<DecoderFn<'b>>,
}

impl<'b> Decoder<'b> {
    /// Constructs a rewriter based on a given function
    pub fn new(f: Box<DecoderFn<'b>>) -> Self {
        Self { f }
    }

    pub(crate) fn call(
        &self,
        encoding: &'b str,
        input: &'b [u8],
        blob: &'b Blob<'b>,
    ) -> DecoderResult<'b> {
        (self.f)(encoding, input, blob)
    }
}

impl std::fmt::Debug for Decoder<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decoder").finish()
    }
}

pub fn decode_input<'b>(
    input: &'b [u8],
    enc: &'b str,
    decoder: &mut Option<Decoder<'b>>,
    blob: &'b Blob<'b>,
) -> DecoderResult<'b> {
    if enc.eq_ignore_ascii_case("UTF-8")
        || enc.eq_ignore_ascii_case("ASCII-8BIT")
        || enc.eq_ignore_ascii_case("BINARY")
    {
        return DecoderResult::Ok(input);
    }

    if let Some(decoder) = decoder.as_mut() {
        decoder.call(enc, input, blob)
    } else {
        DecoderResult::Err(InputError::UnsupportedEncoding(enc))
    }
}
