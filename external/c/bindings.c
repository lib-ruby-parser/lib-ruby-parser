#include <stdlib.h>
#include <string.h>
#include "bindings.h"

/*
    LIB_RUBY_PARSER_Ptr
*/

LIB_RUBY_PARSER_Ptr_BLOB lib_ruby_parser__external__ptr__new(void *ptr)
{
    return PACK_Ptr(ptr);
}
void lib_ruby_parser__external__ptr__of_node__drop(LIB_RUBY_PARSER_Ptr_BLOB *self_blob)
{
    LIB_RUBY_PARSER_Ptr self = *((LIB_RUBY_PARSER_Ptr *)self_blob);
    LIB_RUBY_PARSER_drop_node((LIB_RUBY_PARSER_Node *)self);
    free(self);
}
void lib_ruby_parser__external__ptr__of_token__drop(LIB_RUBY_PARSER_Ptr_BLOB *self_blob)
{
    LIB_RUBY_PARSER_Ptr self = *((LIB_RUBY_PARSER_Ptr *)self_blob);
    LIB_RUBY_PARSER_drop_token((LIB_RUBY_PARSER_Token *)self);
    free(self);
}
const void *lib_ruby_parser__external__ptr__get_raw(const LIB_RUBY_PARSER_Ptr_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_Ptr self = *((const LIB_RUBY_PARSER_Ptr *)self_blob);
    return self;
}
void *lib_ruby_parser__external__ptr__into_raw(LIB_RUBY_PARSER_Ptr_BLOB self_blob)
{
    LIB_RUBY_PARSER_Ptr self = UNPACK_Ptr(self_blob);
    return self;
}

/*
    LIB_RUBY_PARSER_MaybeLoc
*/
LIB_RUBY_PARSER_MaybeLoc_BLOB lib_ruby_parser__external__maybe__loc__new_some(LIB_RUBY_PARSER_Loc_BLOB value)
{
    return PACK_MaybeLoc(((LIB_RUBY_PARSER_MaybeLoc){
        .tag = LIB_RUBY_PARSER_MAYBE_LOC_SOME,
        .as = {.loc = UNPACK_Loc(value)}}));
}
LIB_RUBY_PARSER_MaybeLoc_BLOB lib_ruby_parser__external__maybe__loc__new_none()
{
    return PACK_MaybeLoc(((LIB_RUBY_PARSER_MaybeLoc){
        .tag = LIB_RUBY_PARSER_MAYBE_LOC_NONE,
        .as = {.nothing = {.dummy = 0}}}));
}
void lib_ruby_parser__external__maybe__loc__drop(LIB_RUBY_PARSER_MaybeLoc_BLOB *self_blob)
{
    (void)self_blob;
}
bool lib_ruby_parser__external__maybe__loc__is_some(const LIB_RUBY_PARSER_MaybeLoc_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MaybeLoc *self = (const LIB_RUBY_PARSER_MaybeLoc *)self_blob;
    return self->tag == LIB_RUBY_PARSER_MAYBE_LOC_SOME;
}
bool lib_ruby_parser__external__maybe__loc__is_none(const LIB_RUBY_PARSER_MaybeLoc_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MaybeLoc *self = (const LIB_RUBY_PARSER_MaybeLoc *)self_blob;
    return self->tag == LIB_RUBY_PARSER_MAYBE_LOC_NONE;
}
const LIB_RUBY_PARSER_Loc_BLOB *lib_ruby_parser__external__maybe__loc__as_value(const LIB_RUBY_PARSER_MaybeLoc_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MaybeLoc *self = (const LIB_RUBY_PARSER_MaybeLoc *)self_blob;
    if (self->tag == LIB_RUBY_PARSER_MAYBE_LOC_NONE)
        return NULL;
    return (const LIB_RUBY_PARSER_Loc_BLOB *)(&(self->as.loc));
}
LIB_RUBY_PARSER_Loc_BLOB lib_ruby_parser__external__maybe__loc__into_value(LIB_RUBY_PARSER_MaybeLoc_BLOB self_blob)
{
    return PACK_Loc(UNPACK_MaybeLoc(self_blob).as.loc);
}

/*
    LIB_RUBY_PARSER_MaybePtr
*/
LIB_RUBY_PARSER_MaybePtr_BLOB lib_ruby_parser__external__maybe__ptr__new_some(LIB_RUBY_PARSER_Ptr_BLOB value)
{
    LIB_RUBY_PARSER_Ptr self = UNPACK_Ptr(value);
    return PACK_MaybePtr(self);
}
LIB_RUBY_PARSER_MaybePtr_BLOB lib_ruby_parser__external__maybe__ptr__new_none()
{
    return PACK_MaybePtr(NULL);
}
void lib_ruby_parser__external__maybe__ptr__of_node__drop(LIB_RUBY_PARSER_MaybePtr_BLOB *self_blob)
{
    LIB_RUBY_PARSER_MaybePtr self = *((LIB_RUBY_PARSER_MaybePtr *)self_blob);
    if (self != NULL)
    {
        LIB_RUBY_PARSER_drop_node((LIB_RUBY_PARSER_Node *)self);
        free(self);
    }
}
void lib_ruby_parser__external__maybe__ptr__of_token__drop(LIB_RUBY_PARSER_MaybePtr_BLOB *self_blob)
{
    LIB_RUBY_PARSER_MaybePtr self = *((LIB_RUBY_PARSER_MaybePtr *)self_blob);
    if (self != NULL)
    {
        LIB_RUBY_PARSER_drop_token((LIB_RUBY_PARSER_Token *)self);
        free(self);
    }
}
bool lib_ruby_parser__external__maybe__ptr__is_some(const LIB_RUBY_PARSER_MaybePtr_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MaybePtr self = *((const LIB_RUBY_PARSER_MaybePtr *)self_blob);
    return self != NULL;
}
bool lib_ruby_parser__external__maybe__ptr__is_none(const LIB_RUBY_PARSER_MaybePtr_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MaybePtr self = *((const LIB_RUBY_PARSER_MaybePtr *)self_blob);
    return self == NULL;
}
const LIB_RUBY_PARSER_Ptr_BLOB *lib_ruby_parser__external__maybe__ptr__as_value(const LIB_RUBY_PARSER_MaybePtr_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MaybePtr *self = (const LIB_RUBY_PARSER_MaybePtr *)self_blob;
    return (const LIB_RUBY_PARSER_Ptr_BLOB *)self;
}
LIB_RUBY_PARSER_Ptr_BLOB lib_ruby_parser__external__maybe__ptr__into_value(LIB_RUBY_PARSER_MaybePtr_BLOB self_blob)
{
    LIB_RUBY_PARSER_MaybePtr self = UNPACK_MaybePtr(self_blob);
    return PACK_Ptr(self);
}

/*
    LIB_RUBY_PARSER_MaybeStringPtr
*/
LIB_RUBY_PARSER_MaybeStringPtr_BLOB lib_ruby_parser__external__maybe__string_ptr__new_some(LIB_RUBY_PARSER_StringPtr_BLOB value)
{
    LIB_RUBY_PARSER_StringPtr string_ptr = UNPACK_StringPtr(value);
    LIB_RUBY_PARSER_MaybeStringPtr maybe_string_ptr = {.ptr = string_ptr.ptr, .len = string_ptr.len};
    return PACK_MaybeStringPtr(maybe_string_ptr);
}
LIB_RUBY_PARSER_MaybeStringPtr_BLOB lib_ruby_parser__external__maybe__string_ptr__new_none()
{
    return PACK_MaybeStringPtr(((LIB_RUBY_PARSER_MaybeStringPtr){.ptr = NULL, .len = 0}));
}
void lib_ruby_parser__external__maybe__string_ptr__drop(LIB_RUBY_PARSER_MaybeStringPtr_BLOB *self_blob)
{
    LIB_RUBY_PARSER_MaybeStringPtr *self = (LIB_RUBY_PARSER_MaybeStringPtr *)self_blob;
    LIB_RUBY_PARSER_drop_maybe_string_ptr(self);
}
bool lib_ruby_parser__external__maybe__string_ptr__is_some(const LIB_RUBY_PARSER_MaybeStringPtr_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MaybeStringPtr *self = (const LIB_RUBY_PARSER_MaybeStringPtr *)self_blob;
    return self->ptr != NULL;
}
bool lib_ruby_parser__external__maybe__string_ptr__is_none(const LIB_RUBY_PARSER_MaybeStringPtr_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MaybeStringPtr *self = (const LIB_RUBY_PARSER_MaybeStringPtr *)self_blob;
    return self->ptr == NULL;
}
const LIB_RUBY_PARSER_StringPtr_BLOB *lib_ruby_parser__external__maybe__string_ptr__as_value(const LIB_RUBY_PARSER_MaybeStringPtr_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MaybeStringPtr *self = (const LIB_RUBY_PARSER_MaybeStringPtr *)self_blob;
    // they have equal structure
    return (const LIB_RUBY_PARSER_StringPtr_BLOB *)self;
}
LIB_RUBY_PARSER_StringPtr_BLOB lib_ruby_parser__external__maybe__string_ptr__into_value(LIB_RUBY_PARSER_MaybeStringPtr_BLOB self_blob)
{
    LIB_RUBY_PARSER_MaybeStringPtr self = UNPACK_MaybeStringPtr(self_blob);
    LIB_RUBY_PARSER_StringPtr string_ptr = {.ptr = self.ptr, .len = self.len};
    return PACK_StringPtr(string_ptr);
}

/*
    LIB_RUBY_PARSER_MaybeDecoder
*/
LIB_RUBY_PARSER_MaybeDecoder_BLOB lib_ruby_parser__external__maybe__decoder__new_some(LIB_RUBY_PARSER_Decoder_BLOB value)
{
    return PACK_MaybeDecoder(((LIB_RUBY_PARSER_MaybeDecoder){
        .tag = LIB_RUBY_PARSER_MAYBE_DECODER_SOME,
        .as = {.decoder = UNPACK_Decoder(value)}}));
}
LIB_RUBY_PARSER_MaybeDecoder_BLOB lib_ruby_parser__external__maybe__decoder__new_none()
{
    return PACK_MaybeDecoder(((LIB_RUBY_PARSER_MaybeDecoder){
        .tag = LIB_RUBY_PARSER_MAYBE_DECODER_NONE,
        .as = {.nothing = {.dummy = 42}}}));
}
void lib_ruby_parser__external__maybe__decoder__drop(LIB_RUBY_PARSER_MaybeDecoder_BLOB *self_blob)
{
    (void)self_blob;
}
bool lib_ruby_parser__external__maybe__decoder__is_some(const LIB_RUBY_PARSER_MaybeDecoder_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MaybeDecoder *self = (const LIB_RUBY_PARSER_MaybeDecoder *)self_blob;
    return self->tag == LIB_RUBY_PARSER_MAYBE_DECODER_SOME;
}
bool lib_ruby_parser__external__maybe__decoder__is_none(const LIB_RUBY_PARSER_MaybeDecoder_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MaybeDecoder *self = (const LIB_RUBY_PARSER_MaybeDecoder *)self_blob;
    return self->tag == LIB_RUBY_PARSER_MAYBE_DECODER_NONE;
}
const LIB_RUBY_PARSER_Decoder_BLOB *lib_ruby_parser__external__maybe__decoder__as_value(const LIB_RUBY_PARSER_MaybeDecoder_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MaybeDecoder *self = (const LIB_RUBY_PARSER_MaybeDecoder *)self_blob;
    if (self->tag == LIB_RUBY_PARSER_MAYBE_DECODER_SOME)
        return (const LIB_RUBY_PARSER_Decoder_BLOB *)(&(self->as.decoder));
    return NULL;
}
LIB_RUBY_PARSER_Decoder_BLOB lib_ruby_parser__external__maybe__decoder__into_value(LIB_RUBY_PARSER_MaybeDecoder_BLOB self_blob)
{
    return PACK_Decoder(UNPACK_MaybeDecoder(self_blob).as.decoder);
}

/*
    LIB_RUBY_PARSER_MaybeTokenRewriter
*/
LIB_RUBY_PARSER_MaybeTokenRewriter_BLOB lib_ruby_parser__external__maybe__token_rewriter__new_some(LIB_RUBY_PARSER_TokenRewriter_BLOB value)
{
    return PACK_MaybeTokenRewriter(((LIB_RUBY_PARSER_MaybeTokenRewriter){
        .tag = LIB_RUBY_PARSER_MAYBE_TOKEN_REWRITER_SOME,
        .as = {.token_rewriter = UNPACK_TokenRewriter(value)}}));
}
LIB_RUBY_PARSER_MaybeTokenRewriter_BLOB lib_ruby_parser__external__maybe__token_rewriter__new_none()
{
    return PACK_MaybeTokenRewriter(((LIB_RUBY_PARSER_MaybeTokenRewriter){
        .tag = LIB_RUBY_PARSER_MAYBE_TOKEN_REWRITER_NONE,
        .as = {.nothing = {.dummy = 42}}}));
}
void lib_ruby_parser__external__maybe__token_rewriter__drop(LIB_RUBY_PARSER_MaybeTokenRewriter_BLOB *self_blob)
{
    (void)self_blob;
}
bool lib_ruby_parser__external__maybe__token_rewriter__is_some(const LIB_RUBY_PARSER_MaybeTokenRewriter_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MaybeTokenRewriter *self = (const LIB_RUBY_PARSER_MaybeTokenRewriter *)self_blob;
    return self->tag == LIB_RUBY_PARSER_MAYBE_TOKEN_REWRITER_SOME;
}
bool lib_ruby_parser__external__maybe__token_rewriter__is_none(const LIB_RUBY_PARSER_MaybeTokenRewriter_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MaybeTokenRewriter *self = (const LIB_RUBY_PARSER_MaybeTokenRewriter *)self_blob;
    return self->tag == LIB_RUBY_PARSER_MAYBE_TOKEN_REWRITER_NONE;
}
const LIB_RUBY_PARSER_TokenRewriter_BLOB *lib_ruby_parser__external__maybe__token_rewriter__as_value(const LIB_RUBY_PARSER_MaybeTokenRewriter_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MaybeTokenRewriter *self = (const LIB_RUBY_PARSER_MaybeTokenRewriter *)self_blob;
    if (self->tag == LIB_RUBY_PARSER_MAYBE_TOKEN_REWRITER_SOME)
        return (const LIB_RUBY_PARSER_TokenRewriter_BLOB *)(&(self->as.token_rewriter));
    return NULL;
}
LIB_RUBY_PARSER_TokenRewriter_BLOB lib_ruby_parser__external__maybe__token_rewriter__into_value(LIB_RUBY_PARSER_MaybeTokenRewriter_BLOB self_blob)
{
    return PACK_TokenRewriter(UNPACK_MaybeTokenRewriter(self_blob).as.token_rewriter);
}

/*
    LIB_RUBY_PARSER_StringPtr
*/
LIB_RUBY_PARSER_StringPtr_BLOB lib_ruby_parser__external__string_ptr__new(const uint8_t *ptr, uint64_t len)
{
    uint8_t *new_ptr = malloc(len);
    memcpy(new_ptr, ptr, len);
    return PACK_StringPtr(((LIB_RUBY_PARSER_StringPtr){.ptr = new_ptr, .len = len}));
}
void lib_ruby_parser__external__string_ptr__drop(LIB_RUBY_PARSER_StringPtr_BLOB *self_blob)
{
    LIB_RUBY_PARSER_StringPtr *self = (LIB_RUBY_PARSER_StringPtr *)self_blob;
    LIB_RUBY_PARSER_drop_string_ptr(self);
}
const uint8_t *lib_ruby_parser__external__string_ptr__as_raw(const LIB_RUBY_PARSER_StringPtr_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_StringPtr *self = (const LIB_RUBY_PARSER_StringPtr *)self_blob;
    if (self->len == 0)
        return NULL;
    return self->ptr;
}
uint64_t lib_ruby_parser__external__string_ptr__get_len(const LIB_RUBY_PARSER_StringPtr_BLOB *self_blob)
{
    LIB_RUBY_PARSER_StringPtr *self = (LIB_RUBY_PARSER_StringPtr *)self_blob;
    return self->len;
}

/*
    LIB_RUBY_PARSER_SharedByteList
*/
LIB_RUBY_PARSER_SharedByteList_BLOB lib_ruby_parser__external__shared_byte_list__new(const uint8_t *ptr, uint64_t len)
{
    return PACK_SharedByteList(((LIB_RUBY_PARSER_SharedByteList){.ptr = ptr, .len = len}));
}
void lib_ruby_parser__external__shared_byte_list__drop(LIB_RUBY_PARSER_SharedByteList_BLOB *self_blob)
{
    (void)self_blob;
}
const uint8_t *lib_ruby_parser__external__shared_byte_list__get_raw(const LIB_RUBY_PARSER_SharedByteList_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_SharedByteList *self = (const LIB_RUBY_PARSER_SharedByteList *)self_blob;
    if (self->len == 0)
        return NULL;
    return self->ptr;
}
uint64_t lib_ruby_parser__external__shared_byte_list__get_len(const LIB_RUBY_PARSER_SharedByteList_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_SharedByteList *self = (const LIB_RUBY_PARSER_SharedByteList *)self_blob;
    return self->len;
}

/*
    Lists
*/
#define LIST_IMPL(ITEM, NS, drop)                                                                                                               \
    LIB_RUBY_PARSER_##ITEM##List_BLOB lib_ruby_parser__external__list__##NS##__new()                                                            \
    {                                                                                                                                           \
        return PACK_##ITEM##List(((LIB_RUBY_PARSER_##ITEM##List){.ptr = NULL, .len = 0, .capacity = 0}));                                       \
    }                                                                                                                                           \
    void lib_ruby_parser__external__list__##NS##__drop(LIB_RUBY_PARSER_##ITEM##List_BLOB *self_blob)                                            \
    {                                                                                                                                           \
        LIB_RUBY_PARSER_##ITEM##List *self = (LIB_RUBY_PARSER_##ITEM##List *)self_blob;                                                         \
        for (uint64_t i = 0; i < self->len; i++)                                                                                                \
        {                                                                                                                                       \
            drop(&self->ptr[i]);                                                                                                                \
        }                                                                                                                                       \
        free(self->ptr);                                                                                                                        \
    }                                                                                                                                           \
    LIB_RUBY_PARSER_##ITEM##List_BLOB lib_ruby_parser__external__list__##NS##__with_capacity(uint64_t capacity)                                 \
    {                                                                                                                                           \
        LIB_RUBY_PARSER_##ITEM##List list = {.ptr = malloc(sizeof(LIB_RUBY_PARSER_##ITEM) * capacity), .len = 0, .capacity = capacity};         \
        return PACK_##ITEM##List(list);                                                                                                         \
    }                                                                                                                                           \
    LIB_RUBY_PARSER_##ITEM##List_BLOB lib_ruby_parser__external__list__##NS##__from_raw(LIB_RUBY_PARSER_##ITEM##_BLOB *ptr, uint64_t len)       \
    {                                                                                                                                           \
        if (len > 0)                                                                                                                            \
        {                                                                                                                                       \
            LIB_RUBY_PARSER_##ITEM##List list = {.ptr = (LIB_RUBY_PARSER_##ITEM *)ptr, .len = len, .capacity = len};                            \
            return PACK_##ITEM##List(list);                                                                                                     \
        }                                                                                                                                       \
        else                                                                                                                                    \
        {                                                                                                                                       \
            return lib_ruby_parser__external__list__##NS##__new();                                                                              \
        }                                                                                                                                       \
    }                                                                                                                                           \
    void lib_ruby_parser__external__list__##NS##__push(LIB_RUBY_PARSER_##ITEM##List_BLOB *self_blob, LIB_RUBY_PARSER_##ITEM##_BLOB item_blob)   \
    {                                                                                                                                           \
        LIB_RUBY_PARSER_##ITEM##List *self = (LIB_RUBY_PARSER_##ITEM##List *)self_blob;                                                         \
        LIB_RUBY_PARSER_##ITEM item = UNPACK_##ITEM(item_blob);                                                                                 \
        if (self->len + 1 > self->capacity)                                                                                                     \
        {                                                                                                                                       \
            if (self->capacity == 0)                                                                                                            \
            {                                                                                                                                   \
                self->capacity += 1;                                                                                                            \
            }                                                                                                                                   \
            else                                                                                                                                \
            {                                                                                                                                   \
                self->capacity *= 2;                                                                                                            \
            }                                                                                                                                   \
            LIB_RUBY_PARSER_##ITEM *old_ptr = self->ptr;                                                                                        \
            LIB_RUBY_PARSER_##ITEM *new_ptr = malloc(sizeof(LIB_RUBY_PARSER_##ITEM) * self->capacity);                                          \
            memcpy(new_ptr, old_ptr, sizeof(LIB_RUBY_PARSER_##ITEM) * self->len);                                                               \
            self->ptr = new_ptr;                                                                                                                \
            free(old_ptr);                                                                                                                      \
        }                                                                                                                                       \
        self->ptr[self->len] = item;                                                                                                            \
        self->len++;                                                                                                                            \
    }                                                                                                                                           \
    LIB_RUBY_PARSER_##ITEM##_BLOB lib_ruby_parser__external__list__##NS##__remove(LIB_RUBY_PARSER_##ITEM##List_BLOB *self_blob, uint64_t index) \
    {                                                                                                                                           \
        LIB_RUBY_PARSER_##ITEM##List *self = (LIB_RUBY_PARSER_##ITEM##List *)self_blob;                                                         \
        LIB_RUBY_PARSER_##ITEM item = self->ptr[index];                                                                                         \
        memmove(self->ptr + index, self->ptr + index + 1, sizeof(LIB_RUBY_PARSER_##ITEM) * (self->len - index - 1));                            \
        self->len--;                                                                                                                            \
        return PACK_##ITEM(item);                                                                                                               \
    }                                                                                                                                           \
    void lib_ruby_parser__external__list__##NS##__shrink_to_fit(LIB_RUBY_PARSER_##ITEM##List_BLOB *self_blob)                                   \
    {                                                                                                                                           \
        LIB_RUBY_PARSER_##ITEM##List *self = (LIB_RUBY_PARSER_##ITEM##List *)self_blob;                                                         \
                                                                                                                                                \
        uint64_t new_len = self->len;                                                                                                           \
        uint64_t new_capacity = self->len;                                                                                                      \
                                                                                                                                                \
        LIB_RUBY_PARSER_##ITEM *new_ptr = malloc(sizeof(LIB_RUBY_PARSER_##ITEM) * new_capacity);                                                \
        memcpy(new_ptr, self->ptr, sizeof(LIB_RUBY_PARSER_##ITEM) * new_len);                                                                   \
                                                                                                                                                \
        LIB_RUBY_PARSER_##ITEM *old_ptr = self->ptr;                                                                                            \
        self->ptr = new_ptr;                                                                                                                    \
        self->len = new_len;                                                                                                                    \
        self->capacity = new_capacity;                                                                                                          \
        free(old_ptr);                                                                                                                          \
    }                                                                                                                                           \
    const LIB_RUBY_PARSER_##ITEM##_BLOB *lib_ruby_parser__external__list__##NS##__as_ptr(const LIB_RUBY_PARSER_##ITEM##List_BLOB *self_blob)    \
    {                                                                                                                                           \
        const LIB_RUBY_PARSER_##ITEM##List *self = (const LIB_RUBY_PARSER_##ITEM##List *)self_blob;                                             \
        return (const LIB_RUBY_PARSER_##ITEM##_BLOB *)(self->ptr);                                                                              \
    }                                                                                                                                           \
    LIB_RUBY_PARSER_##ITEM##_BLOB *lib_ruby_parser__external__list__##NS##__into_ptr(LIB_RUBY_PARSER_##ITEM##List_BLOB self_blob)               \
    {                                                                                                                                           \
        LIB_RUBY_PARSER_##ITEM##List self = UNPACK_##ITEM##List(self_blob);                                                                     \
        return (LIB_RUBY_PARSER_##ITEM##_BLOB *)(self.ptr);                                                                                     \
    }                                                                                                                                           \
    uint64_t lib_ruby_parser__external__list__##NS##__get_len(const LIB_RUBY_PARSER_##ITEM##List_BLOB *self_blob)                               \
    {                                                                                                                                           \
        const LIB_RUBY_PARSER_##ITEM##List *self = (const LIB_RUBY_PARSER_##ITEM##List *)self_blob;                                             \
        return self->len;                                                                                                                       \
    }                                                                                                                                           \
    uint64_t lib_ruby_parser__external__list__##NS##__get_capacity(const LIB_RUBY_PARSER_##ITEM##List_BLOB *self_blob)                          \
    {                                                                                                                                           \
        const LIB_RUBY_PARSER_##ITEM##List *self = (const LIB_RUBY_PARSER_##ITEM##List *)self_blob;                                             \
        return self->capacity;                                                                                                                  \
    }

void drop_nothing(void *byte)
{
    (void)byte;
}
LIST_IMPL(Byte, of_bytes, drop_nothing)
LIST_IMPL(Token, of_tokens, LIB_RUBY_PARSER_drop_token)
LIST_IMPL(Node, of_nodes, LIB_RUBY_PARSER_drop_node)
LIST_IMPL(Diagnostic, of_diagnostics, LIB_RUBY_PARSER_drop_diagnostic)
LIST_IMPL(Comment, of_comments, drop_nothing)
LIST_IMPL(MagicComment, of_magic_comments, drop_nothing)
LIST_IMPL(SourceLine, of_source_lines, drop_nothing)

/*
    LIB_RUBY_PARSER_SourceLine
*/
LIB_RUBY_PARSER_SourceLine_BLOB lib_ruby_parser__external__source_line__new(uint64_t start, uint64_t end, bool ends_with_eof)
{
    return PACK_SourceLine(((LIB_RUBY_PARSER_SourceLine){.start = start, .end = end, .ends_with_eof = ends_with_eof}));
}
void lib_ruby_parser__external__source_line__drop(LIB_RUBY_PARSER_SourceLine_BLOB *self_blob)
{
    (void)(self_blob);
}
uint64_t lib_ruby_parser__external__source_line__get_start(const LIB_RUBY_PARSER_SourceLine_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_SourceLine *self = (const LIB_RUBY_PARSER_SourceLine *)self_blob;
    return self->start;
}
uint64_t lib_ruby_parser__external__source_line__get_end(const LIB_RUBY_PARSER_SourceLine_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_SourceLine *self = (const LIB_RUBY_PARSER_SourceLine *)self_blob;
    return self->end;
}
bool lib_ruby_parser__external__source_line__get_ends_with_eof(const LIB_RUBY_PARSER_SourceLine_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_SourceLine *self = (const LIB_RUBY_PARSER_SourceLine *)self_blob;
    return self->ends_with_eof;
}
void lib_ruby_parser__external__source_line__set_start(LIB_RUBY_PARSER_SourceLine_BLOB *self_blob, uint64_t start)
{
    LIB_RUBY_PARSER_SourceLine *self = (LIB_RUBY_PARSER_SourceLine *)self_blob;
    self->start = start;
}
void lib_ruby_parser__external__source_line__set_end(LIB_RUBY_PARSER_SourceLine_BLOB *self_blob, uint64_t end)
{
    LIB_RUBY_PARSER_SourceLine *self = (LIB_RUBY_PARSER_SourceLine *)self_blob;
    self->end = end;
}
void lib_ruby_parser__external__source_line__set_ends_with_eof(LIB_RUBY_PARSER_SourceLine_BLOB *self_blob, bool ends_with_eof)
{
    LIB_RUBY_PARSER_SourceLine *self = (LIB_RUBY_PARSER_SourceLine *)self_blob;
    self->ends_with_eof = ends_with_eof;
}

/*
    LIB_RUBY_PARSER_Bytes
*/
LIB_RUBY_PARSER_Bytes_BLOB lib_ruby_parser__external__bytes__new(LIB_RUBY_PARSER_ByteList_BLOB raw_blob)
{
    return PACK_Bytes(((LIB_RUBY_PARSER_Bytes){.raw = UNPACK_ByteList(raw_blob)}));
}
void lib_ruby_parser__external__bytes__drop(LIB_RUBY_PARSER_Bytes_BLOB *self_blob)
{
    LIB_RUBY_PARSER_Bytes *self = (LIB_RUBY_PARSER_Bytes *)self_blob;
    LIB_RUBY_PARSER_drop_bytes(self);
}
const LIB_RUBY_PARSER_ByteList_BLOB *lib_ruby_parser__external__bytes__get_raw(const LIB_RUBY_PARSER_Bytes_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_Bytes *self = (const LIB_RUBY_PARSER_Bytes *)self_blob;
    return (const LIB_RUBY_PARSER_ByteList_BLOB *)(&(self->raw));
}
void lib_ruby_parser__external__bytes__set_raw(LIB_RUBY_PARSER_Bytes_BLOB *self_blob, LIB_RUBY_PARSER_ByteList_BLOB raw_blob)
{
    LIB_RUBY_PARSER_Bytes *self = (LIB_RUBY_PARSER_Bytes *)self_blob;
    lib_ruby_parser__external__list__of_bytes__drop((LIB_RUBY_PARSER_ByteList_BLOB *)(&(self->raw)));
    self->raw = UNPACK_ByteList(raw_blob);
}
LIB_RUBY_PARSER_ByteList_BLOB lib_ruby_parser__external__bytes__into_raw(LIB_RUBY_PARSER_Bytes_BLOB self_blob)
{
    return PACK_ByteList(UNPACK_Bytes(self_blob).raw);
}
void lib_ruby_parser__external__bytes__push(LIB_RUBY_PARSER_Bytes_BLOB *self_blob, LIB_RUBY_PARSER_Byte_BLOB byte_blob)
{
    LIB_RUBY_PARSER_Bytes *self = (LIB_RUBY_PARSER_Bytes *)self_blob;
    LIB_RUBY_PARSER_ByteList *byte_list = &(self->raw);
    lib_ruby_parser__external__list__of_bytes__push((LIB_RUBY_PARSER_ByteList_BLOB *)byte_list, byte_blob);
}

/*
    LIB_RUBY_PARSER_Token
*/
LIB_RUBY_PARSER_Token_BLOB lib_ruby_parser__external__token__new(
    uint32_t token_type,
    LIB_RUBY_PARSER_Bytes_BLOB token_value_blob,
    LIB_RUBY_PARSER_Loc_BLOB loc_blob,
    uint32_t lex_state_before,
    uint32_t lex_state_after)
{
    return PACK_Token(((LIB_RUBY_PARSER_Token){
        .token_type = token_type,
        .token_value = UNPACK_Bytes(token_value_blob),
        .loc = UNPACK_Loc(loc_blob),
        .lex_state_before = lex_state_before,
        .lex_state_after = lex_state_after}));
}
void lib_ruby_parser__external__token__drop(LIB_RUBY_PARSER_Token_BLOB *self_blob)
{
    LIB_RUBY_PARSER_Token *self = (LIB_RUBY_PARSER_Token *)self_blob;
    LIB_RUBY_PARSER_drop_token(self);
}
uint32_t lib_ruby_parser__external__token__get_token_type(const LIB_RUBY_PARSER_Token_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_Token *self = (const LIB_RUBY_PARSER_Token *)self_blob;
    return self->token_type;
}
const LIB_RUBY_PARSER_Bytes_BLOB *lib_ruby_parser__external__token__get_token_value(const LIB_RUBY_PARSER_Token_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_Token *self = (const LIB_RUBY_PARSER_Token *)self_blob;
    return (LIB_RUBY_PARSER_Bytes_BLOB *)(&(self->token_value));
}
void lib_ruby_parser__external__token__set_token_value(LIB_RUBY_PARSER_Token_BLOB *self_blob, LIB_RUBY_PARSER_Bytes_BLOB token_value_blob)
{
    LIB_RUBY_PARSER_Token *self = (LIB_RUBY_PARSER_Token *)self_blob;
    LIB_RUBY_PARSER_drop_bytes(&(self->token_value));
    self->token_value = UNPACK_Bytes(token_value_blob);
}
LIB_RUBY_PARSER_Bytes_BLOB lib_ruby_parser__external__token__into_token_value(LIB_RUBY_PARSER_Token_BLOB self_blob)
{
    return PACK_Bytes(UNPACK_Token(self_blob).token_value);
}
const LIB_RUBY_PARSER_Loc_BLOB *lib_ruby_parser__external__token__get_loc(const LIB_RUBY_PARSER_Token_BLOB *self_blob)
{
    LIB_RUBY_PARSER_Token *self = (LIB_RUBY_PARSER_Token *)self_blob;
    return (LIB_RUBY_PARSER_Loc_BLOB *)(&(self->loc));
}
uint32_t lib_ruby_parser__external__token__get_lex_state_before(const LIB_RUBY_PARSER_Token_BLOB *self_blob)
{
    LIB_RUBY_PARSER_Token *self = (LIB_RUBY_PARSER_Token *)self_blob;
    return self->lex_state_before;
}
uint32_t lib_ruby_parser__external__token__get_lex_state_after(const LIB_RUBY_PARSER_Token_BLOB *self_blob)
{
    LIB_RUBY_PARSER_Token *self = (LIB_RUBY_PARSER_Token *)self_blob;
    return self->lex_state_after;
}

/*
    LIB_RUBY_PARSER_CommentType
*/
LIB_RUBY_PARSER_CommentType_BLOB lib_ruby_parser__external__comment_type__new_inline()
{
    return PACK_CommentType(LIB_RUBY_PARSER_COMMENT_TYPE_INLINE);
}
LIB_RUBY_PARSER_CommentType_BLOB lib_ruby_parser__external__comment_type__new_document()
{
    return PACK_CommentType(LIB_RUBY_PARSER_COMMENT_TYPE_DOCUMENT);
}
LIB_RUBY_PARSER_CommentType_BLOB lib_ruby_parser__external__comment_type__new_unknown()
{
    return PACK_CommentType(LIB_RUBY_PARSER_COMMENT_TYPE_UNKNOWN);
}
void lib_ruby_parser__external__comment_type__drop(LIB_RUBY_PARSER_CommentType_BLOB *self_blob)
{
    (void)self_blob;
}
bool lib_ruby_parser__external__comment_type__is_inline(const LIB_RUBY_PARSER_CommentType_BLOB *self_blob)
{
    LIB_RUBY_PARSER_CommentType *self = (LIB_RUBY_PARSER_CommentType *)self_blob;
    return *self == LIB_RUBY_PARSER_COMMENT_TYPE_INLINE;
}
bool lib_ruby_parser__external__comment_type__is_document(const LIB_RUBY_PARSER_CommentType_BLOB *self_blob)
{
    LIB_RUBY_PARSER_CommentType *self = (LIB_RUBY_PARSER_CommentType *)self_blob;
    return *self == LIB_RUBY_PARSER_COMMENT_TYPE_DOCUMENT;
}
bool lib_ruby_parser__external__comment_type__is_unknown(const LIB_RUBY_PARSER_CommentType_BLOB *self_blob)
{
    LIB_RUBY_PARSER_CommentType *self = (LIB_RUBY_PARSER_CommentType *)self_blob;
    return *self == LIB_RUBY_PARSER_COMMENT_TYPE_UNKNOWN;
}

/*
    LIB_RUBY_PARSER_Comment
*/
LIB_RUBY_PARSER_Comment_BLOB lib_ruby_parser__external__comment__new(LIB_RUBY_PARSER_Loc_BLOB location_blob, LIB_RUBY_PARSER_CommentType_BLOB kind_blob)
{
    return PACK_Comment(((LIB_RUBY_PARSER_Comment){.location = UNPACK_Loc(location_blob), .kind = UNPACK_CommentType(kind_blob)}));
}
void lib_ruby_parser__external__comment__drop(LIB_RUBY_PARSER_Comment_BLOB *self_blob)
{
    (void)self_blob;
}
const LIB_RUBY_PARSER_Loc_BLOB *lib_ruby_parser__external__comment__get_location(const LIB_RUBY_PARSER_Comment_BLOB *self_blob)
{
    LIB_RUBY_PARSER_Comment *self = (LIB_RUBY_PARSER_Comment *)self_blob;
    return (LIB_RUBY_PARSER_Loc_BLOB *)(&(self->location));
}
const LIB_RUBY_PARSER_CommentType_BLOB *lib_ruby_parser__external__comment__get_kind(const LIB_RUBY_PARSER_Comment_BLOB *self_blob)
{
    LIB_RUBY_PARSER_Comment *self = (LIB_RUBY_PARSER_Comment *)self_blob;
    return (LIB_RUBY_PARSER_CommentType_BLOB *)(&(self->kind));
}

/*
    LIB_RUBY_PARSER_Loc
*/
LIB_RUBY_PARSER_Loc_BLOB lib_ruby_parser__external__loc__new(uint64_t begin, uint64_t end)
{
    return PACK_Loc(((LIB_RUBY_PARSER_Loc){.begin = begin, .end = end}));
}
void lib_ruby_parser__external__loc__drop(LIB_RUBY_PARSER_Loc_BLOB *self_blob)
{
    (void)self_blob;
}
uint64_t lib_ruby_parser__external__loc__get_begin(const LIB_RUBY_PARSER_Loc_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_Loc *self = (const LIB_RUBY_PARSER_Loc *)self_blob;
    return self->begin;
}
uint64_t lib_ruby_parser__external__loc__get_end(const LIB_RUBY_PARSER_Loc_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_Loc *self = (const LIB_RUBY_PARSER_Loc *)self_blob;
    return self->end;
}

/*
    LIB_RUBY_PARSER_MagicCommentKind
*/
LIB_RUBY_PARSER_MagicCommentKind_BLOB lib_ruby_parser__external__magic_comment_kind__new_encoding()
{
    return PACK_MagicCommentKind(LIB_RUBY_PARSER_MAGIC_COMMENT_KIND_ENCODING);
}
LIB_RUBY_PARSER_MagicCommentKind_BLOB lib_ruby_parser__external__magic_comment_kind__new_frozen_string_literal()
{
    return PACK_MagicCommentKind(LIB_RUBY_PARSER_MAGIC_COMMENT_KIND_FROZEN_STRING_LITERAL);
}
LIB_RUBY_PARSER_MagicCommentKind_BLOB lib_ruby_parser__external__magic_comment_kind__new_warn_indent()
{
    return PACK_MagicCommentKind(LIB_RUBY_PARSER_MAGIC_COMMENT_KIND_WARN_INDENT);
}
LIB_RUBY_PARSER_MagicCommentKind_BLOB lib_ruby_parser__external__magic_comment_kind__new_shareable_constant_value()
{
    return PACK_MagicCommentKind(LIB_RUBY_PARSER_MAGIC_COMMENT_KIND_SHAREABLE_CONSTANT_VALUE);
}
void lib_ruby_parser__external__magic_comment_kind__drop(LIB_RUBY_PARSER_MagicCommentKind_BLOB *self_blob)
{
    (void)self_blob;
}
bool lib_ruby_parser__external__magic_comment_kind__is_encoding(const LIB_RUBY_PARSER_MagicCommentKind_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MagicCommentKind *self = (const LIB_RUBY_PARSER_MagicCommentKind *)self_blob;
    return *self == LIB_RUBY_PARSER_MAGIC_COMMENT_KIND_ENCODING;
}
bool lib_ruby_parser__external__magic_comment_kind__is_frozen_string_literal(const LIB_RUBY_PARSER_MagicCommentKind_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MagicCommentKind *self = (const LIB_RUBY_PARSER_MagicCommentKind *)self_blob;
    return *self == LIB_RUBY_PARSER_MAGIC_COMMENT_KIND_FROZEN_STRING_LITERAL;
}
bool lib_ruby_parser__external__magic_comment_kind__is_warn_indent(const LIB_RUBY_PARSER_MagicCommentKind_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MagicCommentKind *self = (const LIB_RUBY_PARSER_MagicCommentKind *)self_blob;
    return *self == LIB_RUBY_PARSER_MAGIC_COMMENT_KIND_WARN_INDENT;
}
bool lib_ruby_parser__external__magic_comment_kind__is_shareable_constant_value(const LIB_RUBY_PARSER_MagicCommentKind_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MagicCommentKind *self = (const LIB_RUBY_PARSER_MagicCommentKind *)self_blob;
    return *self == LIB_RUBY_PARSER_MAGIC_COMMENT_KIND_SHAREABLE_CONSTANT_VALUE;
}

/*
    LIB_RUBY_PARSER_MagicComment
*/

LIB_RUBY_PARSER_MagicComment_BLOB lib_ruby_parser__external__magic_comment__new(
    LIB_RUBY_PARSER_MagicCommentKind_BLOB kind_blob,
    LIB_RUBY_PARSER_Loc_BLOB key_l_blob,
    LIB_RUBY_PARSER_Loc_BLOB value_l_blob)
{
    return PACK_MagicComment(((LIB_RUBY_PARSER_MagicComment){
        .kind = UNPACK_MagicCommentKind(kind_blob),
        .key_l = UNPACK_Loc(key_l_blob),
        .value_l = UNPACK_Loc(value_l_blob)}));
}
void lib_ruby_parser__external__magic_comment__drop(LIB_RUBY_PARSER_MagicComment_BLOB *self_blob)
{
    (void)self_blob;
}
const LIB_RUBY_PARSER_MagicCommentKind_BLOB *lib_ruby_parser__external__magic_comment__get_kind(const LIB_RUBY_PARSER_MagicComment_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MagicComment *self = (const LIB_RUBY_PARSER_MagicComment *)self_blob;
    return (const LIB_RUBY_PARSER_MagicCommentKind_BLOB *)(&(self->kind));
}
const LIB_RUBY_PARSER_Loc_BLOB *lib_ruby_parser__external__magic_comment__get_key_l(const LIB_RUBY_PARSER_MagicComment_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MagicComment *self = (const LIB_RUBY_PARSER_MagicComment *)self_blob;
    return (const LIB_RUBY_PARSER_Loc_BLOB *)(&(self->key_l));
}
const LIB_RUBY_PARSER_Loc_BLOB *lib_ruby_parser__external__magic_comment__get_value_l(const LIB_RUBY_PARSER_MagicComment_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_MagicComment *self = (const LIB_RUBY_PARSER_MagicComment *)self_blob;
    return (const LIB_RUBY_PARSER_Loc_BLOB *)(&(self->value_l));
}

/*
    LIB_RUBY_PARSER_ErrorLevel
*/
LIB_RUBY_PARSER_ErrorLevel_BLOB lib_ruby_parser__external__error_level__new_warning()
{
    return PACK_ErrorLevel(LIB_RUBY_PARSER_ERROR_LEVEL_WARNING);
}
LIB_RUBY_PARSER_ErrorLevel_BLOB lib_ruby_parser__external__error_level__new_error()
{
    return PACK_ErrorLevel(LIB_RUBY_PARSER_ERROR_LEVEL_ERROR);
}
void lib_ruby_parser__external__error_level__drop(LIB_RUBY_PARSER_ErrorLevel_BLOB *self_blob)
{
    (void)self_blob;
}
bool lib_ruby_parser__external__error_level__is_warning(const LIB_RUBY_PARSER_ErrorLevel_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_ErrorLevel *self = (const LIB_RUBY_PARSER_ErrorLevel *)self_blob;
    return *self == LIB_RUBY_PARSER_ERROR_LEVEL_WARNING;
}
bool lib_ruby_parser__external__error_level__is_error(const LIB_RUBY_PARSER_ErrorLevel_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_ErrorLevel *self = (const LIB_RUBY_PARSER_ErrorLevel *)self_blob;
    return *self == LIB_RUBY_PARSER_ERROR_LEVEL_ERROR;
}

/*
    LIB_RUBY_PARSER_Diagnostic
*/
LIB_RUBY_PARSER_Diagnostic_BLOB lib_ruby_parser__external__diagnostic__new(
    LIB_RUBY_PARSER_ErrorLevel_BLOB level_blob,
    LIB_RUBY_PARSER_DiagnosticMessage_BLOB message_blob,
    LIB_RUBY_PARSER_Loc_BLOB loc_blob)
{
    return PACK_Diagnostic(((LIB_RUBY_PARSER_Diagnostic){
        .level = UNPACK_ErrorLevel(level_blob),
        .message = UNPACK_DiagnosticMessage(message_blob),
        .loc = UNPACK_Loc(loc_blob)}));
}
void lib_ruby_parser__external__diagnostic__drop(LIB_RUBY_PARSER_Diagnostic_BLOB *self_blob)
{
    LIB_RUBY_PARSER_Diagnostic *self = (LIB_RUBY_PARSER_Diagnostic *)self_blob;
    LIB_RUBY_PARSER_drop_diagnostic(self);
}
const LIB_RUBY_PARSER_ErrorLevel_BLOB *lib_ruby_parser__external__diagnostic__get_level(const LIB_RUBY_PARSER_Diagnostic_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_Diagnostic *self = (const LIB_RUBY_PARSER_Diagnostic *)self_blob;
    return (const LIB_RUBY_PARSER_ErrorLevel_BLOB *)(&(self->level));
}
const LIB_RUBY_PARSER_DiagnosticMessage_BLOB *lib_ruby_parser__external__diagnostic__get_message(const LIB_RUBY_PARSER_Diagnostic_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_Diagnostic *self = (const LIB_RUBY_PARSER_Diagnostic *)self_blob;
    return (const LIB_RUBY_PARSER_DiagnosticMessage_BLOB *)(&(self->message));
}
const LIB_RUBY_PARSER_Loc_BLOB *lib_ruby_parser__external__diagnostic__get_loc(const LIB_RUBY_PARSER_Diagnostic_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_Diagnostic *self = (const LIB_RUBY_PARSER_Diagnostic *)self_blob;
    return (const LIB_RUBY_PARSER_Loc_BLOB *)(&(self->loc));
}

/*
    LIB_RUBY_PARSER_InputError
*/

LIB_RUBY_PARSER_InputError_BLOB lib_ruby_parser__external__input_error__new_unsupported_encoding(LIB_RUBY_PARSER_StringPtr_BLOB err_blob)
{
    return PACK_InputError(((LIB_RUBY_PARSER_InputError){
        .tag = LIB_RUBY_PARSER_INPUT_ERROR_UNSUPPORTED_ENCODING,
        .as = {.unsupported_encoding = UNPACK_StringPtr(err_blob)}}));
}
LIB_RUBY_PARSER_InputError_BLOB lib_ruby_parser__external__input_error__new_decoding_error(LIB_RUBY_PARSER_StringPtr_BLOB err_blob)
{
    return PACK_InputError(((LIB_RUBY_PARSER_InputError){
        .tag = LIB_RUBY_PARSER_INPUT_ERROR_DECODING_ERROR,
        .as = {.decoding_error = UNPACK_StringPtr(err_blob)}}));
}
void lib_ruby_parser__external__input_error__drop(LIB_RUBY_PARSER_InputError_BLOB *self_blob)
{
    LIB_RUBY_PARSER_InputError *self = (LIB_RUBY_PARSER_InputError *)self_blob;
    LIB_RUBY_PARSER_drop_input_error(self);
}
bool lib_ruby_parser__external__input_error__is_unsupported_encoding(const LIB_RUBY_PARSER_InputError_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_InputError *self = (const LIB_RUBY_PARSER_InputError *)self_blob;
    return self->tag == LIB_RUBY_PARSER_INPUT_ERROR_UNSUPPORTED_ENCODING;
}
bool lib_ruby_parser__external__input_error__is_decoding_error(const LIB_RUBY_PARSER_InputError_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_InputError *self = (const LIB_RUBY_PARSER_InputError *)self_blob;
    return self->tag == LIB_RUBY_PARSER_INPUT_ERROR_DECODING_ERROR;
}
const LIB_RUBY_PARSER_StringPtr_BLOB *lib_ruby_parser__external__input_error__get_unsupported_encoding(const LIB_RUBY_PARSER_InputError_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_InputError *self = (const LIB_RUBY_PARSER_InputError *)self_blob;
    if (self->tag == LIB_RUBY_PARSER_INPUT_ERROR_UNSUPPORTED_ENCODING)
        return (const LIB_RUBY_PARSER_StringPtr_BLOB *)(&(self->as.unsupported_encoding));
    return NULL;
}
const LIB_RUBY_PARSER_StringPtr_BLOB *lib_ruby_parser__external__input_error__get_decoding_error(const LIB_RUBY_PARSER_InputError_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_InputError *self = (const LIB_RUBY_PARSER_InputError *)self_blob;
    if (self->tag == LIB_RUBY_PARSER_INPUT_ERROR_DECODING_ERROR)
        return (const LIB_RUBY_PARSER_StringPtr_BLOB *)(&(self->as.decoding_error));
    return NULL;
}

/*
    LIB_RUBY_PARSER_DecoderResult
*/
LIB_RUBY_PARSER_DecoderResult_BLOB lib_ruby_parser__external__decoder_result__new_ok(LIB_RUBY_PARSER_ByteList_BLOB byte_list_blob)
{
    return PACK_DecoderResult(((LIB_RUBY_PARSER_DecoderResult){
        .tag = LIB_RUBY_PARSER_DECODER_RESULT_OK,
        .as = {.ok = UNPACK_ByteList(byte_list_blob)}}));
}
LIB_RUBY_PARSER_DecoderResult_BLOB lib_ruby_parser__external__decoder_result__new_err(LIB_RUBY_PARSER_InputError_BLOB input_error_blob)
{
    return PACK_DecoderResult(((LIB_RUBY_PARSER_DecoderResult){
        .tag = LIB_RUBY_PARSER_DECODER_RESULT_ERR,
        .as = {.err = UNPACK_InputError(input_error_blob)}}));
}
void lib_ruby_parser__external__decoder_result__drop(LIB_RUBY_PARSER_DecoderResult_BLOB *self_blob)
{
    LIB_RUBY_PARSER_DecoderResult *self = (LIB_RUBY_PARSER_DecoderResult *)self_blob;
    LIB_RUBY_PARSER_drop_decoder_result(self);
}
bool lib_ruby_parser__external__decoder_result_is_ok(const LIB_RUBY_PARSER_DecoderResult_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_DecoderResult *self = (const LIB_RUBY_PARSER_DecoderResult *)self_blob;
    return self->tag == LIB_RUBY_PARSER_DECODER_RESULT_OK;
}
bool lib_ruby_parser__external__decoder_result_is_err(const LIB_RUBY_PARSER_DecoderResult_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_DecoderResult *self = (const LIB_RUBY_PARSER_DecoderResult *)self_blob;
    return self->tag == LIB_RUBY_PARSER_DECODER_RESULT_ERR;
}
LIB_RUBY_PARSER_ByteList_BLOB lib_ruby_parser__external__decoder_result_into_ok(LIB_RUBY_PARSER_DecoderResult_BLOB self_blob)
{
    return PACK_ByteList(UNPACK_DecoderResult(self_blob).as.ok);
}
LIB_RUBY_PARSER_InputError_BLOB lib_ruby_parser__external__decoder_result_into_err(LIB_RUBY_PARSER_DecoderResult_BLOB self_blob)
{
    return PACK_InputError(UNPACK_DecoderResult(self_blob).as.err);
}
const LIB_RUBY_PARSER_ByteList_BLOB *lib_ruby_parser__external__decoder_result_as_ok(const LIB_RUBY_PARSER_DecoderResult_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_DecoderResult *self = (const LIB_RUBY_PARSER_DecoderResult *)self_blob;
    return (const LIB_RUBY_PARSER_ByteList_BLOB *)(&(self->as.ok));
}
const LIB_RUBY_PARSER_InputError_BLOB *lib_ruby_parser__external__decoder_result_as_err(const LIB_RUBY_PARSER_DecoderResult_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_DecoderResult *self = (const LIB_RUBY_PARSER_DecoderResult *)self_blob;
    return (const LIB_RUBY_PARSER_InputError_BLOB *)(&(self->as.err));
}

/*
    LIB_RUBY_PARSER_Decoder
*/
LIB_RUBY_PARSER_DecoderResult_BLOB lib_ruby_parser__external__decoder__call(
    LIB_RUBY_PARSER_Decoder_BLOB *self_blob,
    LIB_RUBY_PARSER_StringPtr_BLOB encoding_blob,
    LIB_RUBY_PARSER_ByteList_BLOB input_blob)
{
    // cleanup unused values that we own
    LIB_RUBY_PARSER_StringPtr encoding = UNPACK_StringPtr(encoding_blob);
    LIB_RUBY_PARSER_drop_string_ptr(&encoding);
    LIB_RUBY_PARSER_ByteList input = UNPACK_ByteList(input_blob);
    LIB_RUBY_PARSER_drop_byte_list(&input);

    // call dummy decoder
    LIB_RUBY_PARSER_Decoder *self = (LIB_RUBY_PARSER_Decoder *)self_blob;
    return PACK_DecoderResult(self->f());
}
void lib_ruby_parser__external__decoder_drop(LIB_RUBY_PARSER_Decoder_BLOB *self_blob)
{
    (void)self_blob;
}
LIB_RUBY_PARSER_Decoder_BLOB lib_ruby_parser__external__decoder__new(LIB_RUBY_PARSER_dummy_decoder_t f)
{
    return PACK_Decoder(((LIB_RUBY_PARSER_Decoder){.f = f}));
}

/*
    LIB_RUBY_PARSER_RewriteAction
*/
void lib_ruby_parser__external__rewrite_action__drop(LIB_RUBY_PARSER_RewriteAction_BLOB *self_blob)
{
    (void)self_blob;
}
bool lib_ruby_parser__external__rewrite_action__is_drop(const LIB_RUBY_PARSER_RewriteAction_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_RewriteAction *self = (const LIB_RUBY_PARSER_RewriteAction *)self_blob;
    return *self == LIB_RUBY_PARSER_REWRITE_ACTION_DROP;
}
bool lib_ruby_parser__external__rewrite_action__is_keep(const LIB_RUBY_PARSER_RewriteAction_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_RewriteAction *self = (const LIB_RUBY_PARSER_RewriteAction *)self_blob;
    return *self == LIB_RUBY_PARSER_REWRITE_ACTION_KEEP;
}

/*
    LIB_RUBY_PARSER_LexStateAction
*/
void lib_ruby_parser__external__lex_state_action__drop(LIB_RUBY_PARSER_LexStateAction_BLOB *self_blob)
{
    (void)self_blob;
}
bool lib_ruby_parser__external__lex_state_action__is_set(const LIB_RUBY_PARSER_LexStateAction_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_LexStateAction *self = (const LIB_RUBY_PARSER_LexStateAction *)self_blob;
    return self->tag == LIB_RUBY_PARSER_LEX_STATE_SET;
}
bool lib_ruby_parser__external__lex_state_action__is_keep(const LIB_RUBY_PARSER_LexStateAction_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_LexStateAction *self = (const LIB_RUBY_PARSER_LexStateAction *)self_blob;
    return self->tag == LIB_RUBY_PARSER_LEX_STATE_KEEP;
}
int32_t lib_ruby_parser__external__lex_state_action__get_next_state(const LIB_RUBY_PARSER_LexStateAction_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_LexStateAction *self = (const LIB_RUBY_PARSER_LexStateAction *)self_blob;
    return self->as.set;
}

/*
    LIB_RUBY_PARSER_TokenRewriterResult
*/
void lib_ruby_parser__external__token_rewriter_result__drop(LIB_RUBY_PARSER_TokenRewriterResult_BLOB *self_blob)
{
    LIB_RUBY_PARSER_TokenRewriterResult *self = (LIB_RUBY_PARSER_TokenRewriterResult *)self_blob;
    LIB_RUBY_PARSER_drop_token_rewriter_result(self);
}
InternalTokenRewriterResult lib_ruby_parser__external__token_rewriter_result__into_internal(LIB_RUBY_PARSER_TokenRewriterResult_BLOB self_blob)
{
    LIB_RUBY_PARSER_TokenRewriterResult self = UNPACK_TokenRewriterResult(self_blob);
    return ((InternalTokenRewriterResult){
        .token_action = PACK_RewriteAction(self.token_action),
        .lex_state_action = PACK_LexStateAction(self.lex_state_action),
        .rewritten_token = PACK_Ptr(self.rewritten_token)});
}

/*
    LIB_RUBY_PARSER_TokenRewriter
*/
void lib_ruby_parser__external__token_rewriter__drop(LIB_RUBY_PARSER_TokenRewriter_BLOB *self_blob)
{
    (void)self_blob;
}
LIB_RUBY_PARSER_TokenRewriterResult_BLOB lib_ruby_parser__external__token_rewriter__call(
    LIB_RUBY_PARSER_TokenRewriter_BLOB *self_blob,
    LIB_RUBY_PARSER_Ptr_BLOB token_blob,
    LIB_RUBY_PARSER_SharedByteList_BLOB input_blob)
{
    LIB_RUBY_PARSER_Ptr token = UNPACK_Ptr(token_blob);
    (void)input_blob;

    // call dummy token_rewriter
    LIB_RUBY_PARSER_TokenRewriter *self = (LIB_RUBY_PARSER_TokenRewriter *)self_blob;
    LIB_RUBY_PARSER_TokenRewriterResult result = self->rewrite_f(token, self->build_new_token_f);
    return PACK_TokenRewriterResult(result);
}
// Test APIs
LIB_RUBY_PARSER_TokenRewriter_BLOB lib_ruby_parser__external__token_rewriter__new_keep(
    LIB_RUBY_PARSER_build_new_token_t build_new_token_f)
{
    return PACK_TokenRewriter(__keep_token_rewriter(build_new_token_f));
}
LIB_RUBY_PARSER_TokenRewriter_BLOB lib_ruby_parser__external__token_rewriter__new_drop(
    LIB_RUBY_PARSER_build_new_token_t build_new_token_f)
{
    return PACK_TokenRewriter(__drop_token_rewriter(build_new_token_f));
}
LIB_RUBY_PARSER_TokenRewriter_BLOB lib_ruby_parser__external__token_rewriter__new_rewrite(
    LIB_RUBY_PARSER_build_new_token_t build_new_token_f)
{
    return PACK_TokenRewriter(__rewriter_token_rewriter(build_new_token_f));
}

/*
    LIB_RUBY_PARSER_ParserOptions
*/
LIB_RUBY_PARSER_ParserOptions_BLOB lib_ruby_parser__external__parser_options__new(
    LIB_RUBY_PARSER_StringPtr_BLOB buffer_name_blob,
    uint8_t debug,
    LIB_RUBY_PARSER_MaybeDecoder_BLOB decoder_blob,
    LIB_RUBY_PARSER_MaybeTokenRewriter_BLOB token_rewriter_blob,
    bool record_tokens)
{
    return PACK_ParserOptions(((LIB_RUBY_PARSER_ParserOptions){
        .buffer_name = UNPACK_StringPtr(buffer_name_blob),
        .debug = debug,
        .decoder = UNPACK_MaybeDecoder(decoder_blob),
        .token_rewriter = UNPACK_MaybeTokenRewriter(token_rewriter_blob),
        .record_tokens = record_tokens}));
}
void lib_ruby_parser__external__parser_options__drop(LIB_RUBY_PARSER_ParserOptions_BLOB *self_blob)
{
    LIB_RUBY_PARSER_ParserOptions *options = (LIB_RUBY_PARSER_ParserOptions *)self_blob;
    LIB_RUBY_PARSER_drop_string_ptr(&(options->buffer_name));
}
InternalParserOptions lib_ruby_parser__external__parser_options__into_internal(LIB_RUBY_PARSER_ParserOptions_BLOB self_blob)
{
    LIB_RUBY_PARSER_ParserOptions self = UNPACK_ParserOptions(self_blob);
    return ((InternalParserOptions){
        .buffer_name = PACK_StringPtr(self.buffer_name),
        .debug = self.debug,
        .decoder = PACK_MaybeDecoder(self.decoder),
        .token_rewriter = PACK_MaybeTokenRewriter(self.token_rewriter),
        .record_tokens = self.record_tokens});
}
const LIB_RUBY_PARSER_StringPtr_BLOB *lib_ruby_parser__external__parser_options__get_buffer_name(const LIB_RUBY_PARSER_ParserOptions_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_ParserOptions *self = (const LIB_RUBY_PARSER_ParserOptions *)self_blob;
    return (const LIB_RUBY_PARSER_StringPtr_BLOB *)(&(self->buffer_name));
}
uint8_t lib_ruby_parser__external__parser_options__get_debug(const LIB_RUBY_PARSER_ParserOptions_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_ParserOptions *self = (const LIB_RUBY_PARSER_ParserOptions *)self_blob;
    return self->debug;
}
const LIB_RUBY_PARSER_MaybeDecoder_BLOB *lib_ruby_parser__external__parser_options__get_decoder(const LIB_RUBY_PARSER_ParserOptions_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_ParserOptions *self = (const LIB_RUBY_PARSER_ParserOptions *)self_blob;
    return (const LIB_RUBY_PARSER_MaybeDecoder_BLOB *)(&(self->decoder));
}
const LIB_RUBY_PARSER_MaybeTokenRewriter_BLOB *lib_ruby_parser__external__parser_options__get_token_rewriter(const LIB_RUBY_PARSER_ParserOptions_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_ParserOptions *self = (const LIB_RUBY_PARSER_ParserOptions *)self_blob;
    return (const LIB_RUBY_PARSER_MaybeTokenRewriter_BLOB *)(&(self->token_rewriter));
}
bool lib_ruby_parser__external__parser_options__get_record_tokens(const LIB_RUBY_PARSER_ParserOptions_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_ParserOptions *self = (const LIB_RUBY_PARSER_ParserOptions *)self_blob;
    return self->record_tokens;
}

// LIB_RUBY_PARSER_DecodedInput
LIB_RUBY_PARSER_DecodedInput_BLOB lib_ruby_parser__external__decoded_input__new(
    LIB_RUBY_PARSER_StringPtr_BLOB name_blob,
    LIB_RUBY_PARSER_SourceLineList_BLOB lines_blob,
    LIB_RUBY_PARSER_ByteList_BLOB bytes_blob)
{
    return PACK_DecodedInput(((LIB_RUBY_PARSER_DecodedInput){
        .name = UNPACK_StringPtr(name_blob),
        .lines = UNPACK_SourceLineList(lines_blob),
        .bytes = UNPACK_ByteList(bytes_blob)}));
}
void lib_ruby_parser__external__decoded_input__drop(LIB_RUBY_PARSER_DecodedInput_BLOB *self_blob)
{
    LIB_RUBY_PARSER_DecodedInput *self = (LIB_RUBY_PARSER_DecodedInput *)self_blob;
    LIB_RUBY_PARSER_drop_decoded_input(self);
}
const LIB_RUBY_PARSER_StringPtr_BLOB *lib_ruby_parser__external__decoded_input__get_name(const LIB_RUBY_PARSER_DecodedInput_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_DecodedInput *self = (const LIB_RUBY_PARSER_DecodedInput *)self_blob;
    return (const LIB_RUBY_PARSER_StringPtr_BLOB *)(&(self->name));
}
const LIB_RUBY_PARSER_SourceLineList_BLOB *lib_ruby_parser__external__decoded_input__get_lines(const LIB_RUBY_PARSER_DecodedInput_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_DecodedInput *self = (const LIB_RUBY_PARSER_DecodedInput *)self_blob;
    return (const LIB_RUBY_PARSER_SourceLineList_BLOB *)(&(self->lines));
}
const LIB_RUBY_PARSER_ByteList_BLOB *lib_ruby_parser__external__decoded_input__get_bytes(const LIB_RUBY_PARSER_DecodedInput_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_DecodedInput *self = (const LIB_RUBY_PARSER_DecodedInput *)self_blob;
    return (const LIB_RUBY_PARSER_ByteList_BLOB *)(&(self->bytes));
}
void lib_ruby_parser__external__decoded_input__set_name(LIB_RUBY_PARSER_DecodedInput_BLOB *self_blob, LIB_RUBY_PARSER_StringPtr_BLOB name)
{
    LIB_RUBY_PARSER_DecodedInput *self = (LIB_RUBY_PARSER_DecodedInput *)self_blob;
    LIB_RUBY_PARSER_drop_string_ptr(&(self->name));
    self->name = UNPACK_StringPtr(name);
}
void lib_ruby_parser__external__decoded_input__set_lines(LIB_RUBY_PARSER_DecodedInput_BLOB *self_blob, LIB_RUBY_PARSER_SourceLineList_BLOB lines)
{
    LIB_RUBY_PARSER_DecodedInput *self = (LIB_RUBY_PARSER_DecodedInput *)self_blob;
    LIB_RUBY_PARSER_drop_source_line_list(&(self->lines));
    self->lines = UNPACK_SourceLineList(lines);
}
void lib_ruby_parser__external__decoded_input__set_bytes(LIB_RUBY_PARSER_DecodedInput_BLOB *self_blob, LIB_RUBY_PARSER_ByteList_BLOB bytes)
{
    LIB_RUBY_PARSER_DecodedInput *self = (LIB_RUBY_PARSER_DecodedInput *)self_blob;
    LIB_RUBY_PARSER_drop_byte_list(&(self->bytes));
    self->bytes = UNPACK_ByteList(bytes);
}
LIB_RUBY_PARSER_ByteList_BLOB lib_ruby_parser__external__decoded_input__into_bytes(LIB_RUBY_PARSER_DecodedInput_BLOB self_blob)
{
    LIB_RUBY_PARSER_DecodedInput self = UNPACK_DecodedInput(self_blob);
    LIB_RUBY_PARSER_drop_string_ptr(&(self.name));
    LIB_RUBY_PARSER_drop_source_line_list(&(self.lines));
    return PACK_ByteList(self.bytes);
}
LIB_RUBY_PARSER_ByteList_BLOB lib_ruby_parser__external__decoded_input__take_bytes(LIB_RUBY_PARSER_DecodedInput_BLOB *self_blob)
{
    LIB_RUBY_PARSER_DecodedInput *self = (LIB_RUBY_PARSER_DecodedInput *)self_blob;
    LIB_RUBY_PARSER_ByteList bytes = self->bytes;
    LIB_RUBY_PARSER_ByteList empty = {.ptr = NULL, .len = 0, .capacity = 0};
    self->bytes = empty;
    return PACK_ByteList(bytes);
}

/*
    LIB_RUBY_PARSER_ParserResult
*/
LIB_RUBY_PARSER_ParserResult_BLOB lib_ruby_parser__external__parser_result__new(
    LIB_RUBY_PARSER_MaybePtr_BLOB ast_blob,
    LIB_RUBY_PARSER_TokenList_BLOB tokens_blob,
    LIB_RUBY_PARSER_DiagnosticList_BLOB diagnostics_blob,
    LIB_RUBY_PARSER_CommentList_BLOB comments_blob,
    LIB_RUBY_PARSER_MagicCommentList_BLOB magic_comments_blob,
    LIB_RUBY_PARSER_DecodedInput_BLOB input_blob)
{
    return PACK_ParserResult(((LIB_RUBY_PARSER_ParserResult){
        .ast = UNPACK_MaybePtr(ast_blob),
        .tokens = UNPACK_TokenList(tokens_blob),
        .diagnostics = UNPACK_DiagnosticList(diagnostics_blob),
        .comments = UNPACK_CommentList(comments_blob),
        .magic_comments = UNPACK_MagicCommentList(magic_comments_blob),
        .input = UNPACK_DecodedInput(input_blob)}));
}
void lib_ruby_parser__external__parser_result__drop(LIB_RUBY_PARSER_ParserResult_BLOB *self_blob)
{
    LIB_RUBY_PARSER_ParserResult *self = (LIB_RUBY_PARSER_ParserResult *)self_blob;
    LIB_RUBY_PARSER_drop_parser_result(self);
}
const LIB_RUBY_PARSER_MaybePtr_BLOB *lib_ruby_parser__external__parser_result__get_ast(const LIB_RUBY_PARSER_ParserResult_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_ParserResult *self = (const LIB_RUBY_PARSER_ParserResult *)self_blob;
    return (const LIB_RUBY_PARSER_MaybePtr_BLOB *)(&(self->ast));
}
const LIB_RUBY_PARSER_TokenList_BLOB *lib_ruby_parser__external__parser_result__get_tokens(const LIB_RUBY_PARSER_ParserResult_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_ParserResult *self = (const LIB_RUBY_PARSER_ParserResult *)self_blob;
    return (const LIB_RUBY_PARSER_TokenList_BLOB *)(&(self->tokens));
}
const LIB_RUBY_PARSER_DiagnosticList_BLOB *lib_ruby_parser__external__parser_result__get_diagnostics(const LIB_RUBY_PARSER_ParserResult_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_ParserResult *self = (const LIB_RUBY_PARSER_ParserResult *)self_blob;
    return (const LIB_RUBY_PARSER_DiagnosticList_BLOB *)(&(self->diagnostics));
}
const LIB_RUBY_PARSER_CommentList_BLOB *lib_ruby_parser__external__parser_result__get_comments(const LIB_RUBY_PARSER_ParserResult_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_ParserResult *self = (const LIB_RUBY_PARSER_ParserResult *)self_blob;
    return (const LIB_RUBY_PARSER_CommentList_BLOB *)(&(self->comments));
}
const LIB_RUBY_PARSER_MagicCommentList_BLOB *lib_ruby_parser__external__parser_result__get_magic_comments(const LIB_RUBY_PARSER_ParserResult_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_ParserResult *self = (const LIB_RUBY_PARSER_ParserResult *)self_blob;
    return (const LIB_RUBY_PARSER_MagicCommentList_BLOB *)(&(self->magic_comments));
}
const LIB_RUBY_PARSER_DecodedInput_BLOB *lib_ruby_parser__external__parser_result__get_input(const LIB_RUBY_PARSER_ParserResult_BLOB *self_blob)
{
    const LIB_RUBY_PARSER_ParserResult *self = (const LIB_RUBY_PARSER_ParserResult *)self_blob;
    return (const LIB_RUBY_PARSER_DecodedInput_BLOB *)(&(self->input));
}
