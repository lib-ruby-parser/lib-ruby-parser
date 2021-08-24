#include <stdlib.h>
#include <string.h>
#include "bindings.h"

/*
    Ptr
*/

Ptr_BLOB lib_ruby_parser__internal__containers__ptr__new(void *ptr)
{
    return PACK_Ptr(ptr);
}
Ptr_BLOB lib_ruby_parser__internal__containers__ptr__new_null()
{
    return PACK_Ptr(NULL);
}
void *lib_ruby_parser__internal__containers__ptr__get_raw(Ptr_BLOB *blob)
{
    Ptr ptr = *((Ptr *)blob);
    return ptr;
}
void lib_ruby_parser__internal__containers__ptr__of_node__free(Ptr_BLOB *blob)
{
    Ptr ptr = *((Ptr *)blob);
    drop_node((Node *)ptr);
    free(ptr);
}
void lib_ruby_parser__internal__containers__ptr__of_token__free(Ptr_BLOB *blob)
{
    Ptr ptr = *((Ptr *)blob);
    drop_token((Token *)ptr);
    free(ptr);
}

/*
    MaybePtr
*/
MaybePtr_BLOB lib_ruby_parser__internal__containers__maybe_ptr__new(void *ptr)
{
    return PACK_MaybePtr(ptr);
}
MaybePtr_BLOB lib_ruby_parser__internal__containers__maybe_ptr__new_null()
{
    return PACK_MaybePtr(NULL);
}
void *lib_ruby_parser__internal__containers__maybe_ptr__get_raw(MaybePtr_BLOB *blob)
{
    MaybePtr maybe_ptr = *((MaybePtr *)blob);
    return maybe_ptr;
}
void lib_ruby_parser__internal__containers__maybe_ptr__of_node__free(MaybePtr_BLOB *blob)
{
    MaybePtr maybe_ptr = *((MaybePtr *)blob);
    if (maybe_ptr != NULL)
    {
        drop_node((Node *)maybe_ptr);
        free(maybe_ptr);
    }
}
void lib_ruby_parser__internal__containers__maybe_ptr__of_token__free(MaybePtr_BLOB *blob)
{
    MaybePtr maybe_ptr = *((MaybePtr *)blob);
    if (maybe_ptr != NULL)
    {
        drop_token((Token *)maybe_ptr);
        free(maybe_ptr);
    }
}

/*
    StringPtr
*/
StringPtr_BLOB lib_ruby_parser__internal__containers__string_ptr__new(const uint8_t *ptr, uint64_t len)
{
    uint8_t *new_ptr = malloc(len);
    memcpy(new_ptr, ptr, len);
    StringPtr string_ptr = {.ptr = new_ptr, .len = len};
    return PACK_StringPtr(string_ptr);
}
void lib_ruby_parser__internal__containers__string_ptr__drop(StringPtr_BLOB *blob)
{
    StringPtr *string_ptr = (StringPtr *)blob;
    drop_string_ptr(string_ptr);
}
uint8_t *lib_ruby_parser__internal__containers__string_ptr__get_raw(StringPtr_BLOB *blob)
{
    StringPtr *string_ptr = (StringPtr *)blob;
    if (string_ptr->len == 0)
        return NULL;
    return string_ptr->ptr;
}
uint64_t lib_ruby_parser__internal__containers__string_ptr__get_len(const StringPtr_BLOB *blob)
{
    StringPtr *string_ptr = (StringPtr *)blob;
    return string_ptr->len;
}

/*
    MaybeStringPtr
*/
MaybeStringPtr_BLOB lib_ruby_parser__internal__containers__maybe_string_ptr__new_some(const uint8_t *ptr, uint64_t len)
{
    MaybeStringPtr maybe_string_ptr = {.ptr = malloc(len), .len = len};
    memcpy(maybe_string_ptr.ptr, ptr, len);
    return PACK_MaybeStringPtr(maybe_string_ptr);
}
MaybeStringPtr_BLOB lib_ruby_parser__internal__containers__maybe_string_ptr__new_none()
{
    MaybeStringPtr maybe_string_ptr = {.ptr = NULL, .len = 0};
    return PACK_MaybeStringPtr(maybe_string_ptr);
}
bool lib_ruby_parser__internal__containers__maybe_string_ptr__is_some(const MaybeStringPtr_BLOB *blob)
{
    const MaybeStringPtr *maybe_string_ptr = (const MaybeStringPtr *)blob;
    return maybe_string_ptr->ptr != NULL;
}
bool lib_ruby_parser__internal__containers__maybe_string_ptr__is_none(const MaybeStringPtr_BLOB *blob)
{
    const MaybeStringPtr *maybe_string_ptr = (const MaybeStringPtr *)blob;
    return maybe_string_ptr->ptr == NULL;
}
void lib_ruby_parser__internal__containers__maybe_string_ptr__drop(MaybeStringPtr_BLOB *blob)
{
    MaybeStringPtr *maybe_string_ptr = (MaybeStringPtr *)blob;
    drop_maybe_string_ptr(maybe_string_ptr);
}
uint8_t *lib_ruby_parser__internal__containers__maybe_string_ptr__get_raw(MaybeStringPtr_BLOB *blob)
{
    MaybeStringPtr *maybe_string_ptr = (MaybeStringPtr *)blob;
    return maybe_string_ptr->ptr;
}
uint8_t *lib_ruby_parser__internal__containers__maybe_string_ptr__into_raw(MaybeStringPtr_BLOB *blob)
{
    MaybeStringPtr *maybe_string_ptr = (MaybeStringPtr *)blob;
    if (maybe_string_ptr->ptr == NULL)
        return NULL;
    uint8_t *result = (uint8_t *)(maybe_string_ptr->ptr);
    maybe_string_ptr->ptr = NULL;
    maybe_string_ptr->len = 0;
    return result;
}
uint64_t lib_ruby_parser__internal__containers__maybe_string_ptr__len(const MaybeStringPtr_BLOB *blob)
{
    const MaybeStringPtr *maybe_string_ptr = (const MaybeStringPtr *)blob;
    return maybe_string_ptr->len;
}

/*
    Lists
*/
#define LIST_IMPL(ITEM, ITEM_BLOB, LIST, LIST_BLOB, NS, drop)                                              \
    LIST_BLOB lib_ruby_parser__internal__containers__list__##NS##__new()                                   \
    {                                                                                                      \
        LIST list = {.ptr = NULL, .len = 0, .capacity = 0};                                                \
        return PACK_##LIST(list);                                                                          \
    }                                                                                                      \
    void lib_ruby_parser__internal__containers__list__##NS##__drop(LIST_BLOB *blob)                        \
    {                                                                                                      \
        LIST *list = (LIST *)blob;                                                                         \
        for (uint64_t i = 0; i < list->len; i++)                                                           \
        {                                                                                                  \
            drop(&list->ptr[i]);                                                                           \
        }                                                                                                  \
        free(list->ptr);                                                                                   \
    }                                                                                                      \
    LIST_BLOB lib_ruby_parser__internal__containers__list__##NS##__with_capacity(uint64_t capacity)        \
    {                                                                                                      \
        LIST list = {.ptr = malloc(sizeof(ITEM) * capacity), .len = 0, .capacity = capacity};              \
        return PACK_##LIST(list);                                                                          \
    }                                                                                                      \
    LIST_BLOB lib_ruby_parser__internal__containers__list__##NS##__from_raw(ITEM_BLOB *ptr, uint64_t len)  \
    {                                                                                                      \
        if (len > 0)                                                                                       \
        {                                                                                                  \
            LIST list = {.ptr = (ITEM *)ptr, .len = len, .capacity = len};                                 \
            return PACK_##LIST(list);                                                                      \
        }                                                                                                  \
        else                                                                                               \
        {                                                                                                  \
            return lib_ruby_parser__internal__containers__list__##NS##__new();                             \
        }                                                                                                  \
    }                                                                                                      \
    void lib_ruby_parser__internal__containers__list__##NS##__push(LIST_BLOB *blob, ITEM_BLOB item_blob)   \
    {                                                                                                      \
        LIST *list = (LIST *)blob;                                                                         \
        ITEM item = UNPACK_##ITEM(item_blob);                                                              \
        if (list->len + 1 > list->capacity)                                                                \
        {                                                                                                  \
            if (list->capacity == 0)                                                                       \
            {                                                                                              \
                list->capacity += 1;                                                                       \
            }                                                                                              \
            else                                                                                           \
            {                                                                                              \
                list->capacity *= 2;                                                                       \
            }                                                                                              \
            ITEM *old_ptr = list->ptr;                                                                     \
            ITEM *new_ptr = malloc(sizeof(ITEM) * list->capacity);                                         \
            memcpy(new_ptr, old_ptr, sizeof(ITEM) * list->len);                                            \
            list->ptr = new_ptr;                                                                           \
            free(old_ptr);                                                                                 \
        }                                                                                                  \
        list->ptr[list->len] = item;                                                                       \
        list->len++;                                                                                       \
    }                                                                                                      \
    ITEM_BLOB lib_ruby_parser__internal__containers__list__##NS##__remove(LIST_BLOB *blob, uint64_t index) \
    {                                                                                                      \
        LIST *list = (LIST *)blob;                                                                         \
        ITEM item = list->ptr[index];                                                                      \
        memmove(list->ptr + index, list->ptr + index + 1, sizeof(ITEM) * (list->len - index - 1));         \
        list->len--;                                                                                       \
        return PACK_##ITEM(item);                                                                          \
    }                                                                                                      \
    void lib_ruby_parser__internal__containers__list__##NS##__shrink_to_fit(LIST_BLOB *blob)               \
    {                                                                                                      \
        LIST *list = (LIST *)blob;                                                                         \
                                                                                                           \
        uint64_t new_len = list->len;                                                                      \
        uint64_t new_capacity = list->len;                                                                 \
                                                                                                           \
        ITEM *new_ptr = malloc(sizeof(ITEM) * new_capacity);                                               \
        memcpy(new_ptr, list->ptr, sizeof(ITEM) * new_len);                                                \
                                                                                                           \
        ITEM *old_ptr = list->ptr;                                                                         \
        list->ptr = new_ptr;                                                                               \
        list->len = new_len;                                                                               \
        list->capacity = new_capacity;                                                                     \
        free(old_ptr);                                                                                     \
    }                                                                                                      \
    ITEM_BLOB *lib_ruby_parser__internal__containers__list__##NS##__as_ptr(LIST_BLOB *blob)                \
    {                                                                                                      \
        LIST *list = (LIST *)blob;                                                                         \
        return (ITEM_BLOB *)(list->ptr);                                                                   \
    }                                                                                                      \
    uint64_t lib_ruby_parser__internal__containers__list__##NS##__len(const LIST_BLOB *blob)               \
    {                                                                                                      \
        const LIST *list = (const LIST *)blob;                                                             \
        return list->len;                                                                                  \
    }                                                                                                      \
    uint64_t lib_ruby_parser__internal__containers__list__##NS##__capacity(const LIST_BLOB *blob)          \
    {                                                                                                      \
        const LIST *list = (const LIST *)blob;                                                             \
        return list->capacity;                                                                             \
    }

void drop_nothing(void *byte)
{
    (void)byte;
}
LIST_IMPL(Byte, Byte_BLOB, ByteList, ByteList_BLOB, of_bytes, drop_nothing)
LIST_IMPL(Token, Token_BLOB, TokenList, TokenList_BLOB, of_tokens, drop_token)
LIST_IMPL(Node, Node_BLOB, NodeList, NodeList_BLOB, of_nodes, drop_node)
LIST_IMPL(Diagnostic, Diagnostic_BLOB, DiagnosticList, DiagnosticList_BLOB, of_diagnostics, drop_diagnostic)
LIST_IMPL(Comment, Comment_BLOB, CommentList, CommentList_BLOB, of_comments, drop_nothing)
LIST_IMPL(MagicComment, MagicComment_BLOB, MagicCommentList, MagicCommentList_BLOB, of_magic_comments, drop_nothing)
LIST_IMPL(SourceLine, SourceLine_BLOB, SourceLineList, SourceLineList_BLOB, of_source_lines, drop_nothing)

/*
    SourceLine
*/
SourceLine_BLOB lib_ruby_parser__internal__containers__source_line__new(uint64_t start, uint64_t end, bool ends_with_eof)
{
    SourceLine source_line = {.start = start, .end = end, .ends_with_eof = ends_with_eof};
    return PACK_SourceLine(source_line);
}
uint64_t lib_ruby_parser__internal__containers__source_line__get_start(const SourceLine_BLOB *blob)
{
    const SourceLine *source_line = (const SourceLine *)blob;
    return source_line->start;
}
uint64_t lib_ruby_parser__internal__containers__source_line__get_end(const SourceLine_BLOB *blob)
{
    const SourceLine *source_line = (const SourceLine *)blob;
    return source_line->end;
}
bool lib_ruby_parser__internal__containers__source_line__get_ends_with_eof(const SourceLine_BLOB *blob)
{
    const SourceLine *source_line = (const SourceLine *)blob;
    return source_line->ends_with_eof;
}
void lib_ruby_parser__internal__containers__source_line__set_start(SourceLine_BLOB *blob, uint64_t start)
{
    SourceLine *source_line = (SourceLine *)blob;
    source_line->start = start;
}
void lib_ruby_parser__internal__containers__source_line__set_end(SourceLine_BLOB *blob, uint64_t end)
{
    SourceLine *source_line = (SourceLine *)blob;
    source_line->end = end;
}
void lib_ruby_parser__internal__containers__source_line__set_ends_with_eof(SourceLine_BLOB *blob, bool ends_with_eof)
{
    SourceLine *source_line = (SourceLine *)blob;
    source_line->ends_with_eof = ends_with_eof;
}
void lib_ruby_parser__internal__containers__source_line__drop(SourceLine_BLOB *blob)
{
    (void)(blob);
}

/*
    Bytes
*/
void lib_ruby_parser__internal__containers__bytes__drop(Bytes_BLOB *blob)
{
    Bytes *bytes = (Bytes *)blob;
    drop_bytes(bytes);
}
Bytes_BLOB lib_ruby_parser__internal__containers__bytes__new_from_byte_list(ByteList_BLOB list_blob)
{
    Bytes bytes = {.raw = UNPACK_ByteList(list_blob)};
    return PACK_Bytes(bytes);
}
const ByteList_BLOB *lib_ruby_parser__internal__containers__bytes__get_byte_list(const Bytes_BLOB *blob)
{
    const Bytes *bytes = (const Bytes *)blob;
    return (const ByteList_BLOB *)(&(bytes->raw));
}
void lib_ruby_parser__internal__containers__bytes__set_byte_list(Bytes_BLOB *blob, ByteList_BLOB list_blob)
{
    Bytes *bytes = (Bytes *)blob;
    lib_ruby_parser__internal__containers__list__of_bytes__drop((ByteList_BLOB *)(&(bytes->raw)));
    bytes->raw = UNPACK_ByteList(list_blob);
}
ByteList_BLOB lib_ruby_parser__internal__containers__bytes__into_byte_list(Bytes_BLOB blob)
{
    return PACK_ByteList(UNPACK_Bytes(blob).raw);
}
void lib_ruby_parser__internal__containers__bytes__push(Bytes_BLOB *blob, Byte byte)
{
    Bytes *bytes = (Bytes *)blob;
    ByteList *byte_list = &(bytes->raw);
    lib_ruby_parser__internal__containers__list__of_bytes__push((ByteList_BLOB *)byte_list, byte);
}

/*
    Token
*/
Token_BLOB lib_ruby_parser__internal__containers__token__new(uint32_t token_type,
                                                             Bytes_BLOB token_value,
                                                             Loc_BLOB loc,
                                                             uint32_t lex_state_before,
                                                             uint32_t lex_state_after)
{
    Token token = {
        .token_type = token_type,
        .token_value = UNPACK_Bytes(token_value),
        .loc = UNPACK_Loc(loc),
        .lex_state_before = lex_state_before,
        .lex_state_after = lex_state_after};
    return PACK_Token(token);
}
uint32_t lib_ruby_parser__internal__containers__token__get_token_type(const Token_BLOB *blob)
{
    const Token *token = (const Token *)blob;
    return token->token_type;
}
const Bytes_BLOB *lib_ruby_parser__internal__containers__token__get_token_value(const Token_BLOB *blob)
{
    const Token *token = (const Token *)blob;
    return (Bytes_BLOB *)(&(token->token_value));
}
void lib_ruby_parser__internal__containers__token__set_token_value(Token_BLOB *blob, Bytes_BLOB token_value_blob)
{
    Token *token = (Token *)blob;
    drop_bytes(&(token->token_value));
    token->token_value = UNPACK_Bytes(token_value_blob);
}
Bytes_BLOB lib_ruby_parser__internal__containers__token__into_token_value(Token_BLOB blob)
{
    return PACK_Bytes(UNPACK_Token(blob).token_value);
}
const Loc_BLOB *lib_ruby_parser__internal__containers__token__get_loc(const Token_BLOB *blob)
{
    Token *token = (Token *)blob;
    return (Loc_BLOB *)(&(token->loc));
}
uint32_t lib_ruby_parser__internal__containers__token__get_lex_state_before(const Token_BLOB *blob)
{
    Token *token = (Token *)blob;
    return token->lex_state_before;
}
uint32_t lib_ruby_parser__internal__containers__token__get_lex_state_after(const Token_BLOB *blob)
{
    Token *token = (Token *)blob;
    return token->lex_state_after;
}
void lib_ruby_parser__internal__containers__token__drop(Token_BLOB *blob)
{
    Token *token = (Token *)blob;
    drop_token(token);
}

/*
    CommentType
*/
CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__new_inline()
{
    return PACK_CommentType(INLINE);
}
CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__new_document()
{
    return PACK_CommentType(DOCUMENT);
}
CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__new_unknown()
{
    return PACK_CommentType(UNKNOWN);
}
bool lib_ruby_parser__internal__containers__comment_type__is_inline(CommentType_BLOB blob)
{
    return UNPACK_CommentType(blob) == INLINE;
}
bool lib_ruby_parser__internal__containers__comment_type__is_document(CommentType_BLOB blob)
{
    return UNPACK_CommentType(blob) == DOCUMENT;
}
bool lib_ruby_parser__internal__containers__comment_type__is_unknown(CommentType_BLOB blob)
{
    return UNPACK_CommentType(blob) == UNKNOWN;
}

/*
    Comment
*/
Comment_BLOB lib_ruby_parser__internal__containers__comment__new(Loc_BLOB location, CommentType_BLOB kind)
{
    Comment comment = {.location = UNPACK_Loc(location), .kind = UNPACK_CommentType(kind)};
    return PACK_Comment(comment);
}
Loc_BLOB *lib_ruby_parser__internal__containers__comment__get_location(Comment_BLOB *blob)
{
    Comment *comment = (Comment *)blob;
    return (Loc_BLOB *)(&(comment->location));
}
CommentType_BLOB *lib_ruby_parser__internal__containers__comment__get_kind(Comment_BLOB *blob)
{
    Comment *comment = (Comment *)blob;
    return (CommentType_BLOB *)(&(comment->kind));
}
void lib_ruby_parser__internal__containers__comment__drop(Comment_BLOB *blob)
{
    (void)blob;
    // noop
}

/*
    ErrorLevel
*/
ErrorLevel_BLOB lib_ruby_parser__internal__containers__error_level__new_warning()
{
    return PACK_ErrorLevel(WARNING);
}
ErrorLevel_BLOB lib_ruby_parser__internal__containers__error_level__new_error()
{
    return PACK_ErrorLevel(ERROR);
}
bool lib_ruby_parser__internal__containers__error_level__is_warning(ErrorLevel_BLOB blob)
{
    return UNPACK_ErrorLevel(blob) == WARNING;
}
bool lib_ruby_parser__internal__containers__error_level__is_error(ErrorLevel_BLOB blob)
{
    return UNPACK_ErrorLevel(blob) == ERROR;
}

/*
    Loc
*/
Loc_BLOB lib_ruby_parser__internal__containers__loc__new(uint64_t begin, uint64_t end)
{
    Loc loc = {.begin = begin, .end = end};
    return PACK_Loc(loc);
}
uint64_t lib_ruby_parser__internal__containers__loc__begin(const Loc_BLOB *blob)
{
    const Loc *loc = (const Loc *)blob;
    return loc->begin;
}
uint64_t lib_ruby_parser__internal__containers__loc__end(const Loc_BLOB *blob)
{
    const Loc *loc = (const Loc *)blob;
    return loc->end;
}
void lib_ruby_parser__internal__containers__loc__drop(Loc_BLOB *blob)
{
    (void)blob;
    // noop
}

/*
    MaybeLoc
*/
MaybeLoc_BLOB lib_ruby_parser__internal__containers__maybe_loc__new_some(Loc_BLOB loc_blob)
{
    MaybeLoc maybe_loc = {.tag = MAYBE_LOC_SOME, .as = {.loc = UNPACK_Loc(loc_blob)}};
    return PACK_MaybeLoc(maybe_loc);
}
MaybeLoc_BLOB lib_ruby_parser__internal__containers__maybe_loc__new_none()
{
    MaybeLoc maybe_loc = {.tag = MAYBE_LOC_NONE, .as = {.nothing = {.dummy = 0}}};
    return PACK_MaybeLoc(maybe_loc);
}
bool lib_ruby_parser__internal__containers__maybe_loc__is_some(const MaybeLoc_BLOB *blob)
{
    const MaybeLoc *maybe_loc = (const MaybeLoc *)blob;
    return maybe_loc->tag == MAYBE_LOC_SOME;
}
bool lib_ruby_parser__internal__containers__maybe_loc__is_none(const MaybeLoc_BLOB *blob)
{
    const MaybeLoc *maybe_loc = (const MaybeLoc *)blob;
    return maybe_loc->tag == MAYBE_LOC_NONE;
}
Loc_BLOB *lib_ruby_parser__internal__containers__maybe_loc__get_loc(MaybeLoc_BLOB *blob)
{
    MaybeLoc *maybe_loc = (MaybeLoc *)blob;
    if (maybe_loc->tag == MAYBE_LOC_NONE)
        return NULL;
    return (Loc_BLOB *)(&(maybe_loc->as.loc));
}
Loc_BLOB lib_ruby_parser__internal__containers__maybe_loc__into_loc(MaybeLoc_BLOB blob)
{
    return PACK_Loc(UNPACK_MaybeLoc(blob).as.loc);
}
void lib_ruby_parser__internal__containers__maybe_loc__drop(MaybeLoc_BLOB *blob)
{
    (void)blob;
    // noop
}

/*
    MagicCommentKind
*/
MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__new_encoding()
{
    return PACK_MagicCommentKind(MAGIC_COMMENT_KIND_ENCODING);
}
MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__new_frozen_string_literal()
{
    return PACK_MagicCommentKind(MAGIC_COMMENT_KIND_FROZEN_STRING_LITERAL);
}
MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__new_warn_indent()
{
    return PACK_MagicCommentKind(MAGIC_COMMENT_KIND_WARN_INDENT);
}
MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__new_shareable_constant_value()
{
    return PACK_MagicCommentKind(MAGIC_COMMENT_KIND_SHAREABLE_CONSTANT_VALUE);
}

bool lib_ruby_parser__internal__containers__magic_comment_kind__is_encoding(MagicCommentKind_BLOB blob)
{
    return UNPACK_MagicCommentKind(blob) == MAGIC_COMMENT_KIND_ENCODING;
}
bool lib_ruby_parser__internal__containers__magic_comment_kind__is_frozen_string_literal(MagicCommentKind_BLOB blob)
{
    return UNPACK_MagicCommentKind(blob) == MAGIC_COMMENT_KIND_FROZEN_STRING_LITERAL;
}
bool lib_ruby_parser__internal__containers__magic_comment_kind__is_warn_indent(MagicCommentKind_BLOB blob)
{
    return UNPACK_MagicCommentKind(blob) == MAGIC_COMMENT_KIND_WARN_INDENT;
}
bool lib_ruby_parser__internal__containers__magic_comment_kind__is_shareable_constant_value(MagicCommentKind_BLOB blob)
{
    return UNPACK_MagicCommentKind(blob) == MAGIC_COMMENT_KIND_SHAREABLE_CONSTANT_VALUE;
}

/*
    MagicComment
*/

MagicComment_BLOB lib_ruby_parser__internal__containers__magic_comment__new(MagicCommentKind_BLOB kind, Loc_BLOB key_l, Loc_BLOB value_l)
{
    MagicComment magic_comment = {.kind = UNPACK_MagicCommentKind(kind), .key_l = UNPACK_Loc(key_l), .value_l = UNPACK_Loc(value_l)};
    return PACK_MagicComment(magic_comment);
}
const MagicCommentKind_BLOB *lib_ruby_parser__internal__containers__magic_comment__get_kind(const MagicComment_BLOB *blob)
{
    const MagicComment *magic_comment = (const MagicComment *)blob;
    return (const MagicCommentKind_BLOB *)(&(magic_comment->kind));
}
const Loc_BLOB *lib_ruby_parser__internal__containers__magic_comment__get_key_l(const MagicComment_BLOB *blob)
{
    const MagicComment *magic_comment = (const MagicComment *)blob;
    return (const Loc_BLOB *)(&(magic_comment->key_l));
}
const Loc_BLOB *lib_ruby_parser__internal__containers__magic_comment__get_value_l(const MagicComment_BLOB *blob)
{
    const MagicComment *magic_comment = (const MagicComment *)blob;
    return (const Loc_BLOB *)(&(magic_comment->value_l));
}
void lib_ruby_parser__internal__containers__magic_comment__drop(MagicComment_BLOB *blob)
{
    (void)blob;
    // noop
}

/*
    SharedByteList
*/
SharedByteList_BLOB lib_ruby_parser__internal__containers__shared_byte_list__new(const uint8_t *ptr, uint64_t len)
{
    SharedByteList shared_byte_list = {.ptr = ptr, .len = len};
    return PACK_SharedByteList(shared_byte_list);
}
const uint8_t *lib_ruby_parser__internal__containers__shared_byte_list__get_raw(const SharedByteList_BLOB *blob)
{
    const SharedByteList *shared_byte_list = (const SharedByteList *)blob;
    if (shared_byte_list->len == 0)
        return NULL;
    return shared_byte_list->ptr;
}
uint64_t lib_ruby_parser__internal__containers__shared_byte_list__get_len(const SharedByteList_BLOB *blob)
{
    const SharedByteList *shared_byte_list = (const SharedByteList *)blob;
    return shared_byte_list->len;
}

/*
    Diagnostic
*/
Diagnostic_BLOB lib_ruby_parser__internal__containers__diagnostic__new(ErrorLevel_BLOB level, DiagnosticMessage_BLOB message, Loc_BLOB loc)
{
    Diagnostic diagnostic = {.level = UNPACK_ErrorLevel(level), .message = UNPACK_DiagnosticMessage(message), .loc = UNPACK_Loc(loc)};
    return PACK_Diagnostic(diagnostic);
}
const ErrorLevel_BLOB *lib_ruby_parser__internal__containers__diagnostic__get_level(const Diagnostic_BLOB *blob)
{
    const Diagnostic *diagnostic = (const Diagnostic *)blob;
    return (const ErrorLevel_BLOB *)(&(diagnostic->level));
}
const DiagnosticMessage_BLOB *lib_ruby_parser__internal__containers__diagnostic__get_message(const Diagnostic_BLOB *blob)
{
    const Diagnostic *diagnostic = (const Diagnostic *)blob;
    return (const DiagnosticMessage_BLOB *)(&(diagnostic->message));
}
const Loc_BLOB *lib_ruby_parser__internal__containers__diagnostic__get_loc(const Diagnostic_BLOB *blob)
{
    const Diagnostic *diagnostic = (const Diagnostic *)blob;
    return (const Loc_BLOB *)(&(diagnostic->loc));
}
void lib_ruby_parser__internal__containers__diagnostic__drop(Diagnostic_BLOB *blob)
{
    Diagnostic *diagnostic = (Diagnostic *)blob;
    drop_diagnostic(diagnostic);
}

/*
    InputError
*/

InputError_BLOB lib_ruby_parser__internal__containers__input_error__new_unsupported_encoding(StringPtr_BLOB err)
{
    InputError input_error = {.tag = UNSUPPORTED_ENCODING, .as = {.unsupported_encoding = UNPACK_StringPtr(err)}};
    return PACK_InputError(input_error);
}
InputError_BLOB lib_ruby_parser__internal__containers__input_error__new_decoding_error(StringPtr_BLOB err)
{
    InputError input_error = {.tag = DECODING_ERROR, .as = {.decoding_error = UNPACK_StringPtr(err)}};
    return PACK_InputError(input_error);
}
bool lib_ruby_parser__internal__containers__input_error__is_unsupported_encoding(const InputError_BLOB *blob)
{
    const InputError *input_error = (const InputError *)blob;
    return input_error->tag == UNSUPPORTED_ENCODING;
}
bool lib_ruby_parser__internal__containers__input_error__is_decoding_error(const InputError_BLOB *blob)
{
    const InputError *input_error = (const InputError *)blob;
    return input_error->tag == DECODING_ERROR;
}
const StringPtr_BLOB *lib_ruby_parser__internal__containers__input_error__get_unsupported_encoding(const InputError_BLOB *blob)
{
    const InputError *input_error = (const InputError *)blob;
    if (input_error->tag == UNSUPPORTED_ENCODING)
        return (const StringPtr_BLOB *)(&(input_error->as.unsupported_encoding));
    return NULL;
}
const StringPtr_BLOB *lib_ruby_parser__internal__containers__input_error__get_decoding_error(const InputError_BLOB *blob)
{
    const InputError *input_error = (const InputError *)blob;
    if (input_error->tag == DECODING_ERROR)
        return (const StringPtr_BLOB *)(&(input_error->as.decoding_error));
    return NULL;
}
void lib_ruby_parser__internal__containers__input_error__drop(InputError_BLOB *blob)
{
    InputError *input_error = (InputError *)blob;
    drop_input_error(input_error);
}

/*
    DecoderResult
*/
DecoderResult_BLOB lib_ruby_parser__internal__containers__decoder_result__new_ok(ByteList_BLOB byte_list)
{
    DecoderResult decoder_result = {.tag = DECODE_OK, .as = {.ok = UNPACK_ByteList(byte_list)}};
    return PACK_DecoderResult(decoder_result);
}
DecoderResult_BLOB lib_ruby_parser__internal__containers__decoder_result__new_err(InputError_BLOB input_error)
{
    DecoderResult decoder_result = {.tag = DECODE_ERR, .as = {.err = UNPACK_InputError(input_error)}};
    return PACK_DecoderResult(decoder_result);
}
bool lib_ruby_parser__internal__containers__decoder_result_is_ok(const DecoderResult_BLOB *blob)
{
    const DecoderResult *decoder_result = (const DecoderResult *)blob;
    return decoder_result->tag == DECODE_OK;
}
bool lib_ruby_parser__internal__containers__decoder_result_is_err(const DecoderResult_BLOB *blob)
{
    const DecoderResult *decoder_result = (const DecoderResult *)blob;
    return decoder_result->tag == DECODE_ERR;
}
ByteList_BLOB lib_ruby_parser__internal__containers__decoder_result_into_ok(DecoderResult_BLOB blob)
{
    return PACK_ByteList(UNPACK_DecoderResult(blob).as.ok);
}
InputError_BLOB lib_ruby_parser__internal__containers__decoder_result_into_err(DecoderResult_BLOB blob)
{
    return PACK_InputError(UNPACK_DecoderResult(blob).as.err);
}
const ByteList_BLOB *lib_ruby_parser__internal__containers__decoder_result_as_ok(const DecoderResult_BLOB *blob)
{
    const DecoderResult *decoder_result = (const DecoderResult *)blob;
    return (const ByteList_BLOB *)(&(decoder_result->as.ok));
}
const InputError_BLOB *lib_ruby_parser__internal__containers__decoder_result_as_err(const DecoderResult_BLOB *blob)
{
    const DecoderResult *decoder_result = (const DecoderResult *)blob;
    return (const InputError_BLOB *)(&(decoder_result->as.err));
}
void lib_ruby_parser__internal__containers__decoder_result__drop(DecoderResult_BLOB *blob)
{
    DecoderResult *decoder_result = (DecoderResult *)blob;
    drop_decoder_result(decoder_result);
}

/*
    Decoder
*/
DecoderResult_BLOB lib_ruby_parser__internal__containers__decoder__call(
    const Decoder_BLOB *blob,
    StringPtr_BLOB encoding_blob,
    ByteList_BLOB input_blob)
{
    // cleanup unused values that we own
    StringPtr encoding = UNPACK_StringPtr(encoding_blob);
    drop_string_ptr(&encoding);
    ByteList input = UNPACK_ByteList(input_blob);
    drop_byte_list(&input);

    // call dummy decoder
    const Decoder *decoder = (const Decoder *)blob;
    return PACK_DecoderResult(decoder->f());
}
void lib_ruby_parser__internal__containers__decoder_drop(Decoder_BLOB *blob)
{
    (void)blob;
}
Decoder_BLOB lib_ruby_parser__internal__containers__decoder__new(dummy_decoder_t f)
{
    Decoder decoder = {.f = f};
    return PACK_Decoder(decoder);
}
