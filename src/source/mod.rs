pub mod buffer;
mod comment;
mod decoder;
mod range;
mod recognized_encoding;
mod source_line;

pub use range::Range;

pub use decoder::InputError;
pub(crate) use decoder::{decode_input, CustomDecoder};
pub use recognized_encoding::RecognizedEncoding;
pub(crate) use source_line::SourceLine;
