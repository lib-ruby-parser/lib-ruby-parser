#ifndef LIB_RUBY_PARSER_CPP_BINDINGS_BLOBS
#define LIB_RUBY_PARSER_CPP_BINDINGS_BLOBS

#include "structs.hpp"
#include "declare_blob.hpp"

typedef uint8_t Byte_BLOB;
Byte UNPACK_Byte(Byte_BLOB blob);
Byte_BLOB PACK_Byte(Byte byte);
DECLARE_BLOB_FOR(ByteList);
DECLARE_BLOB_FOR(Ptr);
DECLARE_BLOB_FOR(MaybePtr);
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

#endif // LIB_RUBY_PARSER_CPP_BINDINGS_BLOBS
