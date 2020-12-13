use crate::source::RecognizedEncoding;
use std::error::Error;
use std::fmt;

type DecodeFn = Box<dyn Fn(RecognizedEncoding, &[u8]) -> Result<Vec<u8>, InputError>>;

pub struct CustomDecoder {
    pub(crate) f: Option<DecodeFn>,
}

impl CustomDecoder {
    pub fn new(f: DecodeFn) -> Self {
        Self { f: Some(f) }
    }
}

impl std::fmt::Debug for CustomDecoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CustomDecoder")
            .field("f", &self.f.as_ref().map(|_| "function"))
            .finish()
    }
}

impl Default for CustomDecoder {
    fn default() -> Self {
        Self { f: None }
    }
}

impl std::clone::Clone for CustomDecoder {
    fn clone(&self) -> Self {
        Self::default()
    }
}

#[derive(Debug)]
pub enum InputError {
    UnableToRecognizeEncoding,
    UnsupportdEncoding(String),
    NoDecoder(RecognizedEncoding),
    DecodingError(String),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for InputError {}

pub fn decode_input(
    input: &[u8],
    enc: &str,
    decoder: &CustomDecoder,
) -> Result<Vec<u8>, InputError> {
    match &enc.to_uppercase()[..] {
        "UTF-8" | "ASCII-8BIT" | "BINARY" => Ok(input.to_vec()),
        _ => {
            let enc = RecognizedEncoding::parse(&enc)
                .ok_or(InputError::UnsupportdEncoding(enc.to_owned()))?;
            if let Some(f) = &decoder.f {
                f(enc, input)
            } else {
                Err(InputError::NoDecoder(enc))
            }
        }
    }
}
