use std::error::Error;

/// A trait to implement custom decoder.
///
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
pub trait CustomDecoder: std::fmt::Debug {
    /// Decoding function
    ///
    /// Takes encoding name and initial input as arguments
    /// and returns `Ok(decoded)` vector of bytes or `Err(error)` that will be returned
    /// in the `ParserResult::diagnostics` vector.
    fn decode(&self, encoding: &str, input: &[u8]) -> Result<Vec<u8>, InputError>;
}

type DecodeFn = Box<dyn Fn(&str, &[u8]) -> Result<Vec<u8>, InputError>>;

/// Default decoder implementation that is based on **your**
/// Rust function or Rust closure.
///
/// Bindings to other languages implement their own structures,
/// this is why this struct has a prefix `Rust`.
pub struct RustFnBasedCustomDecoder {
    pub(crate) f: DecodeFn,
}

impl RustFnBasedCustomDecoder {
    /// Constructs a new decoder based on a function of closure.
    pub fn new(f: DecodeFn) -> Self {
        Self { f }
    }
}

impl CustomDecoder for RustFnBasedCustomDecoder {
    fn decode(&self, encoding: &str, input: &[u8]) -> Result<Vec<u8>, InputError> {
        let f = &self.f;
        f(encoding, input)
    }
}

impl std::fmt::Debug for RustFnBasedCustomDecoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RustFnBasedCustomDecoder").finish()
    }
}

/// An enum with all possible kinds of errors that can be returned
/// from a decoder
#[derive(Debug)]
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
    input: &[u8],
    enc: &str,
    decoder: &Option<Box<dyn CustomDecoder>>,
) -> Result<Vec<u8>, InputError> {
    match &enc.to_uppercase()[..] {
        "UTF-8" | "ASCII-8BIT" | "BINARY" => Ok(input.to_vec()),
        enc => {
            if let Some(f) = &decoder {
                f.decode(enc, input)
            } else {
                Err(InputError::UnsupportedEncoding(enc.to_string()))
            }
        }
    }
}
