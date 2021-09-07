#ifndef LIB_RUBY_PARSER_C_BINDINGS_BLOBS
#define LIB_RUBY_PARSER_C_BINDINGS_BLOBS

#include "structs.h"
#include "declare_blob.h"

typedef uint8_t LIB_RUBY_PARSER_Byte_BLOB;
#define UNPACK_Byte(blob) blob
#define PACK_Byte(byte) byte

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_ByteList);
#define UNPACK_ByteList(blob) ((LIB_RUBY_PARSER_ByteList_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_ByteList(value) ((LIB_RUBY_PARSER_ByteList_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_Ptr);
#define UNPACK_Ptr(blob) ((LIB_RUBY_PARSER_Ptr_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_Ptr(value) ((LIB_RUBY_PARSER_Ptr_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_MaybePtr);
#define UNPACK_MaybePtr(blob) ((LIB_RUBY_PARSER_MaybePtr_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_MaybePtr(value) ((LIB_RUBY_PARSER_MaybePtr_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_StringPtr);
#define UNPACK_StringPtr(blob) ((LIB_RUBY_PARSER_StringPtr_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_StringPtr(value) ((LIB_RUBY_PARSER_StringPtr_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_MaybeStringPtr);
#define UNPACK_MaybeStringPtr(blob) ((LIB_RUBY_PARSER_MaybeStringPtr_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_MaybeStringPtr(value) ((LIB_RUBY_PARSER_MaybeStringPtr_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_SharedByteList);
#define UNPACK_SharedByteList(blob) ((LIB_RUBY_PARSER_SharedByteList_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_SharedByteList(value) ((LIB_RUBY_PARSER_SharedByteList_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_SourceLine);
#define UNPACK_SourceLine(blob) ((LIB_RUBY_PARSER_SourceLine_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_SourceLine(value) ((LIB_RUBY_PARSER_SourceLine_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_SourceLineList);
#define UNPACK_SourceLineList(blob) ((LIB_RUBY_PARSER_SourceLineList_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_SourceLineList(value) ((LIB_RUBY_PARSER_SourceLineList_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_Loc);
#define UNPACK_Loc(blob) ((LIB_RUBY_PARSER_Loc_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_Loc(value) ((LIB_RUBY_PARSER_Loc_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_MaybeLoc);
#define UNPACK_MaybeLoc(blob) ((LIB_RUBY_PARSER_MaybeLoc_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_MaybeLoc(value) ((LIB_RUBY_PARSER_MaybeLoc_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_Bytes);
#define UNPACK_Bytes(blob) ((LIB_RUBY_PARSER_Bytes_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_Bytes(value) ((LIB_RUBY_PARSER_Bytes_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_Token);
#define UNPACK_Token(blob) ((LIB_RUBY_PARSER_Token_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_Token(value) ((LIB_RUBY_PARSER_Token_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_TokenList);
#define UNPACK_TokenList(blob) ((LIB_RUBY_PARSER_TokenList_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_TokenList(value) ((LIB_RUBY_PARSER_TokenList_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_CommentType);
#define UNPACK_CommentType(blob) ((LIB_RUBY_PARSER_CommentType_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_CommentType(value) ((LIB_RUBY_PARSER_CommentType_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_Comment);
#define UNPACK_Comment(blob) ((LIB_RUBY_PARSER_Comment_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_Comment(value) ((LIB_RUBY_PARSER_Comment_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_CommentList);
#define UNPACK_CommentList(blob) ((LIB_RUBY_PARSER_CommentList_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_CommentList(value) ((LIB_RUBY_PARSER_CommentList_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_MagicCommentKind);
#define UNPACK_MagicCommentKind(blob) ((LIB_RUBY_PARSER_MagicCommentKind_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_MagicCommentKind(value) ((LIB_RUBY_PARSER_MagicCommentKind_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_MagicComment);
#define UNPACK_MagicComment(blob) ((LIB_RUBY_PARSER_MagicComment_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_MagicComment(value) ((LIB_RUBY_PARSER_MagicComment_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_MagicCommentList);
#define UNPACK_MagicCommentList(blob) ((LIB_RUBY_PARSER_MagicCommentList_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_MagicCommentList(value) ((LIB_RUBY_PARSER_MagicCommentList_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_ErrorLevel);
#define UNPACK_ErrorLevel(blob) ((LIB_RUBY_PARSER_ErrorLevel_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_ErrorLevel(value) ((LIB_RUBY_PARSER_ErrorLevel_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_DiagnosticMessage);
#define UNPACK_DiagnosticMessage(blob) ((LIB_RUBY_PARSER_DiagnosticMessage_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_DiagnosticMessage(value) ((LIB_RUBY_PARSER_DiagnosticMessage_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_Diagnostic);
#define UNPACK_Diagnostic(blob) ((LIB_RUBY_PARSER_Diagnostic_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_Diagnostic(value) ((LIB_RUBY_PARSER_Diagnostic_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_DiagnosticList);
#define UNPACK_DiagnosticList(blob) ((LIB_RUBY_PARSER_DiagnosticList_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_DiagnosticList(value) ((LIB_RUBY_PARSER_DiagnosticList_BLOB_UNION){.as_value = value}).as_blob

#include "blobs_gen.h"

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_Node);
#define UNPACK_Node(blob) ((LIB_RUBY_PARSER_Node_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_Node(value) ((LIB_RUBY_PARSER_Node_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_NodeList);
#define UNPACK_NodeList(blob) ((LIB_RUBY_PARSER_NodeList_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_NodeList(value) ((LIB_RUBY_PARSER_NodeList_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_InputError);
#define UNPACK_InputError(blob) ((LIB_RUBY_PARSER_InputError_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_InputError(value) ((LIB_RUBY_PARSER_InputError_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_DecoderResult);
#define UNPACK_DecoderResult(blob) ((LIB_RUBY_PARSER_DecoderResult_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_DecoderResult(value) ((LIB_RUBY_PARSER_DecoderResult_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_Decoder);
#define UNPACK_Decoder(blob) ((LIB_RUBY_PARSER_Decoder_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_Decoder(value) ((LIB_RUBY_PARSER_Decoder_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_RewriteAction);
#define UNPACK_RewriteAction(blob) ((LIB_RUBY_PARSER_RewriteAction_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_RewriteAction(value) ((LIB_RUBY_PARSER_RewriteAction_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_LexStateAction);
#define UNPACK_LexStateAction(blob) ((LIB_RUBY_PARSER_LexStateAction_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_LexStateAction(value) ((LIB_RUBY_PARSER_LexStateAction_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_TokenRewriterResult);
#define UNPACK_TokenRewriterResult(blob) ((LIB_RUBY_PARSER_TokenRewriterResult_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_TokenRewriterResult(value) ((LIB_RUBY_PARSER_TokenRewriterResult_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_TokenRewriter);
#define UNPACK_TokenRewriter(blob) ((LIB_RUBY_PARSER_TokenRewriter_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_TokenRewriter(value) ((LIB_RUBY_PARSER_TokenRewriter_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_MaybeDecoder);
#define UNPACK_MaybeDecoder(blob) ((LIB_RUBY_PARSER_MaybeDecoder_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_MaybeDecoder(value) ((LIB_RUBY_PARSER_MaybeDecoder_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_MaybeTokenRewriter);
#define UNPACK_MaybeTokenRewriter(blob) ((LIB_RUBY_PARSER_MaybeTokenRewriter_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_MaybeTokenRewriter(value) ((LIB_RUBY_PARSER_MaybeTokenRewriter_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_ParserOptions);
#define UNPACK_ParserOptions(blob) ((LIB_RUBY_PARSER_ParserOptions_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_ParserOptions(value) ((LIB_RUBY_PARSER_ParserOptions_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_DecodedInput);
#define UNPACK_DecodedInput(blob) ((LIB_RUBY_PARSER_DecodedInput_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_DecodedInput(value) ((LIB_RUBY_PARSER_DecodedInput_BLOB_UNION){.as_value = value}).as_blob

DECLARE_BLOB_FOR(LIB_RUBY_PARSER_ParserResult);
#define UNPACK_ParserResult(blob) ((LIB_RUBY_PARSER_ParserResult_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_ParserResult(value) ((LIB_RUBY_PARSER_ParserResult_BLOB_UNION){.as_value = value}).as_blob

#endif // LIB_RUBY_PARSER_C_BINDINGS_BLOBS
