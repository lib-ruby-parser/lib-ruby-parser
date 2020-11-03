pub mod buffer;
mod comment;
mod decoder;
mod magic_comment;
mod range;
mod recognized_encoding;
mod source_line;

pub use comment::{Comment, CommentType};
pub use decoder::InputError;
pub(crate) use decoder::{decode_input, CustomDecoder};
pub use magic_comment::{MagicComment, MagicCommentKind};
pub use range::Range;
pub use recognized_encoding::RecognizedEncoding;
pub(crate) use source_line::SourceLine;
