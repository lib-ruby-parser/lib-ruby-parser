/// Generic trait to convert values to blobs and vice versa
pub trait HasBlob {
    /// Associated blob type
    type Blob;

    /// Converts blob type to value type
    fn from_blob(blob: Self::Blob) -> Self
    where
        Self: Sized;

    /// Converts value type to blob type
    fn into_blob(self) -> Self::Blob
    where
        Self: Sized;
}

macro_rules! declare_blob {
    (
        size = $size:expr,
        value = $value:ty,
        blob = $blob:ident,
        doc = $doc:literal
    ) => {
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        #[doc=$doc]
        pub struct $blob {
            pub(crate) bytes: [u8; $size],
        }

        #[cfg(test)]
        impl $blob {
            #[allow(dead_code)]
            pub(crate) fn zeroed() -> Self {
                let bytes = [0; std::mem::size_of::<Self>()];
                Self { bytes }
            }
        }

        impl $crate::blobs::HasBlob for $value {
            type Blob = $blob;

            fn from_blob(blob: Self::Blob) -> Self {
                Self { blob }
            }

            fn into_blob(self) -> Self::Blob {
                let blob = self.blob;
                std::mem::forget(self);
                blob
            }
        }
    };
}

mod gen;
pub use gen::*;

use crate::containers::size;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Blob of the `Ptr`"]
pub struct PtrBlob {
    pub(crate) bytes: [u8; size::PTR_SIZE],
}
impl<T> HasBlob for crate::containers::ExternalPtr<T>
where
    T: crate::containers::get_drop_fn::GetDropPtrFn,
{
    type Blob = PtrBlob;

    fn from_blob(blob: Self::Blob) -> Self {
        Self {
            blob,
            _t: std::marker::PhantomData,
        }
    }

    fn into_blob(self) -> Self::Blob {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Blob of the `MaybePtr`"]
pub struct MaybePtrBlob {
    pub(crate) bytes: [u8; size::MAYBE_PTR_SIZE],
}
impl<T> HasBlob for crate::containers::ExternalMaybePtr<T>
where
    T: crate::containers::get_drop_fn::GetDropMaybePtrFn,
{
    type Blob = MaybePtrBlob;

    fn from_blob(blob: Self::Blob) -> Self {
        Self {
            blob,
            _t: std::marker::PhantomData,
        }
    }

    fn into_blob(self) -> Self::Blob {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[doc = "Blob of the `List`"]
pub struct ListBlob {
    pub(crate) bytes: [u8; size::LIST_SIZE],
}
impl<T> HasBlob for crate::containers::ExternalList<T>
where
    T: crate::containers::list::external::ExternalListMember,
{
    type Blob = ListBlob;

    fn from_blob(blob: Self::Blob) -> Self {
        Self {
            blob,
            _t: std::marker::PhantomData,
        }
    }

    fn into_blob(self) -> Self::Blob {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

declare_blob!(
    size = size::STRING_PTR_SIZE,
    value = crate::containers::ExternalStringPtr,
    blob = StringPtrBlob,
    doc = "Blob of the `StringPtr`"
);
declare_blob!(
    size = size::MAYBE_STRING_PTR_SIZE,
    value = crate::containers::ExternalMaybeStringPtr,
    blob = MaybeStringPtrBlob,
    doc = "Blob of the `MaybeStringPtr`"
);
declare_blob!(
    size = size::SHARED_BYTE_LIST_SIZE,
    value = crate::containers::ExternalSharedByteList,
    blob = SharedByteListBlob,
    doc = "Blob of the `SharedByteList`"
);
declare_blob!(
    size = size::BYTES_SIZE,
    value = crate::Bytes,
    blob = BytesBlob,
    doc = "Blob of the `Bytes`"
);
declare_blob!(
    size = size::TOKEN_SIZE,
    value = crate::Token,
    blob = TokenBlob,
    doc = "Blob of the `Token`"
);
declare_blob!(
    size = size::ERROR_LEVEL_SIZE,
    value = crate::ErrorLevel,
    blob = ErrorLevelBlob,
    doc = "Blob of the `ErrorLevel`"
);
declare_blob!(
    size = size::LOC_SIZE,
    value = crate::Loc,
    blob = LocBlob,
    doc = "Blob of the `Loc`"
);
declare_blob!(
    size = size::MAYBE_LOC_SIZE,
    value = crate::containers::ExternalMaybeLoc,
    blob = MaybeLocBlob,
    doc = "Blob of the `MaybeLoc`"
);
declare_blob!(
    size = size::SOURCE_LINE_SIZE,
    value = crate::source::SourceLine,
    blob = SourceLineBlob,
    doc = "Blob of the `SourceLine`"
);
declare_blob!(
    size = size::COMMENT_TYPE_SIZE,
    value = crate::source::CommentType,
    blob = CommentTypeBlob,
    doc = "Blob of the `CommentType`"
);
declare_blob!(
    size = size::COMMENT_SIZE,
    value = crate::source::Comment,
    blob = CommentBlob,
    doc = "Blob of the `Comment`"
);
declare_blob!(
    size = size::MAGIC_COMMENT_KIND_SIZE,
    value = crate::source::MagicCommentKind,
    blob = MagicCommentKindBlob,
    doc = "Blob of the `MagicCommentKind`"
);
declare_blob!(
    size = size::MAGIC_COMMENT_SIZE,
    value = crate::source::MagicComment,
    blob = MagicCommentBlob,
    doc = "Blob of the `MagicComment`"
);
declare_blob!(
    size = size::DIAGNOSTIC_MESSAGE_SIZE,
    value = crate::error::DiagnosticMessage,
    blob = DiagnosticMessageBlob,
    doc = "Blob of the `DiagnosticMessage`"
);
declare_blob!(
    size = size::DIAGNOSTIC_SIZE,
    value = crate::error::Diagnostic,
    blob = DiagnosticBlob,
    doc = "Blob of the `Diagnostic`"
);
declare_blob!(
    size = size::NODE_SIZE,
    value = crate::Node,
    blob = NodeBlob,
    doc = "Blob of the `Node`"
);
declare_blob!(
    size = size::INPUT_ERROR_SIZE,
    value = crate::source::InputError,
    blob = InputErrorBlob,
    doc = "Blob of the `InputError`"
);
declare_blob!(
    size = size::DECODER_RESULT_SIZE,
    value = crate::source::DecoderResult,
    blob = DecoderResultBlob,
    doc = "Blob of the `DecoderResult`"
);
declare_blob!(
    size = size::DECODER_SIZE,
    value = crate::source::Decoder,
    blob = DecoderBlob,
    doc = "Blob of the `Decoder`"
);
declare_blob!(
    size = size::REWRITE_ACTION_SIZE,
    value = crate::source::token_rewriter::RewriteAction,
    blob = RewriteActionBlob,
    doc = "Blob of the `RewriteAction`"
);
declare_blob!(
    size = size::LEX_STATE_ACTION_SIZE,
    value = crate::source::token_rewriter::LexStateAction,
    blob = LexStateActionBlob,
    doc = "Blob of the `LexStateAction`"
);
declare_blob!(
    size = size::TOKEN_REWRITER_RESULT_SIZE,
    value = crate::source::token_rewriter::TokenRewriterResult,
    blob = TokenRewriterResultBlob,
    doc = "Blob of the `TokenRewriterResult`"
);
declare_blob!(
    size = size::TOKEN_REWRITER_SIZE,
    value = crate::source::token_rewriter::TokenRewriter,
    blob = TokenRewriterBlob,
    doc = "Blob of the `TokenRewriter`"
);
declare_blob!(
    size = size::MAYBE_DECODER_SIZE,
    value = crate::source::MaybeDecoder,
    blob = MaybeDecoderBlob,
    doc = "Blob of the `MaybeDecoder`"
);
declare_blob!(
    size = size::MAYBE_TOKEN_REWRITER_SIZE,
    value = crate::source::maybe_token_rewriter::MaybeTokenRewriter,
    blob = MaybeTokenRewriterBlob,
    doc = "Blob of the `MaybeTokenRewriter`"
);
declare_blob!(
    size = size::PARSER_OPTIONS_SIZE,
    value = crate::ParserOptions,
    blob = ParserOptionsBlob,
    doc = "Blob of the `ParserOptions`"
);
declare_blob!(
    size = size::DECODED_INPUT_SIZE,
    value = crate::source::DecodedInput,
    blob = DecodedInputBlob,
    doc = "Blob of the `DecodedInput`"
);
declare_blob!(
    size = size::PARSER_RESULT_SIZE,
    value = crate::ParserResult,
    blob = ParserResultBlob,
    doc = "Blob of the `ParserResult`"
);

// Dummy implementation for u8 to simplify codegen
// This way u8 is a blob-like structure that has Blob = u8
// (i.e. there's no casting)

impl HasBlob for u8 {
    type Blob = u8;

    fn from_blob(blob: Self::Blob) -> Self {
        blob
    }

    fn into_blob(self) -> Self::Blob {
        self
    }
}

/// Shortcut helper to get a blob of a given type
pub type Blob<T> = <T as HasBlob>::Blob;
