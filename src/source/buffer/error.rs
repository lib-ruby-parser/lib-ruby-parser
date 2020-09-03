use std::fmt;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum BufferError {
    InputFileDoesNotExit,
    UnrecognizedEncoding,
    EncodingError(String),
}

impl fmt::Display for BufferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for BufferError {
}

impl From<std::io::Error> for BufferError {
    fn from(_err: std::io::Error) -> Self {
        BufferError::InputFileDoesNotExit
    }
}
