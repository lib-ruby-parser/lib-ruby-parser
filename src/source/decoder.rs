use std::error::Error;

use crate::containers::{maybe_ptr::AsOption, List, MaybePtr, String};

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
pub type CustomDecoder = fn(String, List<u8>) -> CustomDecoderResult;
pub type CustomDecoderResult = DecoderResult<List<u8>, InputError>;

#[repr(C)]
#[derive(Debug)]
pub enum DecoderResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> DecoderResult<T, E> {
    pub(crate) fn to_result(self) -> Result<T, E> {
        match self {
            DecoderResult::Ok(value) => Ok(value),
            DecoderResult::Err(err) => Err(err),
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
    UnsupportedEncoding(String),

    /// Generic error that can be emitted from a custom decoder
    DecodingError(String),
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for InputError {}

pub fn decode_input(
    input: List<u8>,
    enc: String,
    decoder: MaybePtr<CustomDecoder>,
) -> CustomDecoderResult {
    let enc = match enc.to_uppercase() {
        Ok(value) => value,
        Err(_) => {
            return CustomDecoderResult::Err(InputError::UnsupportedEncoding(String::from(
                "encoding name is invalid",
            )));
        }
    };

    if enc == "UTF-8" || enc == "ASCII-8BIT" || enc == "BINARY" {
        return DecoderResult::Ok(input.into());
    }

    if let Some(f) = decoder.as_option() {
        f(enc, input)
    } else {
        DecoderResult::Err(InputError::UnsupportedEncoding(enc))
    }
}
