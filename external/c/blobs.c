#include "blobs.h"
#include "impl_blob.h"

// Byte
Byte UNPACK_Byte(Byte_BLOB blob)
{
    return blob;
}
Byte_BLOB PACK_Byte(Byte byte)
{
    return byte;
}
IMPL_BLOB(ByteList);

// Ptr
IMPL_BLOB(Ptr);

// MaybePtr
IMPL_BLOB(MaybePtr);

// StringPtr
IMPL_BLOB(StringPtr);

// MaybeStringPtr
IMPL_BLOB(MaybeStringPtr);

// SharedByteList
IMPL_BLOB(SharedByteList);

// SourceLine
IMPL_BLOB(SourceLine);
IMPL_BLOB(SourceLineList);

// Loc
IMPL_BLOB(Loc);

// MaybeLoc
IMPL_BLOB(MaybeLoc);

// Bytes
IMPL_BLOB(Bytes);

// Token
IMPL_BLOB(Token);
IMPL_BLOB(TokenList);

// CommentType
IMPL_BLOB(CommentType);

// Comment
IMPL_BLOB(Comment);
IMPL_BLOB(CommentList);

// MagicCommentKind
IMPL_BLOB(MagicCommentKind);

// MagicComment
IMPL_BLOB(MagicComment);
IMPL_BLOB(MagicCommentList);

// ErrorLevel
IMPL_BLOB(ErrorLevel);

// Diagnostic
IMPL_BLOB(Diagnostic);
IMPL_BLOB(DiagnosticList);

// DiagnosticMessage
IMPL_BLOB(DiagnosticMessage);

// Node
IMPL_BLOB(Node);
IMPL_BLOB(NodeList);

// InputError
IMPL_BLOB(InputError);

// DecoderResult
IMPL_BLOB(DecoderResult);

// Decoder
IMPL_BLOB(Decoder);

// TokenRewriter
IMPL_BLOB(RewriteAction);
IMPL_BLOB(LexStateAction);
IMPL_BLOB(TokenRewriterResult);
IMPL_BLOB(TokenRewriter);

// ParserOptions
IMPL_BLOB(MaybeDecoder);
IMPL_BLOB(MaybeTokenRewriter);
IMPL_BLOB(ParserOptions);
