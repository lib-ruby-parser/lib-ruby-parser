pub(crate) mod buffer;
mod comment;
mod decoded_input;
pub(crate) mod decoder;
mod input;
mod magic_comment;
mod source_line;

pub use comment::{Comment, CommentType};
pub use decoded_input::DecodedInput;
pub(crate) use decoder::decode_input;
pub use decoder::{Decoder, DecoderResult, InputError};
pub use input::Input;
pub use magic_comment::{MagicComment, MagicCommentKind};
pub use source_line::SourceLine;
