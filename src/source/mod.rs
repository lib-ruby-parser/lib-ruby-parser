pub(crate) mod buffer;
mod comment;
mod decoder;
mod input;
mod magic_comment;
mod source_line;

pub use comment::{Comment, CommentType};
pub(crate) use decoder::decode_input;
pub use decoder::InputError;
pub use decoder::{CustomDecoder, CustomDecoderResult};
pub use input::Input;
pub use magic_comment::{MagicComment, MagicCommentKind};
pub(crate) use source_line::SourceLine;
