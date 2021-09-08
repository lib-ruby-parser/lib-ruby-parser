#ifndef LIB_RUBY_PARSER_C_BINDINGS_STRUCTS
#define LIB_RUBY_PARSER_C_BINDINGS_STRUCTS

#include <stddef.h>
#include <stdbool.h>
#include <stdint.h>

// Byte
typedef uint8_t LIB_RUBY_PARSER_Byte;
typedef struct LIB_RUBY_PARSER_ByteList
{
    LIB_RUBY_PARSER_Byte *ptr;
    uint64_t len;
    uint64_t capacity;
} LIB_RUBY_PARSER_ByteList;
void LIB_RUBY_PARSER_drop_byte(LIB_RUBY_PARSER_Byte *);
void LIB_RUBY_PARSER_drop_byte_list(LIB_RUBY_PARSER_ByteList *);

// Ptr
typedef void *LIB_RUBY_PARSER_Ptr;

// MaybePtr
typedef void *LIB_RUBY_PARSER_MaybePtr;

// StringPtr
typedef struct LIB_RUBY_PARSER_StringPtr
{
    uint8_t *ptr;
    uint64_t len;
} LIB_RUBY_PARSER_StringPtr;
void LIB_RUBY_PARSER_drop_string_ptr(LIB_RUBY_PARSER_StringPtr *);

// MaybeStringPtr
typedef struct LIB_RUBY_PARSER_MaybeStringPtr
{
    uint8_t *ptr;
    uint64_t len;
} LIB_RUBY_PARSER_MaybeStringPtr;
void LIB_RUBY_PARSER_drop_maybe_string_ptr(LIB_RUBY_PARSER_MaybeStringPtr *maybe_string_ptr);

// SharedByteList
typedef struct LIB_RUBY_PARSER_SharedByteList
{
    const uint8_t *ptr;
    uint64_t len;
} LIB_RUBY_PARSER_SharedByteList;

// SourceLine
typedef struct LIB_RUBY_PARSER_SourceLine
{
    uint64_t start;
    uint64_t end;
    bool ends_with_eof;
} LIB_RUBY_PARSER_SourceLine;
typedef struct LIB_RUBY_PARSER_SourceLineList
{
    LIB_RUBY_PARSER_SourceLine *ptr;
    uint64_t len;
    uint64_t capacity;
} LIB_RUBY_PARSER_SourceLineList;
void LIB_RUBY_PARSER_drop_source_line_list(LIB_RUBY_PARSER_SourceLineList *source_line_list);

// Loc
typedef struct LIB_RUBY_PARSER_Loc
{
    uint64_t begin;
    uint64_t end;
} LIB_RUBY_PARSER_Loc;
void LIB_RUBY_PARSER_drop_loc(LIB_RUBY_PARSER_Loc *loc);

// MaybeLoc
typedef struct LIB_RUBY_PARSER_MaybeLoc
{
    enum
    {
        LIB_RUBY_PARSER_MAYBE_LOC_SOME,
        LIB_RUBY_PARSER_MAYBE_LOC_NONE,
    } tag;

    union
    {
        struct
        {
            uint8_t dummy;
        } nothing;
        LIB_RUBY_PARSER_Loc loc;
    } as;
} LIB_RUBY_PARSER_MaybeLoc;
void LIB_RUBY_PARSER_drop_maybe_loc(LIB_RUBY_PARSER_MaybeLoc *maybe_loc);

// Bytes
typedef struct LIB_RUBY_PARSER_Bytes
{
    LIB_RUBY_PARSER_ByteList raw;
} LIB_RUBY_PARSER_Bytes;
void LIB_RUBY_PARSER_drop_bytes(LIB_RUBY_PARSER_Bytes *bytes);

// Token
typedef struct LIB_RUBY_PARSER_Token
{
    uint32_t token_type;
    LIB_RUBY_PARSER_Bytes token_value;
    LIB_RUBY_PARSER_Loc loc;
    uint32_t lex_state_before;
    uint32_t lex_state_after;
} LIB_RUBY_PARSER_Token;
typedef struct LIB_RUBY_PARSER_TokenList
{
    LIB_RUBY_PARSER_Token *ptr;
    uint64_t len;
    uint64_t capacity;
} LIB_RUBY_PARSER_TokenList;
void LIB_RUBY_PARSER_drop_token(LIB_RUBY_PARSER_Token *);
void LIB_RUBY_PARSER_drop_token_list(LIB_RUBY_PARSER_TokenList *);

// CommentType
typedef enum LIB_RUBY_PARSER_CommentType
{
    LIB_RUBY_PARSER_COMMENT_TYPE_DOCUMENT,
    LIB_RUBY_PARSER_COMMENT_TYPE_INLINE,
    LIB_RUBY_PARSER_COMMENT_TYPE_UNKNOWN,
} LIB_RUBY_PARSER_CommentType;

// Comment
typedef struct LIB_RUBY_PARSER_Comment
{
    LIB_RUBY_PARSER_Loc location;
    LIB_RUBY_PARSER_CommentType kind;
} LIB_RUBY_PARSER_Comment;
typedef struct LIB_RUBY_PARSER_CommentList
{
    LIB_RUBY_PARSER_Comment *ptr;
    uint64_t len;
    uint64_t capacity;
} LIB_RUBY_PARSER_CommentList;
void LIB_RUBY_PARSER_drop_comment_list(LIB_RUBY_PARSER_CommentList *);

// MagicCommentKind
typedef enum LIB_RUBY_PARSER_MagicCommentKind
{
    LIB_RUBY_PARSER_MAGIC_COMMENT_KIND_ENCODING,
    LIB_RUBY_PARSER_MAGIC_COMMENT_KIND_FROZEN_STRING_LITERAL,
    LIB_RUBY_PARSER_MAGIC_COMMENT_KIND_WARN_INDENT,
    LIB_RUBY_PARSER_MAGIC_COMMENT_KIND_SHAREABLE_CONSTANT_VALUE,
} LIB_RUBY_PARSER_MagicCommentKind;

// MagicComment
typedef struct LIB_RUBY_PARSER_MagicComment
{
    LIB_RUBY_PARSER_MagicCommentKind kind;
    LIB_RUBY_PARSER_Loc key_l;
    LIB_RUBY_PARSER_Loc value_l;
} LIB_RUBY_PARSER_MagicComment;
typedef struct LIB_RUBY_PARSER_MagicCommentList
{
    LIB_RUBY_PARSER_MagicComment *ptr;
    uint64_t len;
    uint64_t capacity;
} LIB_RUBY_PARSER_MagicCommentList;
void LIB_RUBY_PARSER_drop_magic_comment_list(LIB_RUBY_PARSER_MagicCommentList *);

// ErrorLevel
typedef enum LIB_RUBY_PARSER_ErrorLevel
{
    LIB_RUBY_PARSER_ERROR_LEVEL_WARNING,
    LIB_RUBY_PARSER_ERROR_LEVEL_ERROR
} LIB_RUBY_PARSER_ErrorLevel;

// DiagnosticMessage
#include "messages.h"
void LIB_RUBY_PARSER_drop_diagnostic_list(LIB_RUBY_PARSER_DiagnosticList *);

// Node
#include "nodes.h"

// InputError
typedef struct LIB_RUBY_PARSER_InputError
{
    enum
    {
        LIB_RUBY_PARSER_INPUT_ERROR_UNSUPPORTED_ENCODING,
        LIB_RUBY_PARSER_INPUT_ERROR_DECODING_ERROR
    } tag;

    union
    {
        LIB_RUBY_PARSER_StringPtr unsupported_encoding;
        LIB_RUBY_PARSER_StringPtr decoding_error;
    } as;
} LIB_RUBY_PARSER_InputError;
void LIB_RUBY_PARSER_drop_input_error(LIB_RUBY_PARSER_InputError *);

// DecoderResult
typedef struct LIB_RUBY_PARSER_DecoderResult
{
    enum
    {
        LIB_RUBY_PARSER_DECODER_RESULT_OK,
        LIB_RUBY_PARSER_DECODER_RESULT_ERR
    } tag;

    union
    {
        LIB_RUBY_PARSER_ByteList ok;
        LIB_RUBY_PARSER_InputError err;
    } as;
} LIB_RUBY_PARSER_DecoderResult;
void LIB_RUBY_PARSER_drop_decoder_result(LIB_RUBY_PARSER_DecoderResult *);

// Decoder
typedef LIB_RUBY_PARSER_DecoderResult (*LIB_RUBY_PARSER_dummy_decoder_t)(void);
typedef struct LIB_RUBY_PARSER_Decoder
{
    // Here for tests we use a dummy fn that (when called) blindly returns what's configured
    LIB_RUBY_PARSER_dummy_decoder_t f;
} LIB_RUBY_PARSER_Decoder;

// TokenRewriter
typedef enum LIB_RUBY_PARSER_RewriteAction
{
    LIB_RUBY_PARSER_REWRITE_ACTION_DROP,
    LIB_RUBY_PARSER_REWRITE_ACTION_KEEP
} LIB_RUBY_PARSER_RewriteAction;
typedef struct LIB_RUBY_PARSER_LexStateAction
{
    enum
    {
        LIB_RUBY_PARSER_LEX_STATE_SET,
        LIB_RUBY_PARSER_LEX_STATE_KEEP
    } tag;
    union
    {
        int32_t set;
    } as;
} LIB_RUBY_PARSER_LexStateAction;
typedef struct LIB_RUBY_PARSER_TokenRewriterResult
{
    LIB_RUBY_PARSER_Token *rewritten_token;
    LIB_RUBY_PARSER_RewriteAction token_action;
    LIB_RUBY_PARSER_LexStateAction lex_state_action;
} LIB_RUBY_PARSER_TokenRewriterResult;
void LIB_RUBY_PARSER_drop_token_rewriter_result(LIB_RUBY_PARSER_TokenRewriterResult *);
typedef LIB_RUBY_PARSER_Token *(*LIB_RUBY_PARSER_build_new_token_t)(void);
typedef LIB_RUBY_PARSER_TokenRewriterResult (*LIB_RUBY_PARSER_rewrite_token_t)(LIB_RUBY_PARSER_Token *, LIB_RUBY_PARSER_build_new_token_t);
typedef struct LIB_RUBY_PARSER_TokenRewriter
{
    // Here for tests we use a dummy fn that (when called) blindly returns what's configured
    LIB_RUBY_PARSER_rewrite_token_t rewrite_f;
    LIB_RUBY_PARSER_build_new_token_t build_new_token_f;
} LIB_RUBY_PARSER_TokenRewriter;
// Test APIS
LIB_RUBY_PARSER_TokenRewriter __keep_token_rewriter(LIB_RUBY_PARSER_build_new_token_t build_new_token_f);
LIB_RUBY_PARSER_TokenRewriter __drop_token_rewriter(LIB_RUBY_PARSER_build_new_token_t build_new_token_f);
LIB_RUBY_PARSER_TokenRewriter __rewriter_token_rewriter(LIB_RUBY_PARSER_build_new_token_t build_new_token_f);

// ParserOptions
typedef struct LIB_RUBY_PARSER_MaybeDecoder
{
    enum
    {
        LIB_RUBY_PARSER_MAYBE_DECODER_SOME,
        LIB_RUBY_PARSER_MAYBE_DECODER_NONE
    } tag;

    union
    {
        struct
        {
            uint8_t dummy;
        } nothing;
        LIB_RUBY_PARSER_Decoder decoder;
    } as;
} LIB_RUBY_PARSER_MaybeDecoder;
typedef struct LIB_RUBY_PARSER_MaybeTokenRewriter
{
    enum
    {
        LIB_RUBY_PARSER_MAYBE_TOKEN_REWRITER_SOME,
        LIB_RUBY_PARSER_MAYBE_TOKEN_REWRITER_NONE
    } tag;

    union
    {
        struct
        {
            uint8_t dummy;
        } nothing;
        LIB_RUBY_PARSER_TokenRewriter token_rewriter;
    } as;
} LIB_RUBY_PARSER_MaybeTokenRewriter;
typedef struct LIB_RUBY_PARSER_ParserOptions
{
    LIB_RUBY_PARSER_StringPtr buffer_name;
    uint8_t debug;
    LIB_RUBY_PARSER_MaybeDecoder decoder;
    LIB_RUBY_PARSER_MaybeTokenRewriter token_rewriter;
    bool record_tokens;
} LIB_RUBY_PARSER_ParserOptions;
void LIB_RUBY_PARSER_drop_parser_options(LIB_RUBY_PARSER_ParserOptions *);

// DecodedInput
typedef struct LIB_RUBY_PARSER_DecodedInput
{
    LIB_RUBY_PARSER_StringPtr name;
    LIB_RUBY_PARSER_SourceLineList lines;
    LIB_RUBY_PARSER_ByteList bytes;
} LIB_RUBY_PARSER_DecodedInput;
void LIB_RUBY_PARSER_drop_decoded_input(LIB_RUBY_PARSER_DecodedInput *);

// ParserResult
typedef struct LIB_RUBY_PARSER_ParserResult
{
    LIB_RUBY_PARSER_Node *ast;
    LIB_RUBY_PARSER_TokenList tokens;
    LIB_RUBY_PARSER_DiagnosticList diagnostics;
    LIB_RUBY_PARSER_CommentList comments;
    LIB_RUBY_PARSER_MagicCommentList magic_comments;
    LIB_RUBY_PARSER_DecodedInput input;
} LIB_RUBY_PARSER_ParserResult;
void LIB_RUBY_PARSER_drop_parser_result(LIB_RUBY_PARSER_ParserResult *);

#endif // LIB_RUBY_PARSER_C_BINDINGS_STRUCTS
