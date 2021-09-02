#ifndef LIB_RUBY_PARSER_CPP_BINDINGS_BLOBS
#define LIB_RUBY_PARSER_CPP_BINDINGS_BLOBS

#include "structs.hpp"
#include "declare_blob.hpp"

typedef uint8_t Byte_BLOB;
inline Byte UNPACK_Byte(Byte_BLOB blob)
{
    return blob;
}
inline Byte_BLOB PACK_Byte(Byte byte)
{
    return byte;
}
DECLARE_BLOB_FOR(ByteList);
DECLARE_BLOB_FOR(Ptr);
inline std::unique_ptr<Node> UNPACK_NodePtr(Ptr_BLOB blob)
{
    Ptr ptr = UNPACK_Ptr(blob);
    return std::unique_ptr<Node>((Node *)(ptr.release()));
}
inline Ptr_BLOB PACK_NodePtr(std::unique_ptr<Node> ptr)
{
    return PACK_Ptr(std::unique_ptr<int>((int *)(ptr.release())));
}
inline std::unique_ptr<Token> UNPACK_TokenPtr(Ptr_BLOB blob)
{
    Ptr ptr = UNPACK_Ptr(blob);
    return std::unique_ptr<Token>((Token *)(ptr.release()));
}
inline Ptr_BLOB PACK_TokenPtr(std::unique_ptr<Token> ptr)
{
    return PACK_Ptr(std::unique_ptr<int>((int *)(ptr.release())));
}
DECLARE_BLOB_FOR(MaybePtr);
inline std::unique_ptr<Node> UNPACK_MaybeNodePtr(MaybePtr_BLOB blob)
{
    Ptr ptr = UNPACK_MaybePtr(blob);
    return std::unique_ptr<Node>((Node *)(ptr.release()));
}
inline MaybePtr_BLOB PACK_MaybeNodePtr(std::unique_ptr<Node> ptr)
{
    return PACK_MaybePtr(std::unique_ptr<int>((int *)(ptr.release())));
}
inline std::unique_ptr<Token> UNPACK_MaybeTokenPtr(MaybePtr_BLOB blob)
{
    Ptr ptr = UNPACK_MaybePtr(blob);
    return std::unique_ptr<Token>((Token *)(ptr.release()));
}
inline MaybePtr_BLOB PACK_MaybeTokenPtr(std::unique_ptr<Token> ptr)
{
    return PACK_MaybePtr(std::unique_ptr<int>((int *)(ptr.release())));
}
DECLARE_BLOB_FOR(StringPtr);
DECLARE_BLOB_FOR(MaybeStringPtr);
DECLARE_BLOB_FOR(SharedByteList);
DECLARE_BLOB_FOR(SourceLine);
DECLARE_BLOB_FOR(SourceLineList);
DECLARE_BLOB_FOR(Loc);
DECLARE_BLOB_FOR(MaybeLoc);
DECLARE_BLOB_FOR(Bytes);
DECLARE_BLOB_FOR(Token);
DECLARE_BLOB_FOR(TokenList);
DECLARE_BLOB_FOR(CommentType);
DECLARE_BLOB_FOR(Comment);
DECLARE_BLOB_FOR(CommentList);
DECLARE_BLOB_FOR(MagicCommentKind);
DECLARE_BLOB_FOR(MagicComment);
DECLARE_BLOB_FOR(MagicCommentList);
DECLARE_BLOB_FOR(ErrorLevel);
DECLARE_BLOB_FOR(DiagnosticMessage);
DECLARE_BLOB_FOR(Diagnostic);
DECLARE_BLOB_FOR(DiagnosticList);

#include "blobs_gen.hpp"

DECLARE_BLOB_FOR(Node);
DECLARE_BLOB_FOR(NodeList);

DECLARE_BLOB_FOR(InputError);
DECLARE_BLOB_FOR(DecoderResult);
DECLARE_BLOB_FOR(Decoder);

DECLARE_BLOB_FOR(RewriteAction);
DECLARE_BLOB_FOR(LexStateAction);
DECLARE_BLOB_FOR(TokenRewriterResult);
DECLARE_BLOB_FOR(TokenRewriter);

DECLARE_BLOB_FOR(MaybeDecoder);
DECLARE_BLOB_FOR(MaybeTokenRewriter);
DECLARE_BLOB_FOR(ParserOptions);

DECLARE_BLOB_FOR(DecodedInput);
DECLARE_BLOB_FOR(ParserResult);

#endif // LIB_RUBY_PARSER_CPP_BINDINGS_BLOBS
