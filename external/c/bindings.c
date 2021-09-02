#include <stdlib.h>
#include <string.h>
#include "bindings.h"

/*
    Ptr
*/

Ptr_BLOB lib_ruby_parser__external__ptr__new(void *ptr)
{
    return PACK_Ptr(ptr);
}
void lib_ruby_parser__external__ptr__of_node__drop(Ptr_BLOB *self_blob)
{
    Ptr self = *((Ptr *)self_blob);
    drop_node((Node *)self);
    free(self);
}
void lib_ruby_parser__external__ptr__of_token__drop(Ptr_BLOB *self_blob)
{
    Ptr self = *((Ptr *)self_blob);
    drop_token((Token *)self);
    free(self);
}
void *lib_ruby_parser__external__ptr__get_raw(Ptr_BLOB *self_blob)
{
    Ptr self = *((Ptr *)self_blob);
    return self;
}

/*
    MaybePtr
*/
MaybePtr_BLOB lib_ruby_parser__external__maybe_ptr__new(void *ptr)
{
    return PACK_MaybePtr(ptr);
}
MaybePtr_BLOB lib_ruby_parser__external__maybe_ptr__new_null()
{
    return PACK_MaybePtr(NULL);
}
void lib_ruby_parser__external__maybe_ptr__of_node__drop(MaybePtr_BLOB *self_blob)
{
    MaybePtr self = *((MaybePtr *)self_blob);
    if (self != NULL)
    {
        drop_node((Node *)self);
        free(self);
    }
}
void lib_ruby_parser__external__maybe_ptr__of_token__drop(MaybePtr_BLOB *self_blob)
{
    MaybePtr self = *((MaybePtr *)self_blob);
    if (self != NULL)
    {
        drop_token((Token *)self);
        free(self);
    }
}
void *lib_ruby_parser__external__maybe_ptr__get_raw(MaybePtr_BLOB *self_blob)
{
    MaybePtr self = *((MaybePtr *)self_blob);
    return self;
}

/*
    StringPtr
*/
StringPtr_BLOB lib_ruby_parser__external__string_ptr__new(const uint8_t *ptr, uint64_t len)
{
    uint8_t *new_ptr = malloc(len);
    memcpy(new_ptr, ptr, len);
    return PACK_StringPtr(((StringPtr){.ptr = new_ptr, .len = len}));
}
void lib_ruby_parser__external__string_ptr__drop(StringPtr_BLOB *self_blob)
{
    StringPtr *self = (StringPtr *)self_blob;
    drop_string_ptr(self);
}
uint8_t *lib_ruby_parser__external__string_ptr__get_raw(StringPtr_BLOB *self_blob)
{
    StringPtr *self = (StringPtr *)self_blob;
    if (self->len == 0)
        return NULL;
    return self->ptr;
}
uint64_t lib_ruby_parser__external__string_ptr__get_len(const StringPtr_BLOB *self_blob)
{
    StringPtr *self = (StringPtr *)self_blob;
    return self->len;
}

/*
    MaybeStringPtr
*/
MaybeStringPtr_BLOB lib_ruby_parser__external__maybe_string_ptr__new_some(const uint8_t *ptr, uint64_t len)
{
    MaybeStringPtr maybe_string_ptr = {.ptr = malloc(len), .len = len};
    memcpy(maybe_string_ptr.ptr, ptr, len);
    return PACK_MaybeStringPtr(maybe_string_ptr);
}
MaybeStringPtr_BLOB lib_ruby_parser__external__maybe_string_ptr__new_none()
{
    return PACK_MaybeStringPtr(((MaybeStringPtr){.ptr = NULL, .len = 0}));
}
void lib_ruby_parser__external__maybe_string_ptr__drop(MaybeStringPtr_BLOB *self_blob)
{
    MaybeStringPtr *self = (MaybeStringPtr *)self_blob;
    drop_maybe_string_ptr(self);
}
bool lib_ruby_parser__external__maybe_string_ptr__is_some(const MaybeStringPtr_BLOB *self_blob)
{
    const MaybeStringPtr *self = (const MaybeStringPtr *)self_blob;
    return self->ptr != NULL;
}
bool lib_ruby_parser__external__maybe_string_ptr__is_none(const MaybeStringPtr_BLOB *self_blob)
{
    const MaybeStringPtr *self = (const MaybeStringPtr *)self_blob;
    return self->ptr == NULL;
}
uint8_t *lib_ruby_parser__external__maybe_string_ptr__get_raw(MaybeStringPtr_BLOB *self_blob)
{
    MaybeStringPtr *self = (MaybeStringPtr *)self_blob;
    return self->ptr;
}
uint8_t *lib_ruby_parser__external__maybe_string_ptr__into_raw(MaybeStringPtr_BLOB self_blob)
{
    MaybeStringPtr self = UNPACK_MaybeStringPtr(self_blob);
    return self.ptr;
}
uint64_t lib_ruby_parser__external__maybe_string_ptr__get_len(const MaybeStringPtr_BLOB *self_blob)
{
    const MaybeStringPtr *self = (const MaybeStringPtr *)self_blob;
    return self->len;
}

/*
    SharedByteList
*/
SharedByteList_BLOB lib_ruby_parser__external__shared_byte_list__new(const uint8_t *ptr, uint64_t len)
{
    return PACK_SharedByteList(((SharedByteList){.ptr = ptr, .len = len}));
}
void lib_ruby_parser__external__shared_byte_list__drop(SharedByteList_BLOB *self_blob)
{
    (void)self_blob;
}
const uint8_t *lib_ruby_parser__external__shared_byte_list__get_raw(const SharedByteList_BLOB *self_blob)
{
    const SharedByteList *self = (const SharedByteList *)self_blob;
    if (self->len == 0)
        return NULL;
    return self->ptr;
}
uint64_t lib_ruby_parser__external__shared_byte_list__get_len(const SharedByteList_BLOB *self_blob)
{
    const SharedByteList *self = (const SharedByteList *)self_blob;
    return self->len;
}

/*
    Lists
*/
#define LIST_IMPL(ITEM, ITEM_BLOB, LIST, LIST_BLOB, NS, drop)                                       \
    LIST_BLOB lib_ruby_parser__external__list__##NS##__new()                                        \
    {                                                                                               \
        return PACK_##LIST(((LIST){.ptr = NULL, .len = 0, .capacity = 0}));                         \
    }                                                                                               \
    void lib_ruby_parser__external__list__##NS##__drop(LIST_BLOB *self_blob)                        \
    {                                                                                               \
        LIST *self = (LIST *)self_blob;                                                             \
        for (uint64_t i = 0; i < self->len; i++)                                                    \
        {                                                                                           \
            drop(&self->ptr[i]);                                                                    \
        }                                                                                           \
        free(self->ptr);                                                                            \
    }                                                                                               \
    LIST_BLOB lib_ruby_parser__external__list__##NS##__with_capacity(uint64_t capacity)             \
    {                                                                                               \
        LIST list = {.ptr = malloc(sizeof(ITEM) * capacity), .len = 0, .capacity = capacity};       \
        return PACK_##LIST(list);                                                                   \
    }                                                                                               \
    LIST_BLOB lib_ruby_parser__external__list__##NS##__from_raw(ITEM_BLOB *ptr, uint64_t len)       \
    {                                                                                               \
        if (len > 0)                                                                                \
        {                                                                                           \
            LIST list = {.ptr = (ITEM *)ptr, .len = len, .capacity = len};                          \
            return PACK_##LIST(list);                                                               \
        }                                                                                           \
        else                                                                                        \
        {                                                                                           \
            return lib_ruby_parser__external__list__##NS##__new();                                  \
        }                                                                                           \
    }                                                                                               \
    void lib_ruby_parser__external__list__##NS##__push(LIST_BLOB *self_blob, ITEM_BLOB item_blob)   \
    {                                                                                               \
        LIST *self = (LIST *)self_blob;                                                             \
        ITEM item = UNPACK_##ITEM(item_blob);                                                       \
        if (self->len + 1 > self->capacity)                                                         \
        {                                                                                           \
            if (self->capacity == 0)                                                                \
            {                                                                                       \
                self->capacity += 1;                                                                \
            }                                                                                       \
            else                                                                                    \
            {                                                                                       \
                self->capacity *= 2;                                                                \
            }                                                                                       \
            ITEM *old_ptr = self->ptr;                                                              \
            ITEM *new_ptr = malloc(sizeof(ITEM) * self->capacity);                                  \
            memcpy(new_ptr, old_ptr, sizeof(ITEM) * self->len);                                     \
            self->ptr = new_ptr;                                                                    \
            free(old_ptr);                                                                          \
        }                                                                                           \
        self->ptr[self->len] = item;                                                                \
        self->len++;                                                                                \
    }                                                                                               \
    ITEM_BLOB lib_ruby_parser__external__list__##NS##__remove(LIST_BLOB *self_blob, uint64_t index) \
    {                                                                                               \
        LIST *self = (LIST *)self_blob;                                                             \
        ITEM item = self->ptr[index];                                                               \
        memmove(self->ptr + index, self->ptr + index + 1, sizeof(ITEM) * (self->len - index - 1));  \
        self->len--;                                                                                \
        return PACK_##ITEM(item);                                                                   \
    }                                                                                               \
    void lib_ruby_parser__external__list__##NS##__shrink_to_fit(LIST_BLOB *self_blob)               \
    {                                                                                               \
        LIST *self = (LIST *)self_blob;                                                             \
                                                                                                    \
        uint64_t new_len = self->len;                                                               \
        uint64_t new_capacity = self->len;                                                          \
                                                                                                    \
        ITEM *new_ptr = malloc(sizeof(ITEM) * new_capacity);                                        \
        memcpy(new_ptr, self->ptr, sizeof(ITEM) * new_len);                                         \
                                                                                                    \
        ITEM *old_ptr = self->ptr;                                                                  \
        self->ptr = new_ptr;                                                                        \
        self->len = new_len;                                                                        \
        self->capacity = new_capacity;                                                              \
        free(old_ptr);                                                                              \
    }                                                                                               \
    ITEM_BLOB *lib_ruby_parser__external__list__##NS##__as_ptr(LIST_BLOB *self_blob)                \
    {                                                                                               \
        LIST *self = (LIST *)self_blob;                                                             \
        return (ITEM_BLOB *)(self->ptr);                                                            \
    }                                                                                               \
    uint64_t lib_ruby_parser__external__list__##NS##__get_len(const LIST_BLOB *self_blob)           \
    {                                                                                               \
        const LIST *self = (const LIST *)self_blob;                                                 \
        return self->len;                                                                           \
    }                                                                                               \
    uint64_t lib_ruby_parser__external__list__##NS##__get_capacity(const LIST_BLOB *self_blob)      \
    {                                                                                               \
        const LIST *self = (const LIST *)self_blob;                                                 \
        return self->capacity;                                                                      \
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
SourceLine_BLOB lib_ruby_parser__external__source_line__new(uint64_t start, uint64_t end, bool ends_with_eof)
{
    return PACK_SourceLine(((SourceLine){.start = start, .end = end, .ends_with_eof = ends_with_eof}));
}
void lib_ruby_parser__external__source_line__drop(SourceLine_BLOB *self_blob)
{
    (void)(self_blob);
}
uint64_t lib_ruby_parser__external__source_line__get_start(const SourceLine_BLOB *self_blob)
{
    const SourceLine *self = (const SourceLine *)self_blob;
    return self->start;
}
uint64_t lib_ruby_parser__external__source_line__get_end(const SourceLine_BLOB *self_blob)
{
    const SourceLine *self = (const SourceLine *)self_blob;
    return self->end;
}
bool lib_ruby_parser__external__source_line__get_ends_with_eof(const SourceLine_BLOB *self_blob)
{
    const SourceLine *self = (const SourceLine *)self_blob;
    return self->ends_with_eof;
}
void lib_ruby_parser__external__source_line__set_start(SourceLine_BLOB *self_blob, uint64_t start)
{
    SourceLine *self = (SourceLine *)self_blob;
    self->start = start;
}
void lib_ruby_parser__external__source_line__set_end(SourceLine_BLOB *self_blob, uint64_t end)
{
    SourceLine *self = (SourceLine *)self_blob;
    self->end = end;
}
void lib_ruby_parser__external__source_line__set_ends_with_eof(SourceLine_BLOB *self_blob, bool ends_with_eof)
{
    SourceLine *self = (SourceLine *)self_blob;
    self->ends_with_eof = ends_with_eof;
}

/*
    Bytes
*/
Bytes_BLOB lib_ruby_parser__external__bytes__new(ByteList_BLOB raw_blob)
{
    return PACK_Bytes(((Bytes){.raw = UNPACK_ByteList(raw_blob)}));
}
void lib_ruby_parser__external__bytes__drop(Bytes_BLOB *self_blob)
{
    Bytes *self = (Bytes *)self_blob;
    drop_bytes(self);
}
const ByteList_BLOB *lib_ruby_parser__external__bytes__get_raw(const Bytes_BLOB *self_blob)
{
    const Bytes *self = (const Bytes *)self_blob;
    return (const ByteList_BLOB *)(&(self->raw));
}
void lib_ruby_parser__external__bytes__set_raw(Bytes_BLOB *self_blob, ByteList_BLOB raw_blob)
{
    Bytes *self = (Bytes *)self_blob;
    lib_ruby_parser__external__list__of_bytes__drop((ByteList_BLOB *)(&(self->raw)));
    self->raw = UNPACK_ByteList(raw_blob);
}
ByteList_BLOB lib_ruby_parser__external__bytes__into_raw(Bytes_BLOB self_blob)
{
    return PACK_ByteList(UNPACK_Bytes(self_blob).raw);
}
void lib_ruby_parser__external__bytes__push(Bytes_BLOB *self_blob, Byte_BLOB byte_blob)
{
    Bytes *self = (Bytes *)self_blob;
    ByteList *byte_list = &(self->raw);
    lib_ruby_parser__external__list__of_bytes__push((ByteList_BLOB *)byte_list, byte_blob);
}

/*
    Token
*/
Token_BLOB lib_ruby_parser__external__token__new(
    uint32_t token_type,
    Bytes_BLOB token_value_blob,
    Loc_BLOB loc_blob,
    uint32_t lex_state_before,
    uint32_t lex_state_after)
{
    return PACK_Token(((Token){
        .token_type = token_type,
        .token_value = UNPACK_Bytes(token_value_blob),
        .loc = UNPACK_Loc(loc_blob),
        .lex_state_before = lex_state_before,
        .lex_state_after = lex_state_after}));
}
void lib_ruby_parser__external__token__drop(Token_BLOB *self_blob)
{
    Token *self = (Token *)self_blob;
    drop_token(self);
}
uint32_t lib_ruby_parser__external__token__get_token_type(const Token_BLOB *self_blob)
{
    const Token *self = (const Token *)self_blob;
    return self->token_type;
}
const Bytes_BLOB *lib_ruby_parser__external__token__get_token_value(const Token_BLOB *self_blob)
{
    const Token *self = (const Token *)self_blob;
    return (Bytes_BLOB *)(&(self->token_value));
}
void lib_ruby_parser__external__token__set_token_value(Token_BLOB *self_blob, Bytes_BLOB token_value_blob)
{
    Token *self = (Token *)self_blob;
    drop_bytes(&(self->token_value));
    self->token_value = UNPACK_Bytes(token_value_blob);
}
Bytes_BLOB lib_ruby_parser__external__token__into_token_value(Token_BLOB self_blob)
{
    return PACK_Bytes(UNPACK_Token(self_blob).token_value);
}
const Loc_BLOB *lib_ruby_parser__external__token__get_loc(const Token_BLOB *self_blob)
{
    Token *self = (Token *)self_blob;
    return (Loc_BLOB *)(&(self->loc));
}
uint32_t lib_ruby_parser__external__token__get_lex_state_before(const Token_BLOB *self_blob)
{
    Token *self = (Token *)self_blob;
    return self->lex_state_before;
}
uint32_t lib_ruby_parser__external__token__get_lex_state_after(const Token_BLOB *self_blob)
{
    Token *self = (Token *)self_blob;
    return self->lex_state_after;
}

/*
    CommentType
*/
CommentType_BLOB lib_ruby_parser__external__comment_type__new_inline()
{
    return PACK_CommentType(INLINE);
}
CommentType_BLOB lib_ruby_parser__external__comment_type__new_document()
{
    return PACK_CommentType(DOCUMENT);
}
CommentType_BLOB lib_ruby_parser__external__comment_type__new_unknown()
{
    return PACK_CommentType(UNKNOWN);
}
void lib_ruby_parser__external__comment_type__drop(CommentType_BLOB *self_blob)
{
    (void)self_blob;
}
bool lib_ruby_parser__external__comment_type__is_inline(const CommentType_BLOB *self_blob)
{
    CommentType *self = (CommentType *)self_blob;
    return *self == INLINE;
}
bool lib_ruby_parser__external__comment_type__is_document(const CommentType_BLOB *self_blob)
{
    CommentType *self = (CommentType *)self_blob;
    return *self == DOCUMENT;
}
bool lib_ruby_parser__external__comment_type__is_unknown(const CommentType_BLOB *self_blob)
{
    CommentType *self = (CommentType *)self_blob;
    return *self == UNKNOWN;
}

/*
    Comment
*/
Comment_BLOB lib_ruby_parser__external__comment__new(Loc_BLOB location_blob, CommentType_BLOB kind_blob)
{
    return PACK_Comment(((Comment){.location = UNPACK_Loc(location_blob), .kind = UNPACK_CommentType(kind_blob)}));
}
void lib_ruby_parser__external__comment__drop(Comment_BLOB *self_blob)
{
    (void)self_blob;
}
const Loc_BLOB *lib_ruby_parser__external__comment__get_location(const Comment_BLOB *self_blob)
{
    Comment *self = (Comment *)self_blob;
    return (Loc_BLOB *)(&(self->location));
}
const CommentType_BLOB *lib_ruby_parser__external__comment__get_kind(const Comment_BLOB *self_blob)
{
    Comment *self = (Comment *)self_blob;
    return (CommentType_BLOB *)(&(self->kind));
}

/*
    Loc
*/
Loc_BLOB lib_ruby_parser__external__loc__new(uint64_t begin, uint64_t end)
{
    return PACK_Loc(((Loc){.begin = begin, .end = end}));
}
void lib_ruby_parser__external__loc__drop(Loc_BLOB *self_blob)
{
    (void)self_blob;
}
uint64_t lib_ruby_parser__external__loc__get_begin(const Loc_BLOB *self_blob)
{
    const Loc *self = (const Loc *)self_blob;
    return self->begin;
}
uint64_t lib_ruby_parser__external__loc__get_end(const Loc_BLOB *self_blob)
{
    const Loc *self = (const Loc *)self_blob;
    return self->end;
}

/*
    MaybeLoc
*/
MaybeLoc_BLOB lib_ruby_parser__external__maybe_loc__new_some(Loc_BLOB loc_blob)
{
    return PACK_MaybeLoc(((MaybeLoc){.tag = MAYBE_LOC_SOME, .as = {.loc = UNPACK_Loc(loc_blob)}}));
}
MaybeLoc_BLOB lib_ruby_parser__external__maybe_loc__new_none()
{
    return PACK_MaybeLoc(((MaybeLoc){.tag = MAYBE_LOC_NONE, .as = {.nothing = {.dummy = 0}}}));
}
void lib_ruby_parser__external__maybe_loc__drop(MaybeLoc_BLOB *self_blob)
{
    (void)self_blob;
}
bool lib_ruby_parser__external__maybe_loc__is_some(const MaybeLoc_BLOB *self_blob)
{
    const MaybeLoc *self = (const MaybeLoc *)self_blob;
    return self->tag == MAYBE_LOC_SOME;
}
bool lib_ruby_parser__external__maybe_loc__is_none(const MaybeLoc_BLOB *self_blob)
{
    const MaybeLoc *self = (const MaybeLoc *)self_blob;
    return self->tag == MAYBE_LOC_NONE;
}
const Loc_BLOB *lib_ruby_parser__external__maybe_loc__as_loc(const MaybeLoc_BLOB *self_blob)
{
    MaybeLoc *self = (MaybeLoc *)self_blob;
    if (self->tag == MAYBE_LOC_NONE)
        return NULL;
    return (Loc_BLOB *)(&(self->as.loc));
}
Loc_BLOB lib_ruby_parser__external__maybe_loc__into_loc(MaybeLoc_BLOB self_blob)
{
    return PACK_Loc(UNPACK_MaybeLoc(self_blob).as.loc);
}

/*
    MagicCommentKind
*/
MagicCommentKind_BLOB lib_ruby_parser__external__magic_comment_kind__new_encoding()
{
    return PACK_MagicCommentKind(MAGIC_COMMENT_KIND_ENCODING);
}
MagicCommentKind_BLOB lib_ruby_parser__external__magic_comment_kind__new_frozen_string_literal()
{
    return PACK_MagicCommentKind(MAGIC_COMMENT_KIND_FROZEN_STRING_LITERAL);
}
MagicCommentKind_BLOB lib_ruby_parser__external__magic_comment_kind__new_warn_indent()
{
    return PACK_MagicCommentKind(MAGIC_COMMENT_KIND_WARN_INDENT);
}
MagicCommentKind_BLOB lib_ruby_parser__external__magic_comment_kind__new_shareable_constant_value()
{
    return PACK_MagicCommentKind(MAGIC_COMMENT_KIND_SHAREABLE_CONSTANT_VALUE);
}
void lib_ruby_parser__external__magic_comment_kind__drop(MagicCommentKind_BLOB *self_blob)
{
    (void)self_blob;
}
bool lib_ruby_parser__external__magic_comment_kind__is_encoding(const MagicCommentKind_BLOB *self_blob)
{
    const MagicCommentKind *self = (const MagicCommentKind *)self_blob;
    return *self == MAGIC_COMMENT_KIND_ENCODING;
}
bool lib_ruby_parser__external__magic_comment_kind__is_frozen_string_literal(const MagicCommentKind_BLOB *self_blob)
{
    const MagicCommentKind *self = (const MagicCommentKind *)self_blob;
    return *self == MAGIC_COMMENT_KIND_FROZEN_STRING_LITERAL;
}
bool lib_ruby_parser__external__magic_comment_kind__is_warn_indent(const MagicCommentKind_BLOB *self_blob)
{
    const MagicCommentKind *self = (const MagicCommentKind *)self_blob;
    return *self == MAGIC_COMMENT_KIND_WARN_INDENT;
}
bool lib_ruby_parser__external__magic_comment_kind__is_shareable_constant_value(const MagicCommentKind_BLOB *self_blob)
{
    const MagicCommentKind *self = (const MagicCommentKind *)self_blob;
    return *self == MAGIC_COMMENT_KIND_SHAREABLE_CONSTANT_VALUE;
}

/*
    MagicComment
*/

MagicComment_BLOB lib_ruby_parser__external__magic_comment__new(
    MagicCommentKind_BLOB kind_blob,
    Loc_BLOB key_l_blob,
    Loc_BLOB value_l_blob)
{
    return PACK_MagicComment(((MagicComment){
        .kind = UNPACK_MagicCommentKind(kind_blob),
        .key_l = UNPACK_Loc(key_l_blob),
        .value_l = UNPACK_Loc(value_l_blob)}));
}
void lib_ruby_parser__external__magic_comment__drop(MagicComment_BLOB *self_blob)
{
    (void)self_blob;
}
const MagicCommentKind_BLOB *lib_ruby_parser__external__magic_comment__get_kind(const MagicComment_BLOB *self_blob)
{
    const MagicComment *self = (const MagicComment *)self_blob;
    return (const MagicCommentKind_BLOB *)(&(self->kind));
}
const Loc_BLOB *lib_ruby_parser__external__magic_comment__get_key_l(const MagicComment_BLOB *self_blob)
{
    const MagicComment *self = (const MagicComment *)self_blob;
    return (const Loc_BLOB *)(&(self->key_l));
}
const Loc_BLOB *lib_ruby_parser__external__magic_comment__get_value_l(const MagicComment_BLOB *self_blob)
{
    const MagicComment *self = (const MagicComment *)self_blob;
    return (const Loc_BLOB *)(&(self->value_l));
}

/*
    ErrorLevel
*/
ErrorLevel_BLOB lib_ruby_parser__external__error_level__new_warning()
{
    return PACK_ErrorLevel(WARNING);
}
ErrorLevel_BLOB lib_ruby_parser__external__error_level__new_error()
{
    return PACK_ErrorLevel(ERROR);
}
void lib_ruby_parser__external__error_level__drop(ErrorLevel_BLOB *self_blob)
{
    (void)self_blob;
}
bool lib_ruby_parser__external__error_level__is_warning(const ErrorLevel_BLOB *self_blob)
{
    const ErrorLevel *self = (const ErrorLevel *)self_blob;
    return *self == WARNING;
}
bool lib_ruby_parser__external__error_level__is_error(const ErrorLevel_BLOB *self_blob)
{
    const ErrorLevel *self = (const ErrorLevel *)self_blob;
    return *self == ERROR;
}

/*
    Diagnostic
*/
Diagnostic_BLOB lib_ruby_parser__external__diagnostic__new(
    ErrorLevel_BLOB level_blob,
    DiagnosticMessage_BLOB message_blob,
    Loc_BLOB loc_blob)
{
    return PACK_Diagnostic(((Diagnostic){
        .level = UNPACK_ErrorLevel(level_blob),
        .message = UNPACK_DiagnosticMessage(message_blob),
        .loc = UNPACK_Loc(loc_blob)}));
}
void lib_ruby_parser__external__diagnostic__drop(Diagnostic_BLOB *self_blob)
{
    Diagnostic *self = (Diagnostic *)self_blob;
    drop_diagnostic(self);
}
const ErrorLevel_BLOB *lib_ruby_parser__external__diagnostic__get_level(const Diagnostic_BLOB *self_blob)
{
    const Diagnostic *self = (const Diagnostic *)self_blob;
    return (const ErrorLevel_BLOB *)(&(self->level));
}
const DiagnosticMessage_BLOB *lib_ruby_parser__external__diagnostic__get_message(const Diagnostic_BLOB *self_blob)
{
    const Diagnostic *self = (const Diagnostic *)self_blob;
    return (const DiagnosticMessage_BLOB *)(&(self->message));
}
const Loc_BLOB *lib_ruby_parser__external__diagnostic__get_loc(const Diagnostic_BLOB *self_blob)
{
    const Diagnostic *self = (const Diagnostic *)self_blob;
    return (const Loc_BLOB *)(&(self->loc));
}

/*
    InputError
*/

InputError_BLOB lib_ruby_parser__external__input_error__new_unsupported_encoding(StringPtr_BLOB err_blob)
{
    return PACK_InputError(((InputError){
        .tag = UNSUPPORTED_ENCODING,
        .as = {.unsupported_encoding = UNPACK_StringPtr(err_blob)}}));
}
InputError_BLOB lib_ruby_parser__external__input_error__new_decoding_error(StringPtr_BLOB err_blob)
{
    return PACK_InputError(((InputError){
        .tag = DECODING_ERROR,
        .as = {.decoding_error = UNPACK_StringPtr(err_blob)}}));
}
void lib_ruby_parser__external__input_error__drop(InputError_BLOB *self_blob)
{
    InputError *self = (InputError *)self_blob;
    drop_input_error(self);
}
bool lib_ruby_parser__external__input_error__is_unsupported_encoding(const InputError_BLOB *self_blob)
{
    const InputError *self = (const InputError *)self_blob;
    return self->tag == UNSUPPORTED_ENCODING;
}
bool lib_ruby_parser__external__input_error__is_decoding_error(const InputError_BLOB *self_blob)
{
    const InputError *self = (const InputError *)self_blob;
    return self->tag == DECODING_ERROR;
}
const StringPtr_BLOB *lib_ruby_parser__external__input_error__get_unsupported_encoding(const InputError_BLOB *self_blob)
{
    const InputError *self = (const InputError *)self_blob;
    if (self->tag == UNSUPPORTED_ENCODING)
        return (const StringPtr_BLOB *)(&(self->as.unsupported_encoding));
    return NULL;
}
const StringPtr_BLOB *lib_ruby_parser__external__input_error__get_decoding_error(const InputError_BLOB *self_blob)
{
    const InputError *self = (const InputError *)self_blob;
    if (self->tag == DECODING_ERROR)
        return (const StringPtr_BLOB *)(&(self->as.decoding_error));
    return NULL;
}

/*
    DecoderResult
*/
DecoderResult_BLOB lib_ruby_parser__external__decoder_result__new_ok(ByteList_BLOB byte_list_blob)
{
    return PACK_DecoderResult(((DecoderResult){
        .tag = DECODE_OK,
        .as = {.ok = UNPACK_ByteList(byte_list_blob)}}));
}
DecoderResult_BLOB lib_ruby_parser__external__decoder_result__new_err(InputError_BLOB input_error_blob)
{
    return PACK_DecoderResult(((DecoderResult){
        .tag = DECODE_ERR,
        .as = {.err = UNPACK_InputError(input_error_blob)}}));
}
void lib_ruby_parser__external__decoder_result__drop(DecoderResult_BLOB *self_blob)
{
    DecoderResult *self = (DecoderResult *)self_blob;
    drop_decoder_result(self);
}
bool lib_ruby_parser__external__decoder_result_is_ok(const DecoderResult_BLOB *self_blob)
{
    const DecoderResult *self = (const DecoderResult *)self_blob;
    return self->tag == DECODE_OK;
}
bool lib_ruby_parser__external__decoder_result_is_err(const DecoderResult_BLOB *self_blob)
{
    const DecoderResult *self = (const DecoderResult *)self_blob;
    return self->tag == DECODE_ERR;
}
ByteList_BLOB lib_ruby_parser__external__decoder_result_into_ok(DecoderResult_BLOB self_blob)
{
    return PACK_ByteList(UNPACK_DecoderResult(self_blob).as.ok);
}
InputError_BLOB lib_ruby_parser__external__decoder_result_into_err(DecoderResult_BLOB self_blob)
{
    return PACK_InputError(UNPACK_DecoderResult(self_blob).as.err);
}
const ByteList_BLOB *lib_ruby_parser__external__decoder_result_as_ok(const DecoderResult_BLOB *self_blob)
{
    const DecoderResult *self = (const DecoderResult *)self_blob;
    return (const ByteList_BLOB *)(&(self->as.ok));
}
const InputError_BLOB *lib_ruby_parser__external__decoder_result_as_err(const DecoderResult_BLOB *self_blob)
{
    const DecoderResult *self = (const DecoderResult *)self_blob;
    return (const InputError_BLOB *)(&(self->as.err));
}

/*
    Decoder
*/
DecoderResult_BLOB lib_ruby_parser__external__decoder__call(
    Decoder_BLOB *self_blob,
    StringPtr_BLOB encoding_blob,
    ByteList_BLOB input_blob)
{
    // cleanup unused values that we own
    StringPtr encoding = UNPACK_StringPtr(encoding_blob);
    drop_string_ptr(&encoding);
    ByteList input = UNPACK_ByteList(input_blob);
    drop_byte_list(&input);

    // call dummy decoder
    Decoder *self = (Decoder *)self_blob;
    return PACK_DecoderResult(self->f());
}
void lib_ruby_parser__external__decoder_drop(Decoder_BLOB *self_blob)
{
    (void)self_blob;
}
Decoder_BLOB lib_ruby_parser__external__decoder__new(dummy_decoder_t f)
{
    return PACK_Decoder(((Decoder){.f = f}));
}

/*
    RewriteAction
*/
void lib_ruby_parser__external__rewrite_action__drop(RewriteAction_BLOB *self_blob)
{
    (void)self_blob;
}
bool lib_ruby_parser__external__rewrite_action__is_drop(const RewriteAction_BLOB *self_blob)
{
    const RewriteAction *self = (const RewriteAction *)self_blob;
    return *self == REWRITE_ACTION_DROP;
}
bool lib_ruby_parser__external__rewrite_action__is_keep(const RewriteAction_BLOB *self_blob)
{
    const RewriteAction *self = (const RewriteAction *)self_blob;
    return *self == REWRITE_ACTION_KEEP;
}

/*
    LexStateAction
*/
void lib_ruby_parser__external__lex_state_action__drop(LexStateAction_BLOB *self_blob)
{
    (void)self_blob;
}
bool lib_ruby_parser__external__lex_state_action__is_set(const LexStateAction_BLOB *self_blob)
{
    const LexStateAction *self = (const LexStateAction *)self_blob;
    return self->tag == LEX_STATE_SET;
}
bool lib_ruby_parser__external__lex_state_action__is_keep(const LexStateAction_BLOB *self_blob)
{
    const LexStateAction *self = (const LexStateAction *)self_blob;
    return self->tag == LEX_STATE_KEEP;
}
int32_t lib_ruby_parser__external__lex_state_action__get_next_state(const LexStateAction_BLOB *self_blob)
{
    const LexStateAction *self = (const LexStateAction *)self_blob;
    return self->as.set;
}

/*
    TokenRewriterResult
*/
void lib_ruby_parser__external__token_rewriter_result__drop(TokenRewriterResult_BLOB *self_blob)
{
    TokenRewriterResult *self = (TokenRewriterResult *)self_blob;
    drop_token_rewriter_result(self);
}
InternalTokenRewriterResult lib_ruby_parser__external__token_rewriter_result__into_internal(TokenRewriterResult_BLOB self_blob)
{
    TokenRewriterResult self = UNPACK_TokenRewriterResult(self_blob);
    return ((InternalTokenRewriterResult){
        .token_action = PACK_RewriteAction(self.token_action),
        .lex_state_action = PACK_LexStateAction(self.lex_state_action),
        .rewritten_token = PACK_Ptr(self.rewritten_token)});
}

/*
    TokenRewriter
*/
void lib_ruby_parser__external__token_rewriter__drop(TokenRewriter_BLOB *self_blob)
{
    (void)self_blob;
}
TokenRewriterResult_BLOB lib_ruby_parser__external__token_rewriter__call(
    TokenRewriter_BLOB *self_blob,
    Ptr_BLOB token_blob,
    SharedByteList_BLOB input_blob)
{
    Ptr token = UNPACK_Ptr(token_blob);
    (void)input_blob;

    // call dummy token_rewriter
    TokenRewriter *self = (TokenRewriter *)self_blob;
    TokenRewriterResult result = self->rewrite_f(token, self->build_new_token_f);
    return PACK_TokenRewriterResult(result);
}
// Test APIs
TokenRewriter_BLOB lib_ruby_parser__external__token_rewriter__new_keep(build_new_token_t build_new_token_f)
{
    return PACK_TokenRewriter(__keep_token_rewriter(build_new_token_f));
}
TokenRewriter_BLOB lib_ruby_parser__external__token_rewriter__new_drop(build_new_token_t build_new_token_f)
{
    return PACK_TokenRewriter(__drop_token_rewriter(build_new_token_f));
}
TokenRewriter_BLOB lib_ruby_parser__external__token_rewriter__new_rewrite(
    build_new_token_t build_new_token_f)
{
    return PACK_TokenRewriter(__rewriter_token_rewriter(build_new_token_f));
}

/*
    MaybeDecoder
*/
MaybeDecoder_BLOB lib_ruby_parser__external__maybe_decoder__new_some(Decoder_BLOB decoder_blob)
{
    return PACK_MaybeDecoder(((MaybeDecoder){
        .tag = MAYBE_DECODER_SOME,
        .as = {.decoder = UNPACK_Decoder(decoder_blob)}}));
}
MaybeDecoder_BLOB lib_ruby_parser__external__maybe_decoder__new_none()
{
    return PACK_MaybeDecoder(((MaybeDecoder){
        .tag = MAYBE_DECODER_NONE,
        .as = {.nothing = {.dummy = 42}}}));
}
void lib_ruby_parser__external__maybe_decoder__drop(MaybeDecoder_BLOB *self_blob)
{
    (void)self_blob;
}
bool lib_ruby_parser__external__maybe_decoder__is_some(const MaybeDecoder_BLOB *self_blob)
{
    const MaybeDecoder *self = (const MaybeDecoder *)self_blob;
    return self->tag == MAYBE_DECODER_SOME;
}
bool lib_ruby_parser__external__maybe_decoder__is_none(const MaybeDecoder_BLOB *self_blob)
{
    const MaybeDecoder *self = (const MaybeDecoder *)self_blob;
    return self->tag == MAYBE_DECODER_NONE;
}
const Decoder_BLOB *lib_ruby_parser__external__maybe_decoder__as_decoder(const MaybeDecoder_BLOB *self_blob)
{
    const MaybeDecoder *self = (const MaybeDecoder *)self_blob;
    if (self->tag == MAYBE_DECODER_SOME)
        return (const Decoder_BLOB *)(&(self->as.decoder));
    return NULL;
}
Decoder_BLOB lib_ruby_parser__external__maybe_decoder__into_decoder(MaybeDecoder_BLOB self_blob)
{
    return PACK_Decoder(UNPACK_MaybeDecoder(self_blob).as.decoder);
}

/*
    MaybeTokenRewriter
*/
MaybeTokenRewriter_BLOB lib_ruby_parser__external__maybe_token_rewriter__new_some(TokenRewriter_BLOB token_rewriter_blob)
{
    return PACK_MaybeTokenRewriter(((MaybeTokenRewriter){
        .tag = MAYBE_TOKEN_REWRITER_SOME,
        .as = {.token_rewriter = UNPACK_TokenRewriter(token_rewriter_blob)}}));
}
MaybeTokenRewriter_BLOB lib_ruby_parser__external__maybe_token_rewriter__new_none()
{
    return PACK_MaybeTokenRewriter(((MaybeTokenRewriter){
        .tag = MAYBE_TOKEN_REWRITER_NONE,
        .as = {.nothing = {.dummy = 42}}}));
}
void lib_ruby_parser__external__maybe_token_rewriter__drop(MaybeTokenRewriter_BLOB *self_blob)
{
    (void)self_blob;
}
bool lib_ruby_parser__external__maybe_token_rewriter__is_some(const MaybeTokenRewriter_BLOB *self_blob)
{
    const MaybeTokenRewriter *self = (const MaybeTokenRewriter *)self_blob;
    return self->tag == MAYBE_TOKEN_REWRITER_SOME;
}
bool lib_ruby_parser__external__maybe_token_rewriter__is_none(const MaybeTokenRewriter_BLOB *self_blob)
{
    const MaybeTokenRewriter *self = (const MaybeTokenRewriter *)self_blob;
    return self->tag == MAYBE_TOKEN_REWRITER_NONE;
}
const TokenRewriter_BLOB *lib_ruby_parser__external__maybe_token_rewriter__as_token_rewriter(const MaybeTokenRewriter_BLOB *self_blob)
{
    const MaybeTokenRewriter *self = (const MaybeTokenRewriter *)self_blob;
    if (self->tag == MAYBE_TOKEN_REWRITER_SOME)
        return (const TokenRewriter_BLOB *)(&(self->as.token_rewriter));
    return NULL;
}
TokenRewriter_BLOB lib_ruby_parser__external__maybe_token_rewriter__into_token_rewriter(MaybeTokenRewriter_BLOB self_blob)
{
    return PACK_TokenRewriter(UNPACK_MaybeTokenRewriter(self_blob).as.token_rewriter);
}

/*
    ParserOptions
*/
ParserOptions_BLOB lib_ruby_parser__external__parser_options__new(
    StringPtr_BLOB buffer_name_blob,
    uint8_t debug,
    MaybeDecoder_BLOB decoder_blob,
    MaybeTokenRewriter_BLOB token_rewriter_blob,
    bool record_tokens)
{
    return PACK_ParserOptions(((ParserOptions){
        .buffer_name = UNPACK_StringPtr(buffer_name_blob),
        .debug = debug,
        .decoder = UNPACK_MaybeDecoder(decoder_blob),
        .token_rewriter = UNPACK_MaybeTokenRewriter(token_rewriter_blob),
        .record_tokens = record_tokens}));
}
void lib_ruby_parser__external__parser_options__drop(ParserOptions_BLOB *self_blob)
{
    ParserOptions *options = (ParserOptions *)self_blob;
    drop_string_ptr(&(options->buffer_name));
}
InternalParserOptions lib_ruby_parser__external__parser_options__into_internal(ParserOptions_BLOB self_blob)
{
    ParserOptions self = UNPACK_ParserOptions(self_blob);
    return ((InternalParserOptions){
        .buffer_name = PACK_StringPtr(self.buffer_name),
        .debug = self.debug,
        .decoder = PACK_MaybeDecoder(self.decoder),
        .token_rewriter = PACK_MaybeTokenRewriter(self.token_rewriter),
        .record_tokens = self.record_tokens});
}
const StringPtr_BLOB *lib_ruby_parser__external__parser_options__get_buffer_name(const ParserOptions_BLOB *self_blob)
{
    const ParserOptions *self = (const ParserOptions *)self_blob;
    return (const StringPtr_BLOB *)(&(self->buffer_name));
}
uint8_t lib_ruby_parser__external__parser_options__get_debug(const ParserOptions_BLOB *self_blob)
{
    const ParserOptions *self = (const ParserOptions *)self_blob;
    return self->debug;
}
const MaybeDecoder_BLOB *lib_ruby_parser__external__parser_options__get_decoder(const ParserOptions_BLOB *self_blob)
{
    const ParserOptions *self = (const ParserOptions *)self_blob;
    return (const MaybeDecoder_BLOB *)(&(self->decoder));
}
const MaybeTokenRewriter_BLOB *lib_ruby_parser__external__parser_options__get_token_rewriter(const ParserOptions_BLOB *self_blob)
{
    const ParserOptions *self = (const ParserOptions *)self_blob;
    return (const MaybeTokenRewriter_BLOB *)(&(self->token_rewriter));
}
bool lib_ruby_parser__external__parser_options__get_record_tokens(const ParserOptions_BLOB *self_blob)
{
    const ParserOptions *self = (const ParserOptions *)self_blob;
    return self->record_tokens;
}

// DecodedInput
DecodedInput_BLOB lib_ruby_parser__external__decoded_input__new(
    StringPtr_BLOB name_blob,
    SourceLineList_BLOB lines_blob,
    ByteList_BLOB bytes_blob)
{
    return PACK_DecodedInput(((DecodedInput){
        .name = UNPACK_StringPtr(name_blob),
        .lines = UNPACK_SourceLineList(lines_blob),
        .bytes = UNPACK_ByteList(bytes_blob)}));
}
void lib_ruby_parser__external__decoded_input__drop(DecodedInput_BLOB *self_blob)
{
    DecodedInput *self = (DecodedInput *)self_blob;
    drop_decoded_input(self);
}
const StringPtr_BLOB *lib_ruby_parser__external__decoded_input__get_name(const DecodedInput_BLOB *self_blob)
{
    const DecodedInput *self = (const DecodedInput *)self_blob;
    return (const StringPtr_BLOB *)(&(self->name));
}
const SourceLineList_BLOB *lib_ruby_parser__external__decoded_input__get_lines(const DecodedInput_BLOB *self_blob)
{
    const DecodedInput *self = (const DecodedInput *)self_blob;
    return (const SourceLineList_BLOB *)(&(self->lines));
}
const ByteList_BLOB *lib_ruby_parser__external__decoded_input__get_bytes(const DecodedInput_BLOB *self_blob)
{
    const DecodedInput *self = (const DecodedInput *)self_blob;
    return (const ByteList_BLOB *)(&(self->bytes));
}
void lib_ruby_parser__external__decoded_input__set_name(DecodedInput_BLOB *self_blob, StringPtr_BLOB name)
{
    DecodedInput *self = (DecodedInput *)self_blob;
    drop_string_ptr(&(self->name));
    self->name = UNPACK_StringPtr(name);
}
void lib_ruby_parser__external__decoded_input__set_lines(DecodedInput_BLOB *self_blob, SourceLineList_BLOB lines)
{
    DecodedInput *self = (DecodedInput *)self_blob;
    drop_source_line_list(&(self->lines));
    self->lines = UNPACK_SourceLineList(lines);
}
void lib_ruby_parser__external__decoded_input__set_bytes(DecodedInput_BLOB *self_blob, ByteList_BLOB bytes)
{
    DecodedInput *self = (DecodedInput *)self_blob;
    drop_byte_list(&(self->bytes));
    self->bytes = UNPACK_ByteList(bytes);
}
ByteList_BLOB lib_ruby_parser__external__decoded_input__into_bytes(DecodedInput_BLOB self_blob)
{
    DecodedInput self = UNPACK_DecodedInput(self_blob);
    drop_string_ptr(&(self.name));
    drop_source_line_list(&(self.lines));
    return PACK_ByteList(self.bytes);
}
ByteList_BLOB lib_ruby_parser__external__decoded_input__take_bytes(DecodedInput_BLOB *self_blob)
{
    DecodedInput *self = (DecodedInput *)self_blob;
    ByteList bytes = self->bytes;
    ByteList empty = {.ptr = NULL, .len = 0, .capacity = 0};
    self->bytes = empty;
    return PACK_ByteList(bytes);
}

/*
    ParserResult
*/
ParserResult_BLOB lib_ruby_parser__external__parser_result__new(
    MaybePtr_BLOB ast_blob,
    TokenList_BLOB tokens_blob,
    DiagnosticList_BLOB diagnostics_blob,
    CommentList_BLOB comments_blob,
    MagicCommentList_BLOB magic_comments_blob,
    DecodedInput_BLOB input_blob)
{
    return PACK_ParserResult(((ParserResult){
        .ast = UNPACK_MaybePtr(ast_blob),
        .tokens = UNPACK_TokenList(tokens_blob),
        .diagnostics = UNPACK_DiagnosticList(diagnostics_blob),
        .comments = UNPACK_CommentList(comments_blob),
        .magic_comments = UNPACK_MagicCommentList(magic_comments_blob),
        .input = UNPACK_DecodedInput(input_blob)}));
}
void lib_ruby_parser__external__parser_result__drop(ParserResult_BLOB *self_blob)
{
    ParserResult *self = (ParserResult *)self_blob;
    drop_parser_result(self);
}
const MaybePtr_BLOB *lib_ruby_parser__external__parser_result__get_ast(const ParserResult_BLOB *self_blob)
{
    const ParserResult *self = (const ParserResult *)self_blob;
    return (const MaybePtr_BLOB *)(&(self->ast));
}
const TokenList_BLOB *lib_ruby_parser__external__parser_result__get_tokens(const ParserResult_BLOB *self_blob)
{
    const ParserResult *self = (const ParserResult *)self_blob;
    return (const TokenList_BLOB *)(&(self->tokens));
}
const DiagnosticList_BLOB *lib_ruby_parser__external__parser_result__get_diagnostics(const ParserResult_BLOB *self_blob)
{
    const ParserResult *self = (const ParserResult *)self_blob;
    return (const DiagnosticList_BLOB *)(&(self->diagnostics));
}
const CommentList_BLOB *lib_ruby_parser__external__parser_result__get_comments(const ParserResult_BLOB *self_blob)
{
    const ParserResult *self = (const ParserResult *)self_blob;
    return (const CommentList_BLOB *)(&(self->comments));
}
const MagicCommentList_BLOB *lib_ruby_parser__external__parser_result__get_magic_comments(const ParserResult_BLOB *self_blob)
{
    const ParserResult *self = (const ParserResult *)self_blob;
    return (const MagicCommentList_BLOB *)(&(self->magic_comments));
}
const DecodedInput_BLOB *lib_ruby_parser__external__parser_result__get_input(const ParserResult_BLOB *self_blob)
{
    const ParserResult *self = (const ParserResult *)self_blob;
    return (const DecodedInput_BLOB *)(&(self->input));
}
