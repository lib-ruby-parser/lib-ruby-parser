use std::error::Error;

pub trait CustomDecoder: std::fmt::Debug {
    fn decode(&self, encoding: &str, input: &[u8]) -> Result<Vec<u8>, InputError>;
}

type DecodeFn = Box<dyn Fn(&str, &[u8]) -> Result<Vec<u8>, InputError>>;

pub struct RustFnBasedCustomDecoder {
    pub(crate) f: DecodeFn,
}

impl RustFnBasedCustomDecoder {
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

#[derive(Debug)]
pub enum InputError {
    UnableToRecognizeEncoding,
    UnsupportdEncoding(String),
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
                Err(InputError::UnsupportdEncoding(enc.to_owned()))
            }
        }
    }
}
