pub(crate) trait IntoBlob {
    type Output: Sized;

    fn into_blob(self) -> Self::Output
    where
        Self: Sized;
}

impl IntoBlob for u8 {
    type Output = u8;

    fn into_blob(self) -> Self::Output {
        self
    }
}

impl<T> IntoBlob for crate::containers::ExternalPtr<T>
where
    T: crate::containers::get_drop_fn::GetDropPtrFn,
{
    type Output = crate::containers::PtrBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl<T> IntoBlob for crate::containers::ExternalMaybePtr<T>
where
    T: crate::containers::get_drop_fn::GetDropMaybePtrFn,
{
    type Output = crate::containers::MaybePtrBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::Bytes {
    type Output = crate::bytes::BytesBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl<T> IntoBlob for crate::containers::ExternalList<T>
where
    T: crate::containers::get_drop_fn::GetDropListFn,
{
    type Output = crate::containers::ListBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::Loc {
    type Output = crate::loc::LocBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::containers::ExternalMaybeLoc {
    type Output = crate::containers::MaybeLocBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::containers::ExternalStringPtr {
    type Output = crate::containers::StringPtrBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::containers::ExternalMaybeStringPtr {
    type Output = crate::containers::MaybeStringPtrBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::Diagnostic {
    type Output = crate::error::diagnostic::DiagnosticBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::ErrorLevel {
    type Output = crate::error::level::ErrorLevelBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::MagicCommentKind {
    type Output = crate::source::magic_comment_kind::MagicCommentKindBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::error::DiagnosticMessage {
    type Output = crate::error::message::DiagnosticMessageBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::InputError {
    type Output = crate::source::InputErrorBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::DecoderResult {
    type Output = crate::source::DecoderResultBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::containers::ExternalSharedByteList {
    type Output = crate::containers::SharedByteListBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::Decoder {
    type Output = crate::source::DecoderBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::MaybeDecoder {
    type Output = crate::source::MaybeDecoderBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::token_rewriter::TokenRewriter {
    type Output = crate::source::token_rewriter::TokenRewriterBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::maybe_token_rewriter::MaybeTokenRewriter {
    type Output = crate::source::maybe_token_rewriter::MaybeTokenRewriterBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::ParserOptions {
    type Output = crate::parser_options::ParserOptionsBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::DecodedInput {
    type Output = crate::source::DecodedInputBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}
