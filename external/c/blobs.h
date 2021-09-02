#ifndef LIB_RUBY_PARSER_C_BINDINGS_BLOBS
#define LIB_RUBY_PARSER_C_BINDINGS_BLOBS

#include "structs.h"
#include "declare_blob.h"

typedef uint8_t Byte_BLOB;
#define UNPACK_Byte(blob) blob
#define PACK_Byte(byte) byte

DECLARE_BLOB_FOR(ByteList);
#define UNPACK_ByteList(blob) ((ByteList_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_ByteList(value) ((ByteList_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(Ptr);
#define UNPACK_Ptr(blob) ((Ptr_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_Ptr(value) ((Ptr_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(MaybePtr);
#define UNPACK_MaybePtr(blob) ((MaybePtr_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_MaybePtr(value) ((MaybePtr_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(StringPtr);
#define UNPACK_StringPtr(blob) ((StringPtr_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_StringPtr(value) ((StringPtr_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(MaybeStringPtr);
#define UNPACK_MaybeStringPtr(blob) ((MaybeStringPtr_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_MaybeStringPtr(value) ((MaybeStringPtr_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(SharedByteList);
#define UNPACK_SharedByteList(blob) ((SharedByteList_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_SharedByteList(value) ((SharedByteList_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(SourceLine);
#define UNPACK_SourceLine(blob) ((SourceLine_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_SourceLine(value) ((SourceLine_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(SourceLineList);
#define UNPACK_SourceLineList(blob) ((SourceLineList_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_SourceLineList(value) ((SourceLineList_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(Loc);
#define UNPACK_Loc(blob) ((Loc_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_Loc(value) ((Loc_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(MaybeLoc);
#define UNPACK_MaybeLoc(blob) ((MaybeLoc_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_MaybeLoc(value) ((MaybeLoc_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(Bytes);
#define UNPACK_Bytes(blob) ((Bytes_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_Bytes(value) ((Bytes_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(Token);
#define UNPACK_Token(blob) ((Token_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_Token(value) ((Token_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(TokenList);
#define UNPACK_TokenList(blob) ((TokenList_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_TokenList(value) ((TokenList_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(CommentType);
#define UNPACK_CommentType(blob) ((CommentType_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_CommentType(value) ((CommentType_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(Comment);
#define UNPACK_Comment(blob) ((Comment_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_Comment(value) ((Comment_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(CommentList);
#define UNPACK_CommentList(blob) ((CommentList_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_CommentList(value) ((CommentList_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(MagicCommentKind);
#define UNPACK_MagicCommentKind(blob) ((MagicCommentKind_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_MagicCommentKind(value) ((MagicCommentKind_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(MagicComment);
#define UNPACK_MagicComment(blob) ((MagicComment_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_MagicComment(value) ((MagicComment_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(MagicCommentList);
#define UNPACK_MagicCommentList(blob) ((MagicCommentList_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_MagicCommentList(value) ((MagicCommentList_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(ErrorLevel);
#define UNPACK_ErrorLevel(blob) ((ErrorLevel_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_ErrorLevel(value) ((ErrorLevel_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(DiagnosticMessage);
#define UNPACK_DiagnosticMessage(blob) ((DiagnosticMessage_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_DiagnosticMessage(value) ((DiagnosticMessage_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(Diagnostic);
#define UNPACK_Diagnostic(blob) ((Diagnostic_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_Diagnostic(value) ((Diagnostic_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(DiagnosticList);
#define UNPACK_DiagnosticList(blob) ((DiagnosticList_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_DiagnosticList(value) ((DiagnosticList_BLOB_UNION){.as_value = value}).as_blob

#include "blobs_gen.h"

DECLARE_BLOB_FOR(Node);
#define UNPACK_Node(blob) ((Node_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_Node(value) ((Node_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(NodeList);
#define UNPACK_NodeList(blob) ((NodeList_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_NodeList(value) ((NodeList_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(InputError);
#define UNPACK_InputError(blob) ((InputError_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_InputError(value) ((InputError_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(DecoderResult);
#define UNPACK_DecoderResult(blob) ((DecoderResult_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_DecoderResult(value) ((DecoderResult_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(Decoder);
#define UNPACK_Decoder(blob) ((Decoder_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_Decoder(value) ((Decoder_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(RewriteAction);
#define UNPACK_RewriteAction(blob) ((RewriteAction_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_RewriteAction(value) ((RewriteAction_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LexStateAction);
#define UNPACK_LexStateAction(blob) ((LexStateAction_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_LexStateAction(value) ((LexStateAction_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(TokenRewriterResult);
#define UNPACK_TokenRewriterResult(blob) ((TokenRewriterResult_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_TokenRewriterResult(value) ((TokenRewriterResult_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(TokenRewriter);
#define UNPACK_TokenRewriter(blob) ((TokenRewriter_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_TokenRewriter(value) ((TokenRewriter_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(MaybeDecoder);
#define UNPACK_MaybeDecoder(blob) ((MaybeDecoder_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_MaybeDecoder(value) ((MaybeDecoder_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(MaybeTokenRewriter);
#define UNPACK_MaybeTokenRewriter(blob) ((MaybeTokenRewriter_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_MaybeTokenRewriter(value) ((MaybeTokenRewriter_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(ParserOptions);
#define UNPACK_ParserOptions(blob) ((ParserOptions_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_ParserOptions(value) ((ParserOptions_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(DecodedInput);
#define UNPACK_DecodedInput(blob) ((DecodedInput_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_DecodedInput(value) ((DecodedInput_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(ParserResult);
#define UNPACK_ParserResult(blob) ((ParserResult_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_ParserResult(value) ((ParserResult_BLOB_UNION){.as_value = value}).as_blob

#endif // LIB_RUBY_PARSER_C_BINDINGS_BLOBS
