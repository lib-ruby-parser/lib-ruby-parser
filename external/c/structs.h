#ifndef LIB_RUBY_PARSER_C_BINDINGS_STRUCTS
#define LIB_RUBY_PARSER_C_BINDINGS_STRUCTS

#include <stddef.h>
#include <stdbool.h>
#include "declare_list.h"

// Byte
typedef uint8_t Byte;
DECLARE_LIST_OF(uint8_t, ByteList);
void drop_byte(Byte *);
void drop_byte_list(ByteList *);

// Ptr
typedef void *Ptr;

// MaybePtr
typedef void *MaybePtr;

// StringPtr
typedef struct StringPtr
{
    uint8_t *ptr;
    uint64_t len;
} StringPtr;
void drop_string_ptr(StringPtr *);

// MaybeStringPtr
typedef struct MaybeStringPtr
{
    uint8_t *ptr;
    uint64_t len;
} MaybeStringPtr;
void drop_maybe_string_ptr(MaybeStringPtr *maybe_string_ptr);

// SharedByteList
typedef struct SharedByteList
{
    const uint8_t *ptr;
    uint64_t len;
} SharedByteList;

// SourceLine
typedef struct SourceLine
{
    uint64_t start;
    uint64_t end;
    bool ends_with_eof;
} SourceLine;
DECLARE_LIST_OF(SourceLine, SourceLineList);

// Loc
typedef struct Loc
{
    uint64_t begin;
    uint64_t end;
} Loc;
void drop_loc(Loc *loc);

// MaybeLoc
typedef struct MaybeLoc
{
    enum
    {
        MAYBE_LOC_SOME,
        MAYBE_LOC_NONE,
    } tag;

    union
    {
        struct
        {
            uint8_t dummy;
        } nothing;
        Loc loc;
    } as;
} MaybeLoc;
void drop_maybe_loc(MaybeLoc *maybe_loc);

// Bytes
typedef struct Bytes
{
    ByteList raw;
} Bytes;
void drop_bytes(Bytes *bytes);

// Token
typedef struct Token
{
    uint32_t token_type;
    Bytes token_value;
    Loc loc;
    uint32_t lex_state_before;
    uint32_t lex_state_after;
} Token;
DECLARE_LIST_OF(Token, TokenList);
void drop_token(Token *);

// CommentType
typedef enum CommentType
{
    DOCUMENT,
    INLINE,
    UNKNOWN,
} CommentType;

// Comment
typedef struct Comment
{
    Loc location;
    CommentType kind;
} Comment;
DECLARE_LIST_OF(Comment, CommentList);

// MagicCommentKind
typedef enum MagicCommentKind
{
    MAGIC_COMMENT_KIND_ENCODING,
    MAGIC_COMMENT_KIND_FROZEN_STRING_LITERAL,
    MAGIC_COMMENT_KIND_WARN_INDENT,
    MAGIC_COMMENT_KIND_SHAREABLE_CONSTANT_VALUE,
} MagicCommentKind;

// MagicComment
typedef struct MagicComment
{
    MagicCommentKind kind;
    Loc key_l;
    Loc value_l;
} MagicComment;
DECLARE_LIST_OF(MagicComment, MagicCommentList);

// ErrorLevel
typedef enum ErrorLevel
{
    WARNING,
    ERROR
} ErrorLevel;

// DiagnosticMessage
#include "messages.h"

// Node
#include "nodes.h"

// InputError
typedef struct InputError
{
    enum
    {
        UNSUPPORTED_ENCODING,
        DECODING_ERROR
    } tag;

    union
    {
        StringPtr unsupported_encoding;
        StringPtr decoding_error;
    } as;
} InputError;
void drop_input_error(InputError *);

// DecoderResult
typedef struct DecoderResult
{
    enum
    {
        DECODE_OK,
        DECODE_ERR
    } tag;

    union
    {
        ByteList ok;
        InputError err;
    } as;
} DecoderResult;
void drop_decoder_result(DecoderResult *);

// Decoder
typedef DecoderResult (*dummy_decoder_t)(void);
typedef struct Decoder
{
    // Here for tests we use a dummy fn that blindly returns what's configured when called
    dummy_decoder_t f;
} Decoder;

#endif // LIB_RUBY_PARSER_C_BINDINGS_STRUCTS
