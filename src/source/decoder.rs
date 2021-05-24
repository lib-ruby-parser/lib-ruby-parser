use std::error::Error;

use crate::containers::StringPtr;

#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalList;
#[cfg(feature = "compile-with-external-structures")]
type List<T> = ExternalList<T>;
#[cfg(not(feature = "compile-with-external-structures"))]
type List<T> = Vec<T>;

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
pub type CustomDecoderFn = dyn Fn(StringPtr, List<u8>) -> CustomDecoderResult;

/// Custom decoder, a wrapper around a function
pub struct CustomDecoder {
    f: Option<Box<CustomDecoderFn>>,
}

impl CustomDecoder {
    /// Constructs a rewriter based on a given function
    pub fn new(f: Box<CustomDecoderFn>) -> Self {
        Self { f: Some(f) }
    }

    /// Constructs a no-op token rewriter that has no side effect. Default value.
    pub fn none() -> Self {
        Self { f: None }
    }

    /// Returns an optional reference to a function that rewrite tokens
    pub fn as_option(&self) -> Option<&CustomDecoderFn> {
        if let Some(f) = &self.f {
            let f = &**f;
            Some(f)
        } else {
            None
        }
    }

    pub(crate) fn take(&mut self) -> Self {
        Self { f: self.f.take() }
    }
}

impl std::fmt::Debug for CustomDecoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomDecoder")
            .field("f", &self.as_option().map(|_| "function"))
            .finish()
    }
}

impl Default for CustomDecoder {
    fn default() -> Self {
        Self::none()
    }
}

/// Result that is returned from decoding function
#[repr(C)]
#[derive(Debug)]
pub enum CustomDecoderResult {
    /// Ok + decoded bytes
    Ok(List<u8>),

    /// Err + reason
    Err(InputError),
}

impl CustomDecoderResult {
    pub(crate) fn to_result(self) -> Result<List<u8>, InputError> {
        match self {
            Self::Ok(value) => Ok(value),
            Self::Err(err) => Err(err),
        }
    }
}

/// An enum with all possible kinds of errors that can be returned
/// from a decoder
#[derive(Debug)]
#[repr(C)]
pub enum InputError {
    /// Emitted when no custom decoder provided but input has custom encoding.
    ///
    /// You can return this error from your custom decoder if you don't support given encoding.
    UnsupportedEncoding(StringPtr),

    /// Generic error that can be emitted from a custom decoder
    DecodingError(StringPtr),
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for InputError {}

pub fn decode_input(
    input: List<u8>,
    enc: StringPtr,
    decoder: CustomDecoder,
) -> CustomDecoderResult {
    match enc.to_uppercase().as_str() {
        "UTF-8" | "ASCII-8BIT" | "BINARY" => {
            return CustomDecoderResult::Ok(input.into());
        }
        _ => {
            if let Some(f) = decoder.as_option() {
                f(enc, input)
            } else {
                CustomDecoderResult::Err(InputError::UnsupportedEncoding(enc))
            }
        }
    }
}
