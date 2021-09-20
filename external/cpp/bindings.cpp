#include "bindings.hpp"
#include <iostream>

using namespace lib_ruby_parser;

extern "C"
{
    /*
        Ptr
    */
    Ptr_BLOB lib_ruby_parser__external__ptr__new(void *raw)
    {
        return PACK_Ptr(std::unique_ptr<int>((int *)raw));
    }
    void lib_ruby_parser__external__ptr__of_node__drop(Ptr_BLOB *self_blob)
    {
        Node *self = (Node *)(((Ptr *)self_blob)->release());
        delete self;
    }
    void lib_ruby_parser__external__ptr__of_token__drop(Ptr_BLOB *self_blob)
    {
        Token *self = (Token *)(((Ptr *)self_blob)->release());
        delete self;
    }
    const void *lib_ruby_parser__external__ptr__get_raw(const Ptr_BLOB *self_blob)
    {
        const Ptr *self = (const Ptr *)self_blob;
        return (const void *)(self->get());
    }
    void *lib_ruby_parser__external__ptr__into_raw(Ptr_BLOB self_blob)
    {
        Ptr self = std::move(UNPACK_Ptr(self_blob));
        return (void *)(self.release());
    }

    /*
        MaybeLoc
    */
    MaybeLoc_BLOB lib_ruby_parser__external__maybe__loc__new_some(Loc_BLOB loc_blob)
    {
        return PACK_MaybeLoc(MaybeLoc(UNPACK_Loc(loc_blob)));
    }
    MaybeLoc_BLOB lib_ruby_parser__external__maybe__loc__new_none()
    {
        return PACK_MaybeLoc(MaybeLoc());
    }
    void lib_ruby_parser__external__maybe__loc__drop(MaybeLoc_BLOB *self_blob)
    {
        MaybeLoc *self = (MaybeLoc *)self_blob;
        self->~optional();
    }
    bool lib_ruby_parser__external__maybe__loc__is_some(const MaybeLoc_BLOB *self_blob)
    {
        const MaybeLoc *self = (const MaybeLoc *)self_blob;
        return self->has_value();
    }
    bool lib_ruby_parser__external__maybe__loc__is_none(const MaybeLoc_BLOB *self_blob)
    {
        const MaybeLoc *self = (const MaybeLoc *)self_blob;
        return !(self->has_value());
    }
    const Loc_BLOB *lib_ruby_parser__external__maybe__loc__as_value(const MaybeLoc_BLOB *self_blob)
    {
        MaybeLoc *self = (MaybeLoc *)self_blob;
        if (!self->has_value())
        {
            return nullptr;
        }
        Loc *loc = &(*(*self));
        return (Loc_BLOB *)loc;
    }
    Loc_BLOB lib_ruby_parser__external__maybe__loc__into_value(MaybeLoc_BLOB self_blob)
    {
        return PACK_Loc(std::move(UNPACK_MaybeLoc(self_blob).value()));
    }

    /*
        MaybePtr
    */
    MaybePtr_BLOB lib_ruby_parser__external__maybe__ptr__new_some(Ptr_BLOB value)
    {
        return PACK_MaybePtr(UNPACK_Ptr(value));
    }
    MaybePtr_BLOB lib_ruby_parser__external__maybe__ptr__new_none()
    {
        return PACK_MaybePtr(nullptr);
    }
    void lib_ruby_parser__external__maybe__ptr__of_node__drop(MaybePtr_BLOB *self_blob)
    {
        Node *self = (Node *)(((MaybePtr *)self_blob)->release());
        if (self != nullptr)
        {
            delete self;
        }
    }
    void lib_ruby_parser__external__maybe__ptr__of_token__drop(MaybePtr_BLOB *self_blob)
    {
        Token *self = (Token *)(((MaybePtr *)self_blob)->release());
        if (self != nullptr)
        {
            delete self;
        }
    }
    bool lib_ruby_parser__external__maybe__ptr__is_some(const MaybePtr_BLOB *self_blob)
    {
        const MaybePtr *self = (const MaybePtr *)self_blob;
        return self->get() != nullptr;
    }
    bool lib_ruby_parser__external__maybe__ptr__is_none(const MaybePtr_BLOB *self_blob)
    {
        const MaybePtr *self = (const MaybePtr *)self_blob;
        return self->get() == nullptr;
    }
    const Ptr_BLOB *lib_ruby_parser__external__maybe__ptr__as_value(const MaybePtr_BLOB *self_blob)
    {
        const MaybePtr *self = (const MaybePtr *)self_blob;
        return (const Ptr_BLOB *)self;
    }
    Ptr_BLOB lib_ruby_parser__external__maybe__ptr__into_value(MaybePtr_BLOB self_blob)
    {
        MaybePtr self = std::move(UNPACK_MaybePtr(self_blob));
        return PACK_Ptr(std::move(self));
    }

    /*
        MaybeStringPtr
    */
    MaybeStringPtr_BLOB lib_ruby_parser__external__maybe__string_ptr__new_some(StringPtr_BLOB value)
    {
        return PACK_MaybeStringPtr(UNPACK_StringPtr(value));
    }
    MaybeStringPtr_BLOB lib_ruby_parser__external__maybe__string_ptr__new_none()
    {
        return PACK_MaybeStringPtr(std::unique_ptr<std::string>(nullptr));
    }
    void lib_ruby_parser__external__maybe__string_ptr__drop(MaybeStringPtr_BLOB *self_blob)
    {
        MaybeStringPtr *self = (MaybeStringPtr *)self_blob;
        self->~unique_ptr();
    }
    bool lib_ruby_parser__external__maybe__string_ptr__is_some(const MaybeStringPtr_BLOB *self_blob)
    {
        const MaybeStringPtr *self = (const MaybeStringPtr *)self_blob;
        return self->get() != nullptr;
    }
    bool lib_ruby_parser__external__maybe__string_ptr__is_none(const MaybeStringPtr_BLOB *self_blob)
    {
        const MaybeStringPtr *self = (const MaybeStringPtr *)self_blob;
        return self->get() == nullptr;
    }
    const StringPtr_BLOB *lib_ruby_parser__external__maybe__string_ptr__as_value(const MaybeStringPtr_BLOB *self_blob)
    {
        const MaybeStringPtr *self = (const MaybeStringPtr *)self_blob;
        // they have equal structure
        return (const StringPtr_BLOB *)self;
    }
    StringPtr_BLOB lib_ruby_parser__external__maybe__string_ptr__into_value(MaybeStringPtr_BLOB self_blob)
    {
        return PACK_StringPtr(UNPACK_MaybeStringPtr(self_blob));
    }

    /*
        MaybeDecoder
    */
    MaybeDecoder_BLOB lib_ruby_parser__external__maybe__decoder__new_some(Decoder_BLOB decoder_blob)
    {
        return PACK_MaybeDecoder(MaybeDecoder(UNPACK_Decoder(decoder_blob)));
    }
    MaybeDecoder_BLOB lib_ruby_parser__external__maybe__decoder__new_none()
    {
        return PACK_MaybeDecoder(MaybeDecoder());
    }
    void lib_ruby_parser__external__maybe__decoder__drop(MaybeDecoder_BLOB *self_blob)
    {
        (void)self_blob;
    }
    bool lib_ruby_parser__external__maybe__decoder__is_some(const MaybeDecoder_BLOB *self_blob)
    {
        const MaybeDecoder *self = (const MaybeDecoder *)self_blob;
        return self->decoder.has_value();
    }
    bool lib_ruby_parser__external__maybe__decoder__is_none(const MaybeDecoder_BLOB *self_blob)
    {
        const MaybeDecoder *self = (const MaybeDecoder *)self_blob;
        return !self->decoder.has_value();
    }
    const Decoder_BLOB *lib_ruby_parser__external__maybe__decoder__as_value(const MaybeDecoder_BLOB *self_blob)
    {
        MaybeDecoder *self = (MaybeDecoder *)self_blob;
        if (self->decoder.has_value())
        {
            auto decoder = &(*(self->decoder));
            return (const Decoder_BLOB *)decoder;
        }
        return nullptr;
    }
    Decoder_BLOB lib_ruby_parser__external__maybe__decoder__into_value(MaybeDecoder_BLOB self_blob)
    {
        MaybeDecoder self = UNPACK_MaybeDecoder(self_blob);
        Decoder decoder = std::move(self.decoder.value());
        return PACK_Decoder(std::move(decoder));
    }

    /*
        MaybeTokenRewriter
    */
    MaybeTokenRewriter_BLOB lib_ruby_parser__external__maybe__token_rewriter__new_some(TokenRewriter_BLOB token_rewriter_blob)
    {
        return PACK_MaybeTokenRewriter(MaybeTokenRewriter(UNPACK_TokenRewriter(token_rewriter_blob)));
    }
    MaybeTokenRewriter_BLOB lib_ruby_parser__external__maybe__token_rewriter__new_none()
    {
        return PACK_MaybeTokenRewriter(MaybeTokenRewriter());
    }
    void lib_ruby_parser__external__maybe__token_rewriter__drop(MaybeTokenRewriter_BLOB *self_blob)
    {
        (void)self_blob;
    }
    bool lib_ruby_parser__external__maybe__token_rewriter__is_some(const MaybeTokenRewriter_BLOB *self_blob)
    {
        const MaybeTokenRewriter *self = (const MaybeTokenRewriter *)self_blob;
        return self->token_rewriter.has_value();
    }
    bool lib_ruby_parser__external__maybe__token_rewriter__is_none(const MaybeTokenRewriter_BLOB *self_blob)
    {
        const MaybeTokenRewriter *self = (const MaybeTokenRewriter *)self_blob;
        return !self->token_rewriter.has_value();
    }
    const TokenRewriter_BLOB *lib_ruby_parser__external__maybe__token_rewriter__as_value(const MaybeTokenRewriter_BLOB *self_blob)
    {
        MaybeTokenRewriter *self = (MaybeTokenRewriter *)self_blob;
        if (self->token_rewriter.has_value())
        {
            auto token_rewriter = &(*(self->token_rewriter));
            return (const TokenRewriter_BLOB *)token_rewriter;
        }
        return nullptr;
    }
    TokenRewriter_BLOB lib_ruby_parser__external__maybe__token_rewriter__into_value(MaybeTokenRewriter_BLOB self_blob)
    {
        MaybeTokenRewriter self = UNPACK_MaybeTokenRewriter(self_blob);
        TokenRewriter token_rewriter = std::move(self.token_rewriter.value());
        return PACK_TokenRewriter(std::move(token_rewriter));
    }

    /*
        StringPtr
    */
    StringPtr_BLOB lib_ruby_parser__external__string_ptr__new(const uint8_t *ptr, uint64_t len)
    {
        return PACK_StringPtr(std::make_unique<std::string>((const char *)ptr, len));
    }
    void lib_ruby_parser__external__string_ptr__drop(StringPtr_BLOB *self_blob)
    {
        StringPtr *self = (StringPtr *)self_blob;
        self->~unique_ptr();
    }
    const uint8_t *lib_ruby_parser__external__string_ptr__as_raw(const StringPtr_BLOB *self_blob)
    {
        StringPtr *self = (StringPtr *)self_blob;
        if (self->get()->length() == 0)
        {
            return nullptr;
        }
        else
        {
            return (uint8_t *)(self->get()->c_str());
        }
    }
    uint64_t lib_ruby_parser__external__string_ptr__get_len(const StringPtr_BLOB *self_blob)
    {
        StringPtr *self = (StringPtr *)self_blob;
        return self->get()->length();
    }

    /*
        SharedByteList
    */
    SharedByteList_BLOB lib_ruby_parser__external__shared_byte_list__new(const uint8_t *ptr, uint64_t len)
    {
        return PACK_SharedByteList(SharedByteList((const char *)ptr, len));
    }
    void lib_ruby_parser__external__shared_byte_list__drop(SharedByteList_BLOB *self_blob)
    {
        (void)self_blob;
    }
    const uint8_t *lib_ruby_parser__external__shared_byte_list__get_raw(const SharedByteList_BLOB *self_blob)
    {
        const SharedByteList *self = (const SharedByteList *)self_blob;
        if (self->length() == 0)
        {
            return nullptr;
        }
        else
        {

            return (const uint8_t *)(self->begin());
        }
    }
    uint64_t lib_ruby_parser__external__shared_byte_list__get_len(const SharedByteList_BLOB *self_blob)
    {
        const SharedByteList *self = (const SharedByteList *)self_blob;
        return self->length();
    }

    /*
        Lists
    */

#define DECLARE_REPACK_LIST_OF(VALUE)                                                                         \
    union VALUE##_U_BLOB_TO_VALUE_                                                                            \
    {                                                                                                         \
        std::vector<VALUE##_BLOB> vec_of_blobs;                                                               \
        std::vector<VALUE> vec_of_values;                                                                     \
                                                                                                              \
        ~VALUE##_U_BLOB_TO_VALUE_() {}                                                                        \
    };                                                                                                        \
    static std::vector<VALUE> VALUE##_VEC_OF_BLOBS_TO_VEC_OF_VALUES(std::vector<VALUE##_BLOB> &&vec_of_blobs) \
    {                                                                                                         \
        return std::move(VALUE##_U_BLOB_TO_VALUE_{std::move(vec_of_blobs)}.vec_of_values);                    \
    }

#define LIST_IMPL(ITEM, ITEM_BLOB, LIST, LIST_BLOB, NS)                                              \
                                                                                                     \
    LIST_BLOB lib_ruby_parser__external__list__##NS##__new()                                         \
    {                                                                                                \
        return PACK_##LIST(LIST());                                                                  \
    }                                                                                                \
    void lib_ruby_parser__external__list__##NS##__drop(LIST_BLOB *self_blob)                         \
    {                                                                                                \
        LIST *self = (LIST *)self_blob;                                                              \
        self->~vector();                                                                             \
    }                                                                                                \
    LIST_BLOB lib_ruby_parser__external__list__##NS##__with_capacity(uint64_t capacity)              \
    {                                                                                                \
        LIST list;                                                                                   \
        list.reserve(capacity);                                                                      \
        return PACK_##LIST(std::move(list));                                                         \
    }                                                                                                \
    LIST_BLOB lib_ruby_parser__external__list__##NS##__from_raw(ITEM_BLOB *ptr, uint64_t len)        \
    {                                                                                                \
        if (len > 0)                                                                                 \
        {                                                                                            \
            std::vector<ITEM_BLOB> list_of_blobs(ptr, ptr + len);                                    \
            std::vector<ITEM> list = ITEM##_VEC_OF_BLOBS_TO_VEC_OF_VALUES(std::move(list_of_blobs)); \
            free(ptr);                                                                               \
            return PACK_##LIST(std::move(list));                                                     \
        }                                                                                            \
        else                                                                                         \
        {                                                                                            \
            return lib_ruby_parser__external__list__##NS##__new();                                   \
        }                                                                                            \
    }                                                                                                \
    void lib_ruby_parser__external__list__##NS##__push(LIST_BLOB *self_blob, ITEM_BLOB item_blob)    \
    {                                                                                                \
        LIST *self = (LIST *)self_blob;                                                              \
        self->push_back(UNPACK_##ITEM(item_blob));                                                   \
    }                                                                                                \
    ITEM_BLOB lib_ruby_parser__external__list__##NS##__remove(LIST_BLOB *self_blob, uint64_t index)  \
    {                                                                                                \
        LIST *self = (LIST *)self_blob;                                                              \
        ITEM item = std::move(self->data()[index]);                                                  \
        self->erase(self->begin() + index);                                                          \
        return PACK_##ITEM(std::move(item));                                                         \
    }                                                                                                \
    void lib_ruby_parser__external__list__##NS##__shrink_to_fit(LIST_BLOB *self_blob)                \
    {                                                                                                \
        LIST *self = (LIST *)self_blob;                                                              \
        self->shrink_to_fit();                                                                       \
    }                                                                                                \
    const ITEM_BLOB *lib_ruby_parser__external__list__##NS##__as_ptr(const LIST_BLOB *self_blob)     \
    {                                                                                                \
        LIST *self = (LIST *)self_blob;                                                              \
        return (ITEM_BLOB *)(self->data());                                                          \
    }                                                                                                \
    ITEM_BLOB *lib_ruby_parser__external__list__##NS##__into_ptr(LIST_BLOB self_blob)                \
    {                                                                                                \
        LIST self = UNPACK_##LIST(self_blob);                                                        \
        ITEM *ptr = self.data();                                                                     \
        union VALUE##_FORGET_LIST                                                                    \
        {                                                                                            \
            LIST list;                                                                               \
            uint8_t data[sizeof(LIST)];                                                              \
                                                                                                     \
            ~VALUE##_FORGET_LIST() {}                                                                \
        };                                                                                           \
        VALUE##_FORGET_LIST forget{std::move(self)};                                                 \
        return (ITEM_BLOB *)ptr;                                                                     \
    }                                                                                                \
    uint64_t lib_ruby_parser__external__list__##NS##__get_len(const LIST_BLOB *self_blob)            \
    {                                                                                                \
        const LIST *self = (const LIST *)self_blob;                                                  \
        return self->size();                                                                         \
    }                                                                                                \
    uint64_t lib_ruby_parser__external__list__##NS##__get_capacity(const LIST_BLOB *self_blob)       \
    {                                                                                                \
        const LIST *self = (const LIST *)self_blob;                                                  \
        return self->capacity();                                                                     \
    }

    DECLARE_REPACK_LIST_OF(Byte);
    LIST_IMPL(Byte, Byte_BLOB, ByteList, ByteList_BLOB, of_bytes)

    DECLARE_REPACK_LIST_OF(Token);
    LIST_IMPL(Token, Token_BLOB, TokenList, TokenList_BLOB, of_tokens)
    DECLARE_REPACK_LIST_OF(Node);
    LIST_IMPL(Node, Node_BLOB, NodeList, NodeList_BLOB, of_nodes)
    DECLARE_REPACK_LIST_OF(Diagnostic);
    LIST_IMPL(Diagnostic, Diagnostic_BLOB, DiagnosticList, DiagnosticList_BLOB, of_diagnostics)
    DECLARE_REPACK_LIST_OF(Comment);
    LIST_IMPL(Comment, Comment_BLOB, CommentList, CommentList_BLOB, of_comments)
    DECLARE_REPACK_LIST_OF(MagicComment);
    LIST_IMPL(MagicComment, MagicComment_BLOB, MagicCommentList, MagicCommentList_BLOB, of_magic_comments)
    DECLARE_REPACK_LIST_OF(SourceLine);
    LIST_IMPL(SourceLine, SourceLine_BLOB, SourceLineList, SourceLineList_BLOB, of_source_lines)

    /*
        SourceLine
    */
    SourceLine_BLOB lib_ruby_parser__external__source_line__new(uint64_t start, uint64_t end, bool ends_with_eof)
    {
        return PACK_SourceLine(SourceLine(start, end, ends_with_eof));
    }
    void lib_ruby_parser__external__source_line__drop(SourceLine_BLOB *self_blob)
    {
        (void)self_blob;
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
        return PACK_Bytes(Bytes(UNPACK_ByteList(raw_blob)));
    }
    void lib_ruby_parser__external__bytes__drop(Bytes_BLOB *self_blob)
    {
        Bytes *self = (Bytes *)self_blob;
        self->~Bytes();
    }
    const ByteList_BLOB *lib_ruby_parser__external__bytes__get_raw(const Bytes_BLOB *self_blob)
    {
        const Bytes *self = (const Bytes *)self_blob;
        return (const ByteList_BLOB *)(&(self->raw));
    }
    void lib_ruby_parser__external__bytes__set_raw(Bytes_BLOB *self_blob, ByteList_BLOB raw_blob)
    {
        Bytes *self = (Bytes *)self_blob;
        self->raw = UNPACK_ByteList(raw_blob);
    }
    ByteList_BLOB lib_ruby_parser__external__bytes__into_raw(Bytes_BLOB self_blob)
    {
        return PACK_ByteList(UNPACK_Bytes(self_blob).raw);
    }
    void lib_ruby_parser__external__bytes__push(Bytes_BLOB *self_blob, Byte byte)
    {
        Bytes *self = (Bytes *)self_blob;
        self->raw.push_back(byte);
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
        return PACK_Token(
            Token(
                token_type,
                UNPACK_Bytes(token_value_blob),
                UNPACK_Loc(loc_blob),
                lex_state_before,
                lex_state_after));
    }
    void lib_ruby_parser__external__token__drop(Token_BLOB *self_blob)
    {
        Token *self = (Token *)self_blob;
        self->~Token();
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
        return PACK_CommentType(CommentType::INLINE);
    }
    CommentType_BLOB lib_ruby_parser__external__comment_type__new_document()
    {
        return PACK_CommentType(CommentType::DOCUMENT);
    }
    CommentType_BLOB lib_ruby_parser__external__comment_type__new_unknown()
    {
        return PACK_CommentType(CommentType::UNKNOWN);
    }
    void lib_ruby_parser__external__comment_type__drop(CommentType_BLOB *self_blob)
    {
        (void)self_blob;
    }
    bool lib_ruby_parser__external__comment_type__is_inline(const CommentType_BLOB *self_blob)
    {
        const CommentType *self = (const CommentType *)self_blob;
        return *self == CommentType::INLINE;
    }
    bool lib_ruby_parser__external__comment_type__is_document(const CommentType_BLOB *self_blob)
    {
        const CommentType *self = (const CommentType *)self_blob;
        return *self == CommentType::DOCUMENT;
    }
    bool lib_ruby_parser__external__comment_type__is_unknown(const CommentType_BLOB *self_blob)
    {
        const CommentType *self = (const CommentType *)self_blob;
        return *self == CommentType::UNKNOWN;
    }

    /*
        Comment
    */
    Comment_BLOB lib_ruby_parser__external__comment__new(Loc_BLOB location_blob, CommentType_BLOB kind_blob)
    {
        return PACK_Comment(
            Comment(
                UNPACK_Loc(location_blob),
                UNPACK_CommentType(kind_blob)));
    }
    void lib_ruby_parser__external__comment__drop(Comment_BLOB *self_blob)
    {
        Comment *self = (Comment *)self_blob;
        self->~Comment();
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
        return PACK_Loc(Loc(begin, end));
    }
    void lib_ruby_parser__external__loc__drop(Loc_BLOB *self_blob)
    {
        Loc *self = (Loc *)self_blob;
        self->~Loc();
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
        MagicCommentKind
    */
    MagicCommentKind_BLOB lib_ruby_parser__external__magic_comment_kind__new_encoding()
    {
        return PACK_MagicCommentKind(MagicCommentKind::ENCODING);
    }
    MagicCommentKind_BLOB lib_ruby_parser__external__magic_comment_kind__new_frozen_string_literal()
    {
        return PACK_MagicCommentKind(MagicCommentKind::FROZEN_STRING_LITERAL);
    }
    MagicCommentKind_BLOB lib_ruby_parser__external__magic_comment_kind__new_warn_indent()
    {
        return PACK_MagicCommentKind(MagicCommentKind::WARN_INDENT);
    }
    MagicCommentKind_BLOB lib_ruby_parser__external__magic_comment_kind__new_shareable_constant_value()
    {
        return PACK_MagicCommentKind(MagicCommentKind::SHAREABLE_CONSTANT_VALUE);
    }
    void lib_ruby_parser__external__magic_comment_kind__drop(MagicCommentKind_BLOB *self_blob)
    {
        (void)self_blob;
    }
    bool lib_ruby_parser__external__magic_comment_kind__is_encoding(const MagicCommentKind_BLOB *self_blob)
    {
        const MagicCommentKind *self = (const MagicCommentKind *)self_blob;
        return *self == MagicCommentKind::ENCODING;
    }
    bool lib_ruby_parser__external__magic_comment_kind__is_frozen_string_literal(const MagicCommentKind_BLOB *self_blob)
    {
        const MagicCommentKind *self = (const MagicCommentKind *)self_blob;
        return *self == MagicCommentKind::FROZEN_STRING_LITERAL;
    }
    bool lib_ruby_parser__external__magic_comment_kind__is_warn_indent(const MagicCommentKind_BLOB *self_blob)
    {
        const MagicCommentKind *self = (const MagicCommentKind *)self_blob;
        return *self == MagicCommentKind::WARN_INDENT;
    }
    bool lib_ruby_parser__external__magic_comment_kind__is_shareable_constant_value(const MagicCommentKind_BLOB *self_blob)
    {
        const MagicCommentKind *self = (const MagicCommentKind *)self_blob;
        return *self == MagicCommentKind::SHAREABLE_CONSTANT_VALUE;
    }

    /*
        MagicComment
    */

    MagicComment_BLOB lib_ruby_parser__external__magic_comment__new(MagicCommentKind_BLOB kind, Loc_BLOB key_l, Loc_BLOB value_l)
    {
        return PACK_MagicComment(
            MagicComment(
                UNPACK_MagicCommentKind(kind),
                UNPACK_Loc(key_l),
                UNPACK_Loc(value_l)));
    }
    void lib_ruby_parser__external__magic_comment__drop(MagicComment_BLOB *self_blob)
    {
        MagicComment *self = (MagicComment *)self_blob;
        self->~MagicComment();
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
        return PACK_ErrorLevel(ErrorLevel::WARNING);
    }
    ErrorLevel_BLOB lib_ruby_parser__external__error_level__new_error()
    {
        return PACK_ErrorLevel(ErrorLevel::ERROR);
    }
    void lib_ruby_parser__external__error_level__drop(ErrorLevel_BLOB *self_blob)
    {
        (void)self_blob;
    }
    bool lib_ruby_parser__external__error_level__is_warning(const ErrorLevel_BLOB *self_blob)
    {
        const ErrorLevel *self = (const ErrorLevel *)self_blob;
        return *self == ErrorLevel::WARNING;
    }
    bool lib_ruby_parser__external__error_level__is_error(const ErrorLevel_BLOB *self_blob)
    {
        const ErrorLevel *self = (const ErrorLevel *)self_blob;
        return *self == ErrorLevel::ERROR;
    }

    /*
        Diagnostic
    */
    Diagnostic_BLOB lib_ruby_parser__external__diagnostic__new(
        ErrorLevel_BLOB level_blob,
        DiagnosticMessage_BLOB message_blob,
        Loc_BLOB loc_blob)
    {
        return PACK_Diagnostic(
            Diagnostic(
                UNPACK_ErrorLevel(level_blob),
                UNPACK_DiagnosticMessage(message_blob),
                UNPACK_Loc(loc_blob)));
    }
    void lib_ruby_parser__external__diagnostic__drop(Diagnostic_BLOB *self_blob)
    {
        Diagnostic *self = (Diagnostic *)self_blob;
        self->~Diagnostic();
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
        return PACK_InputError(InputError(InputError::UnsupportedEncoding(UNPACK_StringPtr(err_blob))));
    }
    InputError_BLOB lib_ruby_parser__external__input_error__new_decoding_error(StringPtr_BLOB err_blob)
    {
        return PACK_InputError(InputError(InputError::DecodingError(UNPACK_StringPtr(err_blob))));
    }
    void lib_ruby_parser__external__input_error__drop(InputError_BLOB *self_blob)
    {
        ((InputError *)self_blob)->~InputError();
    }
    bool lib_ruby_parser__external__input_error__is_unsupported_encoding(const InputError_BLOB *self_blob)
    {
        const InputError *self = (const InputError *)self_blob;
        return std::holds_alternative<InputError::UnsupportedEncoding>(self->variant);
    }
    bool lib_ruby_parser__external__input_error__is_decoding_error(const InputError_BLOB *self_blob)
    {
        const InputError *self = (const InputError *)self_blob;
        return std::holds_alternative<InputError::DecodingError>(self->variant);
    }
    const StringPtr_BLOB *lib_ruby_parser__external__input_error__get_unsupported_encoding(const InputError_BLOB *self_blob)
    {
        const InputError *self = (const InputError *)self_blob;
        const InputError::UnsupportedEncoding *variant = std::get_if<InputError::UnsupportedEncoding>(&(self->variant));
        if (variant == nullptr)
            return nullptr;
        return (const StringPtr_BLOB *)(&(variant->message));
    }
    const StringPtr_BLOB *lib_ruby_parser__external__input_error__get_decoding_error(const InputError_BLOB *self_blob)
    {
        const InputError *self = (const InputError *)self_blob;
        const InputError::DecodingError *variant = std::get_if<InputError::DecodingError>(&(self->variant));
        if (variant == nullptr)
            return nullptr;
        return (const StringPtr_BLOB *)(&(variant->message));
    }

    /*
        Decoder
    */
    DecoderResult_BLOB lib_ruby_parser__external__decoder_result__new_ok(ByteList_BLOB byte_list_blob)
    {
        return PACK_DecoderResult(DecoderResult(DecoderResult::Ok(UNPACK_ByteList(byte_list_blob))));
    }
    DecoderResult_BLOB lib_ruby_parser__external__decoder_result__new_err(InputError_BLOB input_error_blob)
    {
        return PACK_DecoderResult(DecoderResult(DecoderResult::Err(UNPACK_InputError(input_error_blob))));
    }
    void lib_ruby_parser__external__decoder_result__drop(DecoderResult_BLOB *self_blob)
    {
        DecoderResult *self = (DecoderResult *)self_blob;
        self->~DecoderResult();
    }
    bool lib_ruby_parser__external__decoder_result_is_ok(const DecoderResult_BLOB *self_blob)
    {
        const DecoderResult *self = (const DecoderResult *)self_blob;
        return std::holds_alternative<DecoderResult::Ok>(self->variant);
    }
    bool lib_ruby_parser__external__decoder_result_is_err(const DecoderResult_BLOB *self_blob)
    {
        const DecoderResult *self = (const DecoderResult *)self_blob;
        return std::holds_alternative<DecoderResult::Err>(self->variant);
    }
    ByteList_BLOB lib_ruby_parser__external__decoder_result_into_ok(DecoderResult_BLOB self_blob)
    {
        return PACK_ByteList(std::get<DecoderResult::Ok>(UNPACK_DecoderResult(self_blob).variant).output);
    }
    InputError_BLOB lib_ruby_parser__external__decoder_result_into_err(DecoderResult_BLOB self_blob)
    {
        return PACK_InputError(std::get<DecoderResult::Err>(UNPACK_DecoderResult(self_blob).variant).error);
    }
    const ByteList_BLOB *lib_ruby_parser__external__decoder_result_as_ok(const DecoderResult_BLOB *self_blob)
    {
        const DecoderResult *self = (const DecoderResult *)self_blob;
        const DecoderResult::Ok *ok = std::get_if<DecoderResult::Ok>(&(self->variant));
        if (ok == nullptr)
            return nullptr;
        return (const ByteList_BLOB *)(&(ok->output));
    }
    const InputError_BLOB *lib_ruby_parser__external__decoder_result_as_err(const DecoderResult_BLOB *self_blob)
    {
        const DecoderResult *self = (const DecoderResult *)self_blob;
        const DecoderResult::Err *err = std::get_if<DecoderResult::Err>(&(self->variant));
        if (err == nullptr)
            return nullptr;
        return (const InputError_BLOB *)(&(err->error));
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
        (void)encoding; // dtor
        ByteList input = UNPACK_ByteList(input_blob);
        (void)input; // dtor

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
        Decoder decoder;
        decoder.f = f;
        return PACK_Decoder(decoder);
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
        return *self == RewriteAction::DROP;
    }
    bool lib_ruby_parser__external__rewrite_action__is_keep(const RewriteAction_BLOB *self_blob)
    {
        const RewriteAction *self = (const RewriteAction *)self_blob;
        return *self == RewriteAction::KEEP;
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
        return self->kind == LexStateAction::Kind::SET;
    }
    bool lib_ruby_parser__external__lex_state_action__is_keep(const LexStateAction_BLOB *self_blob)
    {
        const LexStateAction *self = (const LexStateAction *)self_blob;
        return self->kind == LexStateAction::Kind::KEEP;
    }
    int32_t lib_ruby_parser__external__lex_state_action__get_next_state(const LexStateAction_BLOB *self_blob)
    {
        const LexStateAction *self = (const LexStateAction *)self_blob;
        return self->next_state;
    }

    /*
        TokenRewriterResult
    */
    void lib_ruby_parser__external__token_rewriter_result__drop(TokenRewriterResult_BLOB *self_blob)
    {
        TokenRewriterResult *self = (TokenRewriterResult *)self_blob;
        self->~TokenRewriterResult();
    }
    InternalTokenRewriterResult lib_ruby_parser__external__token_rewriter_result__into_internal(TokenRewriterResult_BLOB self_blob)
    {
        TokenRewriterResult self = UNPACK_TokenRewriterResult(self_blob);
        InternalTokenRewriterResult output = {
            .rewritten_token = PACK_TokenPtr(std::move(self.rewritten_token)),
            .token_action = PACK_RewriteAction(self.token_action),
            .lex_state_action = PACK_LexStateAction(std::move(self.lex_state_action))};
        return output;
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
        std::unique_ptr<Token> token = UNPACK_TokenPtr(token_blob);
        (void)input_blob;

        // call dummy token_rewriter
        TokenRewriter *self = (TokenRewriter *)self_blob;
        TokenRewriterResult result = self->rewrite_f(std::move(token), self->build_new_token_f);
        return PACK_TokenRewriterResult(std::move(result));
    }
    // Test APIs
    TokenRewriter_BLOB lib_ruby_parser__external__token_rewriter__new_keep(build_new_token_t build_new_token_f)
    {
        return PACK_TokenRewriter(TokenRewriter::NewKeepRewriter(build_new_token_f));
    }
    TokenRewriter_BLOB lib_ruby_parser__external__token_rewriter__new_drop(build_new_token_t build_new_token_f)
    {
        return PACK_TokenRewriter(TokenRewriter::NewDropRewriter(build_new_token_f));
    }
    TokenRewriter_BLOB lib_ruby_parser__external__token_rewriter__new_rewrite(
        build_new_token_t build_new_token_f)
    {
        return PACK_TokenRewriter(TokenRewriter::NewRewriteRewriter(build_new_token_f));
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
        return PACK_ParserOptions(
            ParserOptions(
                UNPACK_StringPtr(buffer_name_blob),
                debug,
                UNPACK_MaybeDecoder(decoder_blob),
                UNPACK_MaybeTokenRewriter(token_rewriter_blob),
                record_tokens));
    }
    void lib_ruby_parser__external__parser_options__drop(ParserOptions_BLOB *self_blob)
    {
        ParserOptions *options = (ParserOptions *)self_blob;
        options->~ParserOptions();
    }
    InternalParserOptions lib_ruby_parser__external__parser_options__into_internal(ParserOptions_BLOB self_blob)
    {
        ParserOptions self = UNPACK_ParserOptions(self_blob);
        InternalParserOptions internal = {
            .buffer_name = PACK_StringPtr(std::move(self.buffer_name)),
            .debug = self.debug,
            .decoder = PACK_MaybeDecoder(std::move(self.decoder)),
            .token_rewriter = PACK_MaybeTokenRewriter(std::move(self.token_rewriter)),
            .record_tokens = self.record_tokens};
        return internal;
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

    /*
        DecodedInput
    */
    DecodedInput_BLOB lib_ruby_parser__external__decoded_input__new(
        StringPtr_BLOB name_blob,
        SourceLineList_BLOB lines_blob,
        ByteList_BLOB bytes_blob)
    {
        return PACK_DecodedInput(
            DecodedInput(
                UNPACK_StringPtr(name_blob),
                UNPACK_SourceLineList(lines_blob),
                UNPACK_ByteList(bytes_blob)));
    }
    void lib_ruby_parser__external__decoded_input__drop(DecodedInput_BLOB *self_blob)
    {
        DecodedInput *self = (DecodedInput *)self_blob;
        self->~DecodedInput();
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
    void lib_ruby_parser__external__decoded_input__set_name(DecodedInput_BLOB *self_blob, StringPtr_BLOB name_blob)
    {
        DecodedInput *self = (DecodedInput *)self_blob;
        self->name = UNPACK_StringPtr(name_blob);
    }
    void lib_ruby_parser__external__decoded_input__set_lines(DecodedInput_BLOB *self_blob, SourceLineList_BLOB lines_bob)
    {
        DecodedInput *self = (DecodedInput *)self_blob;
        self->lines = UNPACK_SourceLineList(lines_bob);
    }
    void lib_ruby_parser__external__decoded_input__set_bytes(DecodedInput_BLOB *self_blob, ByteList_BLOB bytes_blob)
    {
        DecodedInput *self = (DecodedInput *)self_blob;
        self->bytes = UNPACK_ByteList(bytes_blob);
    }
    ByteList_BLOB lib_ruby_parser__external__decoded_input__into_bytes(DecodedInput_BLOB self_blob)
    {
        DecodedInput self = UNPACK_DecodedInput(self_blob);
        return PACK_ByteList(std::move(self.bytes));
    }
    ByteList_BLOB lib_ruby_parser__external__decoded_input__take_bytes(DecodedInput_BLOB *self_blob)
    {
        DecodedInput *self = (DecodedInput *)self_blob;
        ByteList bytes = std::move(self->bytes);
        self->bytes = ByteList();
        return PACK_ByteList(std::move(bytes));
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
        return PACK_ParserResult(
            ParserResult(
                UNPACK_MaybeNodePtr(ast_blob),
                UNPACK_TokenList(tokens_blob),
                UNPACK_DiagnosticList(diagnostics_blob),
                UNPACK_CommentList(comments_blob),
                UNPACK_MagicCommentList(magic_comments_blob),
                UNPACK_DecodedInput(input_blob)));
    }
    void lib_ruby_parser__external__parser_result__drop(ParserResult_BLOB *self_blob)
    {
        ParserResult *self = (ParserResult *)self_blob;
        self->~ParserResult();
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
}
