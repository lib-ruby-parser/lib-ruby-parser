pub(crate) mod buffer;
mod comment;
mod comment_type;
mod decoded_input;
mod decoder;
mod decoder_result;
mod input;
mod input_error;
mod magic_comment;
pub(crate) mod magic_comment_kind;
mod source_line;

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

#[cfg(feature = "compile-with-external-structures")]
#[allow(unused_imports)]
pub(crate) use decoder::DecoderBlob;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use decoder_result::DecoderResultBlob;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use input_error::InputErrorBlob;
