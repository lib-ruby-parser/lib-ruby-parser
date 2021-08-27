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
mod maybe_decoder;
mod source_line;

/// Module with external implementation of Option<TokenRewriter>
pub mod maybe_token_rewriter;
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
pub use maybe_decoder::{MaybeDecoder, MaybeDecoderAPI};
pub use source_line::SourceLine;

#[cfg(feature = "compile-with-external-structures")]
pub(crate) use decoded_input::DecodedInputBlob;
#[cfg(feature = "compile-with-external-structures")]
#[allow(unused_imports)]
pub(crate) use decoder::DecoderBlob;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use decoder_result::DecoderResultBlob;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use input_error::InputErrorBlob;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use maybe_decoder::MaybeDecoderBlob;
