#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_LIST_H
#define LIB_RUBY_PARSER_EXTERNAL_CPP_LIST_H

#include <stdlib.h>
#include "declare_blob.hpp"
#include "list.hpp"

extern "C" typedef void(DropPtrInPlace)(void *);

// List<T>

#define DECLARE_LIST_IMPL(VALUE, VALUE_BLOB, LIST, LIST_BLOB, PREFIX)                                                                    \
    extern "C"                                                                                                                           \
    {                                                                                                                                    \
        LIST_BLOB lib_ruby_parser_containers_##PREFIX##_list_blob_new() noexcept                                                         \
        {                                                                                                                                \
            return PACK(LIST());                                                                                                         \
        }                                                                                                                                \
                                                                                                                                         \
        LIST_BLOB lib_ruby_parser_containers_##PREFIX##_list_blob_with_capacity(uint64_t capacity) noexcept                              \
        {                                                                                                                                \
            LIST list;                                                                                                                   \
            list.reserve(capacity);                                                                                                      \
            return PACK(std::move(list));                                                                                                \
        }                                                                                                                                \
                                                                                                                                         \
        LIST_BLOB lib_ruby_parser_containers_##PREFIX##_list_blob_from_raw(VALUE_BLOB *ptr, uint64_t size) noexcept                      \
        {                                                                                                                                \
            if (size > 0)                                                                                                                \
            {                                                                                                                            \
                auto list = LIST(ptr, ptr + size);                                                                                       \
                free(ptr);                                                                                                               \
                return PACK(std::move(list));                                                                                            \
            }                                                                                                                            \
            else                                                                                                                         \
            {                                                                                                                            \
                return lib_ruby_parser_containers_##PREFIX##_list_blob_new();                                                            \
            }                                                                                                                            \
        }                                                                                                                                \
                                                                                                                                         \
        LIST_BLOB lib_ruby_parser_containers_##PREFIX##_list_blob_push(LIST_BLOB blob, VALUE_BLOB item) noexcept                         \
        {                                                                                                                                \
            LIST list = UNPACK(blob);                                                                                                    \
            list.push_back(item);                                                                                                        \
            return PACK(std::move(list));                                                                                                \
        }                                                                                                                                \
                                                                                                                                         \
        LIST_OF_##PREFIX##_REMOVE_RESULT lib_ruby_parser_containers_##PREFIX##_list_blob_remove(LIST_BLOB blob, uint64_t index) noexcept \
        {                                                                                                                                \
            LIST list = UNPACK(blob);                                                                                                    \
            VALUE_BLOB item = std::move(list[index]);                                                                                    \
            list.erase(list.begin() + index);                                                                                            \
            LIST_OF_##PREFIX##_REMOVE_RESULT result = {                                                                                  \
                .new_blob = PACK(std::move(list)),                                                                                       \
                .removed_item = item};                                                                                                   \
                                                                                                                                         \
            return result;                                                                                                               \
        }                                                                                                                                \
                                                                                                                                         \
        LIST_BLOB lib_ruby_parser_containers_##PREFIX##_list_blob_shrink_to_fit(LIST_BLOB blob) noexcept                                 \
        {                                                                                                                                \
            LIST list = UNPACK(blob);                                                                                                    \
            list.shrink_to_fit();                                                                                                        \
            return PACK(std::move(list));                                                                                                \
        }                                                                                                                                \
                                                                                                                                         \
        VALUE_BLOB *lib_ruby_parser_containers_##PREFIX##_list_blob_as_ptr(LIST_BLOB blob) noexcept                                      \
        {                                                                                                                                \
            LIST list = UNPACK(blob);                                                                                                    \
            auto result = list.data();                                                                                                   \
            PACK(std::move(list));                                                                                                       \
            return result;                                                                                                               \
        }                                                                                                                                \
                                                                                                                                         \
        uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_len(LIST_BLOB blob) noexcept                                            \
        {                                                                                                                                \
            LIST list = UNPACK(blob);                                                                                                    \
            auto result = list.size();                                                                                                   \
            PACK(std::move(list));                                                                                                       \
            return result;                                                                                                               \
        }                                                                                                                                \
                                                                                                                                         \
        uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_capacity(LIST_BLOB blob) noexcept                                       \
        {                                                                                                                                \
            LIST list = UNPACK(blob);                                                                                                    \
            auto result = list.capacity();                                                                                               \
            PACK(std::move(list));                                                                                                       \
            return result;                                                                                                               \
        }                                                                                                                                \
                                                                                                                                         \
        void lib_ruby_parser_containers_##PREFIX##_list_blob_free(                                                                       \
            LIST_BLOB blob, DropPtrInPlace drop_ptr_in_place) noexcept                                                                   \
        {                                                                                                                                \
            LIST list = UNPACK(blob);                                                                                                    \
            for (size_t i = 0; i < list.size(); i++)                                                                                     \
            {                                                                                                                            \
                drop_ptr_in_place(&list[i]);                                                                                             \
            }                                                                                                                            \
        }                                                                                                                                \
    }

DECLARE_LIST_IMPL(Byte, Byte_BLOB, LIST_OF_Byte, LIST_OF_Byte_BLOB, byte);
DECLARE_LIST_IMPL(Node, Node_BLOB, LIST_OF_Node, LIST_OF_Node_BLOB, node);
DECLARE_LIST_IMPL(Diagnostic, Diagnostic_BLOB, LIST_OF_Diagnostic, LIST_OF_Diagnostic_BLOB, diagnostic);
DECLARE_LIST_IMPL(Comment, Comment_BLOB, LIST_OF_Comment, LIST_OF_Comment_BLOB, comment);
DECLARE_LIST_IMPL(MagicComment, MagicComment_BLOB, LIST_OF_MagicComment, LIST_OF_MagicComment_BLOB, magic_comment);
DECLARE_LIST_IMPL(Token, Token_BLOB, LIST_OF_Token, LIST_OF_Token_BLOB, token);
DECLARE_LIST_IMPL(SourceLine, SourceLine_BLOB, LIST_OF_SourceLine, LIST_OF_SourceLine_BLOB, source_line);

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_LIST_H
