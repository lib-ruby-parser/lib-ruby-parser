#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_LIST_H
#define LIB_RUBY_PARSER_EXTERNAL_CPP_LIST_H

#include <stdlib.h>
#include "declare_blob.hpp"
#include "list.hpp"

extern "C" typedef void(DropPtrInPlace)(void *);

// List<T>

#define DECLARE_LIST_IMPL(VALUE, VALUE_BLOB_DATA, LIST, LIST_BLOB_DATA, PREFIX)                                                               \
    LIST lib_ruby_parser_containers_##PREFIX##_unpack_blob(LIST_BLOB_DATA blob) noexcept                                                      \
    {                                                                                                                                         \
        LIST##_BLOB_UNION u = {.as_blob = blob};                                                                                              \
        return std::move(u.as_value);                                                                                                         \
    }                                                                                                                                         \
                                                                                                                                              \
    LIST_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_pack_blob(LIST list) noexcept                                                        \
    {                                                                                                                                         \
        LIST##_BLOB_UNION u = {.as_value = std::move(list)};                                                                                  \
        return u.as_blob;                                                                                                                     \
    }                                                                                                                                         \
                                                                                                                                              \
    extern "C"                                                                                                                                \
    {                                                                                                                                         \
        LIST_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_new() noexcept                                                         \
        {                                                                                                                                     \
            return lib_ruby_parser_containers_##PREFIX##_pack_blob(LIST());                                                                   \
        }                                                                                                                                     \
                                                                                                                                              \
        LIST_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_with_capacity(uint64_t capacity) noexcept                              \
        {                                                                                                                                     \
            LIST list;                                                                                                                        \
            list.reserve(capacity);                                                                                                           \
            return lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(list));                                                          \
        }                                                                                                                                     \
                                                                                                                                              \
        LIST_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_from_raw(VALUE_BLOB_DATA *ptr, uint64_t size) noexcept                 \
        {                                                                                                                                     \
            if (size > 0)                                                                                                                     \
            {                                                                                                                                 \
                auto list = LIST(ptr, ptr + size);                                                                                            \
                free(ptr);                                                                                                                    \
                return lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(list));                                                      \
            }                                                                                                                                 \
            else                                                                                                                              \
            {                                                                                                                                 \
                return lib_ruby_parser_containers_##PREFIX##_list_blob_new();                                                                 \
            }                                                                                                                                 \
        }                                                                                                                                     \
                                                                                                                                              \
        LIST_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_push(LIST_BLOB_DATA blob, VALUE_BLOB_DATA item) noexcept               \
        {                                                                                                                                     \
            LIST list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                              \
            list.push_back(item);                                                                                                             \
            return lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(list));                                                          \
        }                                                                                                                                     \
                                                                                                                                              \
        LIST_OF_##PREFIX##_REMOVE_RESULT lib_ruby_parser_containers_##PREFIX##_list_blob_remove(LIST_BLOB_DATA blob, uint64_t index) noexcept \
        {                                                                                                                                     \
            LIST list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                              \
            VALUE_BLOB_DATA item = std::move(list[index]);                                                                                    \
            list.erase(list.begin() + index);                                                                                                 \
            LIST_OF_##PREFIX##_REMOVE_RESULT result = {                                                                                       \
                .new_blob = lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(list)),                                                 \
                .removed_item = item};                                                                                                        \
                                                                                                                                              \
            return result;                                                                                                                    \
        }                                                                                                                                     \
                                                                                                                                              \
        LIST_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_shrink_to_fit(LIST_BLOB_DATA blob) noexcept                            \
        {                                                                                                                                     \
            LIST list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                              \
            list.shrink_to_fit();                                                                                                             \
            return lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(list));                                                          \
        }                                                                                                                                     \
                                                                                                                                              \
        VALUE_BLOB_DATA *lib_ruby_parser_containers_##PREFIX##_list_blob_as_ptr(LIST_BLOB_DATA blob) noexcept                                 \
        {                                                                                                                                     \
            LIST list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                              \
            auto result = list.data();                                                                                                        \
            lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(list));                                                                 \
            return result;                                                                                                                    \
        }                                                                                                                                     \
                                                                                                                                              \
        uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_len(LIST_BLOB_DATA blob) noexcept                                            \
        {                                                                                                                                     \
            LIST list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                              \
            auto result = list.size();                                                                                                        \
            lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(list));                                                                 \
            return result;                                                                                                                    \
        }                                                                                                                                     \
                                                                                                                                              \
        uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_capacity(LIST_BLOB_DATA blob) noexcept                                       \
        {                                                                                                                                     \
            LIST list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                              \
            auto result = list.capacity();                                                                                                    \
            lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(list));                                                                 \
            return result;                                                                                                                    \
        }                                                                                                                                     \
                                                                                                                                              \
        void lib_ruby_parser_containers_##PREFIX##_list_blob_free(                                                                            \
            LIST_BLOB_DATA blob, DropPtrInPlace drop_ptr_in_place) noexcept                                                                   \
        {                                                                                                                                     \
            LIST list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                              \
            for (size_t i = 0; i < list.size(); i++)                                                                                          \
            {                                                                                                                                 \
                drop_ptr_in_place(&list[i]);                                                                                                  \
            }                                                                                                                                 \
        }                                                                                                                                     \
    }

DECLARE_LIST_IMPL(Byte, Byte_BLOB_DATA, LIST_OF_Byte, LIST_OF_Byte_BLOB_DATA, byte);
DECLARE_LIST_IMPL(Node, Node_BLOB_DATA, LIST_OF_Node, LIST_OF_Node_BLOB_DATA, node);
DECLARE_LIST_IMPL(Diagnostic, Diagnostic_BLOB_DATA, LIST_OF_Diagnostic, LIST_OF_Diagnostic_BLOB_DATA, diagnostic);
DECLARE_LIST_IMPL(Comment, Comment_BLOB_DATA, LIST_OF_Comment, LIST_OF_Comment_BLOB_DATA, comment);
DECLARE_LIST_IMPL(MagicComment, MagicComment_BLOB_DATA, LIST_OF_MagicComment, LIST_OF_MagicComment_BLOB_DATA, magic_comment);
DECLARE_LIST_IMPL(Token, Token_BLOB_DATA, LIST_OF_Token, LIST_OF_Token_BLOB_DATA, token);
DECLARE_LIST_IMPL(SourceLine, SourceLine_BLOB_DATA, LIST_OF_SourceLine, LIST_OF_SourceLine_BLOB_DATA, source_line);

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_LIST_H
