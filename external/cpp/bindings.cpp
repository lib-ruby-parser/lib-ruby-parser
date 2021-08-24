#include "bindings.hpp"
#include <iostream>

extern "C"
{
    /*
        Ptr
    */

    Ptr_BLOB lib_ruby_parser__internal__containers__ptr__new(void *raw)
    {
        Ptr ptr = std::unique_ptr<int>((int *)raw);
        return PACK_Ptr(std::move(ptr));
    }
    Ptr_BLOB lib_ruby_parser__internal__containers__ptr__new_null()
    {
        return PACK_Ptr(nullptr);
    }
    void *lib_ruby_parser__internal__containers__ptr__get_raw(Ptr_BLOB *blob)
    {
        Ptr *ptr = (Ptr *)blob;
        return (void *)(ptr->get());
    }
    void lib_ruby_parser__internal__containers__ptr__of_node__free(Ptr_BLOB *blob)
    {
        Node *ptr = (Node *)(((Ptr *)blob)->release());
        delete ptr;
    }
    void lib_ruby_parser__internal__containers__ptr__of_token__free(Ptr_BLOB *blob)
    {
        Token *ptr = (Token *)(((Ptr *)blob)->release());
        delete ptr;
    }

    /*
        MaybePtr
    */
    MaybePtr_BLOB lib_ruby_parser__internal__containers__maybe_ptr__new(void *raw)
    {
        MaybePtr ptr = std::unique_ptr<int>((int *)raw);
        return PACK_MaybePtr(std::move(ptr));
    }
    MaybePtr_BLOB lib_ruby_parser__internal__containers__maybe_ptr__new_null()
    {
        return PACK_MaybePtr(nullptr);
    }
    void *lib_ruby_parser__internal__containers__maybe_ptr__get_raw(MaybePtr_BLOB *blob)
    {
        MaybePtr *ptr = (MaybePtr *)blob;
        return (void *)(ptr->get());
    }
    void lib_ruby_parser__internal__containers__maybe_ptr__of_node__free(MaybePtr_BLOB *blob)
    {
        Node *ptr = (Node *)(((MaybePtr *)blob)->release());
        if (ptr != nullptr)
        {
            delete ptr;
        }
    }
    void lib_ruby_parser__internal__containers__maybe_ptr__of_token__free(MaybePtr_BLOB *blob)
    {
        Token *ptr = (Token *)(((MaybePtr *)blob)->release());
        if (ptr != nullptr)
        {
            delete ptr;
        }
    }

    /*
        StringPtr
    */
    StringPtr_BLOB lib_ruby_parser__internal__containers__string_ptr__new(const uint8_t *ptr, uint64_t len)
    {
        StringPtr string_ptr = std::make_unique<std::string>((const char *)ptr, len);
        return PACK_StringPtr(std::move(string_ptr));
    }
    void lib_ruby_parser__internal__containers__string_ptr__drop(StringPtr_BLOB *blob)
    {
        StringPtr *string_ptr = (StringPtr *)blob;
        string_ptr->~unique_ptr();
    }
    uint8_t *lib_ruby_parser__internal__containers__string_ptr__get_raw(StringPtr_BLOB *blob)
    {
        StringPtr *string_ptr = (StringPtr *)blob;
        if (string_ptr->get()->length() == 0)
        {
            return nullptr;
        }
        else
        {
            return (uint8_t *)(string_ptr->get()->c_str());
        }
    }
    uint64_t lib_ruby_parser__internal__containers__string_ptr__get_len(const StringPtr_BLOB *blob)
    {
        StringPtr *string_ptr = (StringPtr *)blob;
        return string_ptr->get()->length();
    }

    /*
        MaybeStringPtr
    */
    MaybeStringPtr_BLOB lib_ruby_parser__internal__containers__maybe_string_ptr__new_some(const uint8_t *ptr, uint64_t len)
    {
        return PACK_MaybeStringPtr(std::make_unique<std::string>((const char *)ptr, len));
    }
    MaybeStringPtr_BLOB lib_ruby_parser__internal__containers__maybe_string_ptr__new_none()
    {
        return PACK_MaybeStringPtr(std::unique_ptr<std::string>(nullptr));
    }
    bool lib_ruby_parser__internal__containers__maybe_string_ptr__is_some(const MaybeStringPtr_BLOB *blob)
    {
        const MaybeStringPtr *maybe_string_ptr = (const MaybeStringPtr *)blob;
        return maybe_string_ptr->get() != nullptr;
    }
    bool lib_ruby_parser__internal__containers__maybe_string_ptr__is_none(const MaybeStringPtr_BLOB *blob)
    {
        const MaybeStringPtr *maybe_string_ptr = (const MaybeStringPtr *)blob;
        return maybe_string_ptr->get() == nullptr;
    }
    void lib_ruby_parser__internal__containers__maybe_string_ptr__drop(MaybeStringPtr_BLOB *blob)
    {
        MaybeStringPtr *maybe_string_ptr = (MaybeStringPtr *)blob;
        maybe_string_ptr->~unique_ptr();
    }
    uint8_t *lib_ruby_parser__internal__containers__maybe_string_ptr__get_raw(MaybeStringPtr_BLOB *blob)
    {
        MaybeStringPtr *maybe_string_ptr = (MaybeStringPtr *)blob;
        if (maybe_string_ptr->get() == nullptr)
        {
            return nullptr;
        }
        if (maybe_string_ptr->get()->length() == 0)
        {
            return nullptr;
        }
        return (uint8_t *)(maybe_string_ptr->get()->c_str());
    }
    uint8_t *lib_ruby_parser__internal__containers__maybe_string_ptr__into_raw(MaybeStringPtr_BLOB *blob)
    {
        MaybeStringPtr *maybe_string_ptr = (MaybeStringPtr *)blob;
        std::string *s = maybe_string_ptr->release();
        if (s == nullptr)
        {
            return nullptr;
        }
        uint8_t *result = (uint8_t *)(s->c_str());
        return result;
    }
    uint64_t lib_ruby_parser__internal__containers__maybe_string_ptr__len(const MaybeStringPtr_BLOB *blob)
    {
        const MaybeStringPtr *maybe_string_ptr = (const MaybeStringPtr *)blob;
        return maybe_string_ptr->get()->length();
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

#define LIST_IMPL(ITEM, ITEM_BLOB, LIST, LIST_BLOB, NS)                                                    \
                                                                                                           \
    LIST_BLOB lib_ruby_parser__internal__containers__list__##NS##__new()                                   \
    {                                                                                                      \
        return PACK_##LIST(LIST());                                                                        \
    }                                                                                                      \
    void lib_ruby_parser__internal__containers__list__##NS##__drop(LIST_BLOB *blob)                        \
    {                                                                                                      \
        LIST *list = (LIST *)blob;                                                                         \
        list->~vector();                                                                                   \
    }                                                                                                      \
    LIST_BLOB lib_ruby_parser__internal__containers__list__##NS##__with_capacity(uint64_t capacity)        \
    {                                                                                                      \
        LIST list;                                                                                         \
        list.reserve(capacity);                                                                            \
        return PACK_##LIST(std::move(list));                                                               \
    }                                                                                                      \
    LIST_BLOB lib_ruby_parser__internal__containers__list__##NS##__from_raw(ITEM_BLOB *ptr, uint64_t len)  \
    {                                                                                                      \
        if (len > 0)                                                                                       \
        {                                                                                                  \
            std::vector<ITEM_BLOB> list_of_blobs(ptr, ptr + len);                                          \
            std::vector<ITEM> list = ITEM##_VEC_OF_BLOBS_TO_VEC_OF_VALUES(std::move(list_of_blobs));       \
            free(ptr);                                                                                     \
            return PACK_##LIST(std::move(list));                                                           \
        }                                                                                                  \
        else                                                                                               \
        {                                                                                                  \
            return lib_ruby_parser__internal__containers__list__##NS##__new();                             \
        }                                                                                                  \
    }                                                                                                      \
    void lib_ruby_parser__internal__containers__list__##NS##__push(LIST_BLOB *blob, ITEM_BLOB item_blob)   \
    {                                                                                                      \
        LIST *list = (LIST *)blob;                                                                         \
        list->push_back(UNPACK_##ITEM(item_blob));                                                         \
    }                                                                                                      \
    ITEM_BLOB lib_ruby_parser__internal__containers__list__##NS##__remove(LIST_BLOB *blob, uint64_t index) \
    {                                                                                                      \
        LIST *list = (LIST *)blob;                                                                         \
        ITEM item = std::move(list->data()[index]);                                                        \
        list->erase(list->begin() + index);                                                                \
        return PACK_##ITEM(std::move(item));                                                               \
    }                                                                                                      \
    void lib_ruby_parser__internal__containers__list__##NS##__shrink_to_fit(LIST_BLOB *blob)               \
    {                                                                                                      \
        LIST *list = (LIST *)blob;                                                                         \
        list->shrink_to_fit();                                                                             \
    }                                                                                                      \
    ITEM_BLOB *lib_ruby_parser__internal__containers__list__##NS##__as_ptr(LIST_BLOB *blob)                \
    {                                                                                                      \
        LIST *list = (LIST *)blob;                                                                         \
        return (ITEM_BLOB *)(list->data());                                                                \
    }                                                                                                      \
    uint64_t lib_ruby_parser__internal__containers__list__##NS##__len(const LIST_BLOB *blob)               \
    {                                                                                                      \
        const LIST *list = (const LIST *)blob;                                                             \
        return list->size();                                                                               \
    }                                                                                                      \
    uint64_t lib_ruby_parser__internal__containers__list__##NS##__capacity(const LIST_BLOB *blob)          \
    {                                                                                                      \
        const LIST *list = (const LIST *)blob;                                                             \
        return list->capacity();                                                                           \
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
    SourceLine_BLOB lib_ruby_parser__internal__containers__source_line__new(uint64_t start, uint64_t end, bool ends_with_eof)
    {
        return PACK_SourceLine(SourceLine(start, end, ends_with_eof));
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
        bytes->~Bytes();
    }
    Bytes_BLOB lib_ruby_parser__internal__containers__bytes__new_from_byte_list(ByteList_BLOB list_blob)
    {
        ByteList list = UNPACK_ByteList(list_blob);
        return PACK_Bytes(Bytes(std::move(list)));
    }
    const ByteList_BLOB *lib_ruby_parser__internal__containers__bytes__get_byte_list(const Bytes_BLOB *blob)
    {
        const Bytes *bytes = (const Bytes *)blob;
        return (const ByteList_BLOB *)(&(bytes->raw));
    }
    void lib_ruby_parser__internal__containers__bytes__set_byte_list(Bytes_BLOB *blob, ByteList_BLOB list_blob)
    {
        Bytes *bytes = (Bytes *)blob;
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
        Token token(
            token_type,
            UNPACK_Bytes(token_value),
            UNPACK_Loc(loc),
            lex_state_before,
            lex_state_after);
        return PACK_Token(std::move(token));
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
        token->~Token();
    }

    /*
        CommentType
    */
    CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__new_inline()
    {
        return PACK_CommentType(CommentType::INLINE);
    }
    CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__new_document()
    {
        return PACK_CommentType(CommentType::DOCUMENT);
    }
    CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__new_unknown()
    {
        return PACK_CommentType(CommentType::UNKNOWN);
    }
    bool lib_ruby_parser__internal__containers__comment_type__is_inline(CommentType_BLOB blob)
    {
        return UNPACK_CommentType(blob) == CommentType::INLINE;
    }
    bool lib_ruby_parser__internal__containers__comment_type__is_document(CommentType_BLOB blob)
    {
        return UNPACK_CommentType(blob) == CommentType::DOCUMENT;
    }
    bool lib_ruby_parser__internal__containers__comment_type__is_unknown(CommentType_BLOB blob)
    {
        return UNPACK_CommentType(blob) == CommentType::UNKNOWN;
    }

    /*
        Comment
    */
    Comment_BLOB lib_ruby_parser__internal__containers__comment__new(Loc_BLOB location, CommentType_BLOB kind)
    {
        Comment comment(
            UNPACK_Loc(location),
            UNPACK_CommentType(kind));
        return PACK_Comment(std::move(comment));
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
        Comment *comment = (Comment *)blob;
        comment->~Comment();
    }

    /*
        ErrorLevel
    */
    ErrorLevel_BLOB lib_ruby_parser__internal__containers__error_level__new_warning()
    {
        return PACK_ErrorLevel(ErrorLevel::WARNING);
    }
    ErrorLevel_BLOB lib_ruby_parser__internal__containers__error_level__new_error()
    {
        return PACK_ErrorLevel(ErrorLevel::ERROR);
    }
    bool lib_ruby_parser__internal__containers__error_level__is_warning(ErrorLevel_BLOB blob)
    {
        return UNPACK_ErrorLevel(blob) == ErrorLevel::WARNING;
    }
    bool lib_ruby_parser__internal__containers__error_level__is_error(ErrorLevel_BLOB blob)
    {
        return UNPACK_ErrorLevel(blob) == ErrorLevel::ERROR;
    }

    /*
        Loc
    */
    Loc_BLOB lib_ruby_parser__internal__containers__loc__new(uint64_t begin, uint64_t end)
    {
        Loc loc(begin, end);
        return PACK_Loc(std::move(loc));
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
        Loc *loc = (Loc *)blob;
        loc->~Loc();
    }

    /*
        MaybeLoc
    */
    MaybeLoc_BLOB lib_ruby_parser__internal__containers__maybe_loc__new_some(Loc_BLOB loc_blob)
    {
        MaybeLoc maybe_loc(UNPACK_Loc(loc_blob));
        return PACK_MaybeLoc(std::move(maybe_loc));
    }
    MaybeLoc_BLOB lib_ruby_parser__internal__containers__maybe_loc__new_none()
    {
        MaybeLoc maybe_loc;
        return PACK_MaybeLoc(std::move(maybe_loc));
    }
    bool lib_ruby_parser__internal__containers__maybe_loc__is_some(const MaybeLoc_BLOB *blob)
    {
        const MaybeLoc *maybe_loc = (const MaybeLoc *)blob;
        return maybe_loc->has_value();
    }
    bool lib_ruby_parser__internal__containers__maybe_loc__is_none(const MaybeLoc_BLOB *blob)
    {
        const MaybeLoc *maybe_loc = (const MaybeLoc *)blob;
        return !(maybe_loc->has_value());
    }
    Loc_BLOB *lib_ruby_parser__internal__containers__maybe_loc__get_loc(MaybeLoc_BLOB *blob)
    {
        MaybeLoc *maybe_loc = (MaybeLoc *)blob;
        if (!maybe_loc->has_value())
        {
            return nullptr;
        }
        Loc *loc = &(*(*maybe_loc));
        return (Loc_BLOB *)loc;
    }
    Loc_BLOB lib_ruby_parser__internal__containers__maybe_loc__into_loc(MaybeLoc_BLOB blob)
    {
        return PACK_Loc(UNPACK_MaybeLoc(blob).value());
    }
    void lib_ruby_parser__internal__containers__maybe_loc__drop(MaybeLoc_BLOB *blob)
    {
        MaybeLoc *maybe_loc = (MaybeLoc *)blob;
        maybe_loc->~optional();
    }

    /*
        MagicCommentKind
    */
    MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__new_encoding()
    {
        return PACK_MagicCommentKind(MagicCommentKind::ENCODING);
    }
    MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__new_frozen_string_literal()
    {
        return PACK_MagicCommentKind(MagicCommentKind::FROZEN_STRING_LITERAL);
    }
    MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__new_warn_indent()
    {
        return PACK_MagicCommentKind(MagicCommentKind::WARN_INDENT);
    }
    MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__new_shareable_constant_value()
    {
        return PACK_MagicCommentKind(MagicCommentKind::SHAREABLE_CONSTANT_VALUE);
    }

    bool lib_ruby_parser__internal__containers__magic_comment_kind__is_encoding(MagicCommentKind_BLOB blob)
    {
        return UNPACK_MagicCommentKind(blob) == MagicCommentKind::ENCODING;
    }
    bool lib_ruby_parser__internal__containers__magic_comment_kind__is_frozen_string_literal(MagicCommentKind_BLOB blob)
    {
        return UNPACK_MagicCommentKind(blob) == MagicCommentKind::FROZEN_STRING_LITERAL;
    }
    bool lib_ruby_parser__internal__containers__magic_comment_kind__is_warn_indent(MagicCommentKind_BLOB blob)
    {
        return UNPACK_MagicCommentKind(blob) == MagicCommentKind::WARN_INDENT;
    }
    bool lib_ruby_parser__internal__containers__magic_comment_kind__is_shareable_constant_value(MagicCommentKind_BLOB blob)
    {
        return UNPACK_MagicCommentKind(blob) == MagicCommentKind::SHAREABLE_CONSTANT_VALUE;
    }

    /*
        MagicComment
    */

    MagicComment_BLOB lib_ruby_parser__internal__containers__magic_comment__new(MagicCommentKind_BLOB kind, Loc_BLOB key_l, Loc_BLOB value_l)
    {
        MagicComment magic_comment(
            UNPACK_MagicCommentKind(kind),
            UNPACK_Loc(key_l),
            UNPACK_Loc(value_l));
        return PACK_MagicComment(std::move(magic_comment));
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
        MagicComment *magic_comment = (MagicComment *)blob;
        magic_comment->~MagicComment();
    }

    /*
        SharedByteList
    */
    SharedByteList_BLOB lib_ruby_parser__internal__containers__shared_byte_list__new(const uint8_t *ptr, uint64_t len)
    {
        return PACK_SharedByteList(SharedByteList((const char *)ptr, len));
    }
    const uint8_t *lib_ruby_parser__internal__containers__shared_byte_list__get_raw(const SharedByteList_BLOB *blob)
    {
        const SharedByteList *shared_byte_list = (const SharedByteList *)blob;
        if (shared_byte_list->length() == 0)
        {
            return nullptr;
        }
        else
        {

            return (const uint8_t *)(shared_byte_list->begin());
        }
    }
    uint64_t lib_ruby_parser__internal__containers__shared_byte_list__get_len(const SharedByteList_BLOB *blob)
    {
        const SharedByteList *shared_byte_list = (const SharedByteList *)blob;
        return shared_byte_list->length();
    }

    /*
        Diagnostic
    */
    Diagnostic_BLOB lib_ruby_parser__internal__containers__diagnostic__new(ErrorLevel_BLOB level, DiagnosticMessage_BLOB message, Loc_BLOB loc)
    {
        return PACK_Diagnostic(
            Diagnostic(
                UNPACK_ErrorLevel(level),
                UNPACK_DiagnosticMessage(message),
                UNPACK_Loc(loc)));
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
        diagnostic->~Diagnostic();
    }

    /*
        InputError
    */
    InputError_BLOB lib_ruby_parser__internal__containers__input_error__new_unsupported_encoding(StringPtr_BLOB err)
    {
        return PACK_InputError(InputError(InputError::UnsupportedEncoding(UNPACK_StringPtr(err))));
    }
    InputError_BLOB lib_ruby_parser__internal__containers__input_error__new_decoding_error(StringPtr_BLOB err)
    {
        return PACK_InputError(InputError(InputError::DecodingError(UNPACK_StringPtr(err))));
    }
    bool lib_ruby_parser__internal__containers__input_error__is_unsupported_encoding(const InputError_BLOB *blob)
    {
        const InputError *input_error = (const InputError *)blob;
        return std::holds_alternative<InputError::UnsupportedEncoding>(input_error->variant);
    }
    bool lib_ruby_parser__internal__containers__input_error__is_decoding_error(const InputError_BLOB *blob)
    {
        const InputError *input_error = (const InputError *)blob;
        return std::holds_alternative<InputError::DecodingError>(input_error->variant);
    }
    const StringPtr_BLOB *lib_ruby_parser__internal__containers__input_error__get_unsupported_encoding(const InputError_BLOB *blob)
    {
        const InputError *input_error = (const InputError *)blob;
        const InputError::UnsupportedEncoding *variant = std::get_if<InputError::UnsupportedEncoding>(&(input_error->variant));
        if (variant == nullptr)
            return nullptr;
        return (const StringPtr_BLOB *)(&(variant->message));
    }
    const StringPtr_BLOB *lib_ruby_parser__internal__containers__input_error__get_decoding_error(const InputError_BLOB *blob)
    {
        const InputError *input_error = (const InputError *)blob;
        const InputError::DecodingError *variant = std::get_if<InputError::DecodingError>(&(input_error->variant));
        if (variant == nullptr)
            return nullptr;
        return (const StringPtr_BLOB *)(&(variant->message));
    }
    void lib_ruby_parser__internal__containers__input_error__drop(InputError_BLOB *blob)
    {
        ((InputError *)blob)->~InputError();
    }

    /*
        Decoder
    */
    DecoderResult_BLOB lib_ruby_parser__internal__containers__decoder_result__new_ok(ByteList_BLOB byte_list)
    {
        return PACK_DecoderResult(DecoderResult(DecoderResult::Ok(UNPACK_ByteList(byte_list))));
    }
    DecoderResult_BLOB lib_ruby_parser__internal__containers__decoder_result__new_err(InputError_BLOB input_error)
    {
        return PACK_DecoderResult(DecoderResult(DecoderResult::Err(UNPACK_InputError(input_error))));
    }
    bool lib_ruby_parser__internal__containers__decoder_result_is_ok(const DecoderResult_BLOB *blob)
    {
        const DecoderResult *decoder_result = (const DecoderResult *)blob;
        return std::holds_alternative<DecoderResult::Ok>(decoder_result->variant);
    }
    bool lib_ruby_parser__internal__containers__decoder_result_is_err(const DecoderResult_BLOB *blob)
    {
        const DecoderResult *decoder_result = (const DecoderResult *)blob;
        return std::holds_alternative<DecoderResult::Err>(decoder_result->variant);
    }
    ByteList_BLOB lib_ruby_parser__internal__containers__decoder_result_into_ok(DecoderResult_BLOB blob)
    {
        return PACK_ByteList(std::get<DecoderResult::Ok>(UNPACK_DecoderResult(blob).variant).output);
    }
    InputError_BLOB lib_ruby_parser__internal__containers__decoder_result_into_err(DecoderResult_BLOB blob)
    {
        return PACK_InputError(std::get<DecoderResult::Err>(UNPACK_DecoderResult(blob).variant).error);
    }
    const ByteList_BLOB *lib_ruby_parser__internal__containers__decoder_result_as_ok(const DecoderResult_BLOB *blob)
    {
        const DecoderResult *decoder_result = (const DecoderResult *)blob;
        const DecoderResult::Ok *ok = std::get_if<DecoderResult::Ok>(&(decoder_result->variant));
        if (ok == nullptr)
            return nullptr;
        return (const ByteList_BLOB *)(&(ok->output));
    }
    const InputError_BLOB *lib_ruby_parser__internal__containers__decoder_result_as_err(const DecoderResult_BLOB *blob)
    {
        const DecoderResult *decoder_result = (const DecoderResult *)blob;
        const DecoderResult::Err *err = std::get_if<DecoderResult::Err>(&(decoder_result->variant));
        if (err == nullptr)
            return nullptr;
        return (const InputError_BLOB *)(&(err->error));
    }
    void lib_ruby_parser__internal__containers__decoder_result__drop(DecoderResult_BLOB *blob)
    {
        DecoderResult *decoder_result = (DecoderResult *)blob;
        decoder_result->~DecoderResult();
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
        (void)encoding; // dtor
        ByteList input = UNPACK_ByteList(input_blob);
        (void)input; // dtor

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
        Decoder decoder;
        decoder.f = f;
        return PACK_Decoder(decoder);
    }

    /*
        TokenRewriter
    */
    bool lib_ruby_parser__internal__containers__token_rewriter__rewrite_action__is_drop(const RewriteAction_BLOB *blob)
    {
        const RewriteAction *rewrite_action = (const RewriteAction *)blob;
        return *rewrite_action == RewriteAction::DROP;
    }
    bool lib_ruby_parser__internal__containers__token_rewriter__rewrite_action__is_keep(const RewriteAction_BLOB *blob)
    {
        const RewriteAction *rewrite_action = (const RewriteAction *)blob;
        return *rewrite_action == RewriteAction::KEEP;
    }
    void lib_ruby_parser__internal__containers__token_rewriter__rewrite_action__drop(RewriteAction_BLOB *blob)
    {
        (void)blob;
    }
    bool lib_ruby_parser__internal__containers__token_rewriter__lex_state_action__is_set(const LexStateAction_BLOB *blob)
    {
        const LexStateAction *lex_state_action = (const LexStateAction *)blob;
        return lex_state_action->kind == LexStateAction::Kind::SET;
    }
    bool lib_ruby_parser__internal__containers__token_rewriter__lex_state_action__is_keep(const LexStateAction_BLOB *blob)
    {
        const LexStateAction *lex_state_action = (const LexStateAction *)blob;
        return lex_state_action->kind == LexStateAction::Kind::KEEP;
    }
    void lib_ruby_parser__internal__containers__token_rewriter__lex_state_action__drop(LexStateAction_BLOB *blob)
    {
        (void)blob;
    }
    int32_t lib_ruby_parser__internal__containers__token_rewriter__lex_state_action__get_next_state(const LexStateAction_BLOB *blob)
    {
        const LexStateAction *lex_state_action = (const LexStateAction *)blob;
        return lex_state_action->next_state;
    }
    InternalTokenRewriterResult lib_ruby_parser__internal__containers__token_rewriter__into_internal(TokenRewriterResult_BLOB blob)
    {
        TokenRewriterResult input = UNPACK_TokenRewriterResult(blob);
        InternalTokenRewriterResult output = {
            .token_action = PACK_RewriteAction(input.token_action),
            .lex_state_action = PACK_LexStateAction(input.lex_state_action),
            .rewritten_token = PACK_Ptr(Ptr((int *)(input.rewritten_token.release())))};
        return output;
    }
    void lib_ruby_parser__internal__containers__token_rewriter__drop(TokenRewriterResult_BLOB *blob)
    {
        TokenRewriterResult *result = (TokenRewriterResult *)blob;
        result->~TokenRewriterResult();
    }
}
