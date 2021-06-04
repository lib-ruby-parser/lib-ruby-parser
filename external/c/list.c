#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#include "declare_blob.h"
#include "list.h"

// List<T>

#define DECLARE_LIST_IMPL(VALUE, VALUE_BLOB_DATA, LIST, LIST_BLOB_DATA, PREFIX)                                                  \
    LIST_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_new()                                                         \
    {                                                                                                                            \
        LIST list = {.ptr = NULL, .size = 0, .capacity = 0};                                                                     \
        return PACK_##LIST(list);                                                                                                \
    }                                                                                                                            \
                                                                                                                                 \
    LIST_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_with_capacity(uint64_t capacity)                              \
    {                                                                                                                            \
        LIST list = {.ptr = malloc(sizeof(VALUE) * capacity), .size = 0, .capacity = capacity};                                  \
        return PACK_##LIST(list);                                                                                                \
    }                                                                                                                            \
                                                                                                                                 \
    LIST_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_from_raw(VALUE_BLOB_DATA *ptr, uint64_t size)                 \
    {                                                                                                                            \
        if (size > 0)                                                                                                            \
        {                                                                                                                        \
            LIST list = {.ptr = ptr, .size = size, .capacity = size};                                                            \
            return PACK_##LIST(list);                                                                                            \
        }                                                                                                                        \
        else                                                                                                                     \
        {                                                                                                                        \
            return lib_ruby_parser_containers_##PREFIX##_list_blob_new();                                                        \
        }                                                                                                                        \
    }                                                                                                                            \
                                                                                                                                 \
    LIST_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_push(LIST_BLOB_DATA blob, VALUE_BLOB_DATA item)               \
    {                                                                                                                            \
        LIST list = UNPACK_##LIST(blob);                                                                                         \
        if (list.size + 1 > list.capacity)                                                                                       \
        {                                                                                                                        \
            if (list.capacity == 0)                                                                                              \
            {                                                                                                                    \
                list.capacity += 1;                                                                                              \
            }                                                                                                                    \
            else                                                                                                                 \
            {                                                                                                                    \
                list.capacity *= 2;                                                                                              \
            }                                                                                                                    \
            VALUE_BLOB_DATA *old_ptr = list.ptr;                                                                                 \
            VALUE_BLOB_DATA *new_ptr = malloc(sizeof(VALUE) * list.capacity);                                                    \
            memcpy(new_ptr, old_ptr, sizeof(VALUE) * list.size);                                                                 \
            list.ptr = new_ptr;                                                                                                  \
            free(old_ptr);                                                                                                       \
        }                                                                                                                        \
        list.ptr[list.size] = item;                                                                                              \
        list.size++;                                                                                                             \
        return PACK_##LIST(list);                                                                                                \
    }                                                                                                                            \
                                                                                                                                 \
    LIST_OF_##PREFIX##_REMOVE_RESULT lib_ruby_parser_containers_##PREFIX##_list_blob_remove(LIST_BLOB_DATA blob, uint64_t index) \
    {                                                                                                                            \
        LIST list = UNPACK_##LIST(blob);                                                                                         \
        VALUE_BLOB_DATA item = list.ptr[index];                                                                                  \
        memmove(list.ptr + index, list.ptr + index + 1, sizeof(VALUE) * (list.size - index - 1));                                \
        list.size--;                                                                                                             \
        LIST_OF_##PREFIX##_REMOVE_RESULT result = {                                                                              \
            .new_blob = PACK_##LIST(list),                                                                                       \
            .removed_item = item};                                                                                               \
                                                                                                                                 \
        return result;                                                                                                           \
    }                                                                                                                            \
                                                                                                                                 \
    LIST_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_shrink_to_fit(LIST_BLOB_DATA blob)                            \
    {                                                                                                                            \
        LIST list = UNPACK_##LIST(blob);                                                                                         \
                                                                                                                                 \
        uint64_t new_size = list.size;                                                                                           \
        uint64_t new_capacity = list.size;                                                                                       \
                                                                                                                                 \
        VALUE_BLOB_DATA *new_ptr = malloc(sizeof(VALUE) * new_capacity);                                                         \
        memcpy(new_ptr, list.ptr, sizeof(VALUE) * new_size);                                                                     \
                                                                                                                                 \
        VALUE_BLOB_DATA *old_ptr = list.ptr;                                                                                     \
        list.ptr = new_ptr;                                                                                                      \
        list.size = new_size;                                                                                                    \
        list.capacity = new_capacity;                                                                                            \
        free(old_ptr);                                                                                                           \
                                                                                                                                 \
        return PACK_##LIST(list);                                                                                                \
    }                                                                                                                            \
                                                                                                                                 \
    VALUE_BLOB_DATA *lib_ruby_parser_containers_##PREFIX##_list_blob_as_ptr(LIST_BLOB_DATA blob)                                 \
    {                                                                                                                            \
        LIST list = UNPACK_##LIST(blob);                                                                                         \
        return list.ptr;                                                                                                         \
    }                                                                                                                            \
                                                                                                                                 \
    uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_len(LIST_BLOB_DATA blob)                                            \
    {                                                                                                                            \
        LIST list = UNPACK_##LIST(blob);                                                                                         \
        return list.size;                                                                                                        \
    }                                                                                                                            \
                                                                                                                                 \
    uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_capacity(LIST_BLOB_DATA blob)                                       \
    {                                                                                                                            \
        LIST list = UNPACK_##LIST(blob);                                                                                         \
        return list.capacity;                                                                                                    \
    }                                                                                                                            \
                                                                                                                                 \
    void lib_ruby_parser_containers_##PREFIX##_list_blob_free(                                                                   \
        LIST_BLOB_DATA blob, DropPtrInPlace drop_ptr_in_place)                                                                   \
    {                                                                                                                            \
        LIST list = UNPACK_##LIST(blob);                                                                                         \
        for (size_t i = 0; i < list.size; i++)                                                                                   \
        {                                                                                                                        \
            drop_ptr_in_place(&list.ptr[i]);                                                                                     \
        }                                                                                                                        \
        free(list.ptr);                                                                                                          \
    }

DECLARE_LIST_IMPL(Byte, Byte_BLOB_DATA, LIST_OF_Byte, LIST_OF_Byte_BLOB_DATA, byte);
DECLARE_LIST_IMPL(Node, Node_BLOB_DATA, LIST_OF_Node, LIST_OF_Node_BLOB_DATA, node);
DECLARE_LIST_IMPL(Diagnostic, Diagnostic_BLOB_DATA, LIST_OF_Diagnostic, LIST_OF_Diagnostic_BLOB_DATA, diagnostic);
DECLARE_LIST_IMPL(Comment, Comment_BLOB_DATA, LIST_OF_Comment, LIST_OF_Comment_BLOB_DATA, comment);
DECLARE_LIST_IMPL(MagicComment, MagicComment_BLOB_DATA, LIST_OF_MagicComment, LIST_OF_MagicComment_BLOB_DATA, magic_comment);
DECLARE_LIST_IMPL(Token, Token_BLOB_DATA, LIST_OF_Token, LIST_OF_Token_BLOB_DATA, token);
DECLARE_LIST_IMPL(SourceLine, SourceLine_BLOB_DATA, LIST_OF_SourceLine, LIST_OF_SourceLine_BLOB_DATA, source_line);
