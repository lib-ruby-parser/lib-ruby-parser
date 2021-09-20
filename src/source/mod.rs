pub(crate) mod buffer;
mod comment;
mod comment_type;
mod decoded_input;
pub(crate) mod decoder;
mod decoder_result;
mod input;
mod input_error;
mod magic_comment;
pub(crate) mod magic_comment_kind;
mod source_line;

/// Module to perform token rewriting
pub mod token_rewriter;

pub use comment::Comment;
pub use comment_type::CommentType;
pub use decoded_input::DecodedInput;
pub(crate) use decoder::decode_input;
pub use decoder::Decoder;
pub use decoder_result::DecoderResult;
pub use input::Input;
pub use input_error::InputError;
pub use magic_comment::MagicComment;
pub use magic_comment_kind::MagicCommentKind;
pub use source_line::SourceLine;
