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
    type Output = crate::blobs::PtrBlob;

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
    type Output = crate::blobs::MaybePtrBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::Bytes {
    type Output = crate::blobs::BytesBlob;

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
    type Output = crate::blobs::ListBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::Loc {
    type Output = crate::blobs::LocBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::containers::ExternalMaybeLoc {
    type Output = crate::blobs::MaybeLocBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::containers::ExternalStringPtr {
    type Output = crate::blobs::StringPtrBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::containers::ExternalMaybeStringPtr {
    type Output = crate::blobs::MaybeStringPtrBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::Diagnostic {
    type Output = crate::blobs::DiagnosticBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::ErrorLevel {
    type Output = crate::blobs::ErrorLevelBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::MagicCommentKind {
    type Output = crate::blobs::MagicCommentKindBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::error::DiagnosticMessage {
    type Output = crate::blobs::DiagnosticMessageBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::InputError {
    type Output = crate::blobs::InputErrorBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::DecoderResult {
    type Output = crate::blobs::DecoderResultBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::containers::ExternalSharedByteList {
    type Output = crate::blobs::SharedByteListBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::Decoder {
    type Output = crate::blobs::DecoderBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::MaybeDecoder {
    type Output = crate::blobs::MaybeDecoderBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::token_rewriter::TokenRewriter {
    type Output = crate::blobs::TokenRewriterBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::maybe_token_rewriter::MaybeTokenRewriter {
    type Output = crate::blobs::MaybeTokenRewriterBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::ParserOptions {
    type Output = crate::blobs::ParserOptionsBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::DecodedInput {
    type Output = crate::blobs::DecodedInputBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::ParserResult {
    type Output = crate::blobs::ParserResultBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::Comment {
    type Output = crate::blobs::CommentBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::MagicComment {
    type Output = crate::blobs::MagicCommentBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::SourceLine {
    type Output = crate::blobs::SourceLineBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::token_rewriter::LexStateAction {
    type Output = crate::blobs::LexStateActionBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::token_rewriter::RewriteAction {
    type Output = crate::blobs::RewriteActionBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::source::token_rewriter::TokenRewriterResult {
    type Output = crate::blobs::TokenRewriterResultBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}

impl IntoBlob for crate::Token {
    type Output = crate::blobs::TokenBlob;

    fn into_blob(self) -> Self::Output {
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }
}
