#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_LIST_H
#define LIB_RUBY_PARSER_EXTERNAL_CPP_LIST_H

#include "declare_blob.hpp"

extern "C" typedef void(DropPtrInPlace)(void *);

// List<T>

#define DECLARE_BLOB_FOR_LIST_OF(VALUE, PREFIX)                                                                                                 \
    typedef std::vector<VALUE##_BLOB_DATA> VALUE##List;                                                                                         \
    DECLARE_BLOB_FOR(VALUE##List);                                                                                                              \
    _Static_assert(sizeof(VALUE##List) == 24);                                                                                                  \
                                                                                                                                                \
    VALUE##List lib_ruby_parser_containers_##PREFIX##_unpack_blob(VALUE##List_BLOB_DATA blob) noexcept                                          \
    {                                                                                                                                           \
        VALUE##List_BLOB_UNION u = {.as_blob = blob};                                                                                           \
        return std::move(u.as_value);                                                                                                           \
    }                                                                                                                                           \
                                                                                                                                                \
    VALUE##List_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_pack_blob(VALUE##List list) noexcept                                            \
    {                                                                                                                                           \
        VALUE##List_BLOB_UNION u = {.as_value = std::move(list)};                                                                               \
        return u.as_blob;                                                                                                                       \
    }                                                                                                                                           \
                                                                                                                                                \
    extern "C"                                                                                                                                  \
    {                                                                                                                                           \
        VALUE##List_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_new() noexcept                                                    \
        {                                                                                                                                       \
            return lib_ruby_parser_containers_##PREFIX##_pack_blob(VALUE##List());                                                              \
        }                                                                                                                                       \
                                                                                                                                                \
        VALUE##List_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_with_capacity(uint64_t capacity) noexcept                         \
        {                                                                                                                                       \
            VALUE##List list;                                                                                                                   \
            list.reserve(capacity);                                                                                                             \
            return lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(list));                                                            \
        }                                                                                                                                       \
                                                                                                                                                \
        VALUE##List_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_from_raw(VALUE##_BLOB_DATA *ptr, uint64_t size) noexcept          \
        {                                                                                                                                       \
            if (size > 0)                                                                                                                       \
            {                                                                                                                                   \
                auto list = VALUE##List(ptr, ptr + size);                                                                                       \
                free(ptr);                                                                                                                      \
                return lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(list));                                                        \
            }                                                                                                                                   \
            else                                                                                                                                \
            {                                                                                                                                   \
                return lib_ruby_parser_containers_##PREFIX##_list_blob_new();                                                                   \
            }                                                                                                                                   \
        }                                                                                                                                       \
                                                                                                                                                \
        VALUE##List_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_push(VALUE##List_BLOB_DATA blob, VALUE##_BLOB_DATA item) noexcept \
        {                                                                                                                                       \
            VALUE##List list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                         \
            list.push_back(item);                                                                                                               \
            return lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(list));                                                            \
        }                                                                                                                                       \
                                                                                                                                                \
        typedef struct                                                                                                                          \
        {                                                                                                                                       \
            VALUE##List_BLOB_DATA new_blob;                                                                                                     \
            VALUE##_BLOB_DATA removed_item;                                                                                                     \
        } VALUE##List_REMOVE_RESULT;                                                                                                            \
                                                                                                                                                \
        VALUE##List_REMOVE_RESULT lib_ruby_parser_containers_##PREFIX##_list_blob_remove(VALUE##List_BLOB_DATA blob, uint64_t index) noexcept   \
        {                                                                                                                                       \
            VALUE##List list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                         \
            VALUE##_BLOB_DATA item = std::move(list[index]);                                                                                    \
            list.erase(list.begin() + index);                                                                                                   \
            VALUE##List_REMOVE_RESULT result = {                                                                                                \
                .new_blob = lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(list)),                                                   \
                .removed_item = item};                                                                                                          \
                                                                                                                                                \
            return result;                                                                                                                      \
        }                                                                                                                                       \
                                                                                                                                                \
        VALUE##List_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_shrink_to_fit(VALUE##List_BLOB_DATA blob) noexcept                \
        {                                                                                                                                       \
            VALUE##List list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                         \
            list.shrink_to_fit();                                                                                                               \
            return lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(list));                                                            \
        }                                                                                                                                       \
                                                                                                                                                \
        VALUE##_BLOB_DATA *lib_ruby_parser_containers_##PREFIX##_list_blob_as_ptr(VALUE##List_BLOB_DATA blob) noexcept                          \
        {                                                                                                                                       \
            VALUE##List list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                         \
            auto result = list.data();                                                                                                          \
            lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(list));                                                                   \
            return result;                                                                                                                      \
        }                                                                                                                                       \
                                                                                                                                                \
        uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_len(VALUE##List_BLOB_DATA blob) noexcept                                       \
        {                                                                                                                                       \
            VALUE##List list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                         \
            auto result = list.size();                                                                                                          \
            lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(list));                                                                   \
            return result;                                                                                                                      \
        }                                                                                                                                       \
                                                                                                                                                \
        uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_capacity(VALUE##List_BLOB_DATA blob) noexcept                                  \
        {                                                                                                                                       \
            VALUE##List list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                         \
            auto result = list.capacity();                                                                                                      \
            lib_ruby_parser_containers_##PREFIX##_pack_blob(std::move(list));                                                                   \
            return result;                                                                                                                      \
        }                                                                                                                                       \
                                                                                                                                                \
        void lib_ruby_parser_containers_##PREFIX##_list_blob_free(                                                                              \
            VALUE##List_BLOB_DATA blob, DropPtrInPlace drop_ptr_in_place) noexcept                                                              \
        {                                                                                                                                       \
            VALUE##List list = lib_ruby_parser_containers_##PREFIX##_unpack_blob(blob);                                                         \
            for (size_t i = 0; i < list.size(); i++)                                                                                            \
            {                                                                                                                                   \
                drop_ptr_in_place(&list[i]);                                                                                                    \
            }                                                                                                                                   \
        }                                                                                                                                       \
    }

#include "types/node.hpp"
DECLARE_BLOB_FOR(Node);
DECLARE_BLOB_FOR_LIST_OF(Node, node);

#include "types/diagnostic.hpp"
DECLARE_BLOB_FOR(Diagnostic);
DECLARE_BLOB_FOR_LIST_OF(Diagnostic, diagnostic);

#include "types/comment.hpp"
DECLARE_BLOB_FOR(Comment);
DECLARE_BLOB_FOR_LIST_OF(Comment, comment);

#include "types/magic_comment.hpp"
DECLARE_BLOB_FOR(MagicComment);
DECLARE_BLOB_FOR_LIST_OF(MagicComment, magic_comment);

#include "types/token.hpp"
DECLARE_BLOB_FOR(Token);
DECLARE_BLOB_FOR_LIST_OF(Token, token);

#include "types/source_line.hpp"
DECLARE_BLOB_FOR(SourceLine);
DECLARE_BLOB_FOR_LIST_OF(SourceLine, source_line);

typedef uint8_t Byte;
DECLARE_BLOB_FOR(Byte);
DECLARE_BLOB_FOR_LIST_OF(Byte, byte);

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_LIST_H
