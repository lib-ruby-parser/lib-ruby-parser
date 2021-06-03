#ifndef LIB_RUBY_PARSER_EXTERNAL_C_LIST_H
#define LIB_RUBY_PARSER_EXTERNAL_C_LIST_H

#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#include "declare_blob.h"

typedef void(DropPtrInPlace)(void *);

// List<T>

#define DECLARE_BLOB_FOR_LIST_OF(VALUE, PREFIX)                                                                                                \
    typedef struct                                                                                                                             \
    {                                                                                                                                          \
        VALUE##_BLOB_DATA *ptr;                                                                                                                \
        uint64_t size;                                                                                                                         \
        uint64_t capacity;                                                                                                                     \
    } LIST_OF_##VALUE;                                                                                                                         \
    DECLARE_BLOB_FOR(LIST_OF_##VALUE);                                                                                                         \
    _Static_assert(sizeof(LIST_OF_##VALUE) == 24, "sizeof(List) == 24");                                                                       \
                                                                                                                                               \
    LIST_OF_##VALUE##_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_new()                                                          \
    {                                                                                                                                          \
        LIST_OF_##VALUE list = {.ptr = NULL, .size = 0, .capacity = 0};                                                                        \
        return PACK_LIST_OF_##VALUE(list);                                                                                                     \
    }                                                                                                                                          \
                                                                                                                                               \
    LIST_OF_##VALUE##_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_with_capacity(uint64_t capacity)                               \
    {                                                                                                                                          \
        LIST_OF_##VALUE list = {.ptr = malloc(sizeof(VALUE) * capacity), .size = 0, .capacity = capacity};                                     \
        return PACK_LIST_OF_##VALUE(list);                                                                                                     \
    }                                                                                                                                          \
                                                                                                                                               \
    LIST_OF_##VALUE##_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_from_raw(VALUE##_BLOB_DATA *ptr, uint64_t size)                \
    {                                                                                                                                          \
        if (size > 0)                                                                                                                          \
        {                                                                                                                                      \
            LIST_OF_##VALUE list = {.ptr = ptr, .size = size, .capacity = size};                                                               \
            return PACK_LIST_OF_##VALUE(list);                                                                                                 \
        }                                                                                                                                      \
        else                                                                                                                                   \
        {                                                                                                                                      \
            return lib_ruby_parser_containers_##PREFIX##_list_blob_new();                                                                      \
        }                                                                                                                                      \
    }                                                                                                                                          \
                                                                                                                                               \
    LIST_OF_##VALUE##_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_push(LIST_OF_##VALUE##_BLOB_DATA blob, VALUE##_BLOB_DATA item) \
    {                                                                                                                                          \
        LIST_OF_##VALUE list = UNPACK_LIST_OF_##VALUE(blob);                                                                                   \
        if (list.size + 1 > list.capacity)                                                                                                     \
        {                                                                                                                                      \
            if (list.capacity == 0)                                                                                                            \
            {                                                                                                                                  \
                list.capacity += 1;                                                                                                            \
            }                                                                                                                                  \
            else                                                                                                                               \
            {                                                                                                                                  \
                list.capacity *= 2;                                                                                                            \
            }                                                                                                                                  \
            VALUE##_BLOB_DATA *old_ptr = list.ptr;                                                                                             \
            VALUE##_BLOB_DATA *new_ptr = malloc(sizeof(VALUE) * list.capacity);                                                                \
            memcpy(new_ptr, old_ptr, sizeof(VALUE) * list.size);                                                                               \
            list.ptr = new_ptr;                                                                                                                \
            free(old_ptr);                                                                                                                     \
        }                                                                                                                                      \
        list.ptr[list.size] = item;                                                                                                            \
        list.size++;                                                                                                                           \
        return PACK_LIST_OF_##VALUE(list);                                                                                                     \
    }                                                                                                                                          \
                                                                                                                                               \
    typedef struct                                                                                                                             \
    {                                                                                                                                          \
        LIST_OF_##VALUE##_BLOB_DATA new_blob;                                                                                                  \
        VALUE##_BLOB_DATA removed_item;                                                                                                        \
    } LIST_OF_##VALUE##_REMOVE_RESULT;                                                                                                         \
                                                                                                                                               \
    LIST_OF_##VALUE##_REMOVE_RESULT lib_ruby_parser_containers_##PREFIX##_list_blob_remove(LIST_OF_##VALUE##_BLOB_DATA blob, uint64_t index)   \
    {                                                                                                                                          \
        LIST_OF_##VALUE list = UNPACK_LIST_OF_##VALUE(blob);                                                                                   \
        VALUE##_BLOB_DATA item = list.ptr[index];                                                                                              \
        memmove(list.ptr + index, list.ptr + index + 1, sizeof(VALUE) * (list.size - index - 1));                                              \
        list.size--;                                                                                                                           \
        LIST_OF_##VALUE##_REMOVE_RESULT result = {                                                                                             \
            .new_blob = PACK_LIST_OF_##VALUE(list),                                                                                            \
            .removed_item = item};                                                                                                             \
                                                                                                                                               \
        return result;                                                                                                                         \
    }                                                                                                                                          \
                                                                                                                                               \
    LIST_OF_##VALUE##_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_shrink_to_fit(LIST_OF_##VALUE##_BLOB_DATA blob)                \
    {                                                                                                                                          \
        LIST_OF_##VALUE list = UNPACK_LIST_OF_##VALUE(blob);                                                                                   \
                                                                                                                                               \
        uint64_t new_size = list.size;                                                                                                         \
        uint64_t new_capacity = list.size;                                                                                                     \
                                                                                                                                               \
        VALUE##_BLOB_DATA *new_ptr = malloc(sizeof(VALUE) * new_capacity);                                                                     \
        memcpy(new_ptr, list.ptr, sizeof(VALUE) * new_size);                                                                                   \
                                                                                                                                               \
        VALUE##_BLOB_DATA *old_ptr = list.ptr;                                                                                                 \
        list.ptr = new_ptr;                                                                                                                    \
        list.size = new_size;                                                                                                                  \
        list.capacity = new_capacity;                                                                                                          \
        free(old_ptr);                                                                                                                         \
                                                                                                                                               \
        return PACK_LIST_OF_##VALUE(list);                                                                                                     \
    }                                                                                                                                          \
                                                                                                                                               \
    VALUE##_BLOB_DATA *lib_ruby_parser_containers_##PREFIX##_list_blob_as_ptr(LIST_OF_##VALUE##_BLOB_DATA blob)                                \
    {                                                                                                                                          \
        LIST_OF_##VALUE list = UNPACK_LIST_OF_##VALUE(blob);                                                                                   \
        return list.ptr;                                                                                                                       \
    }                                                                                                                                          \
                                                                                                                                               \
    uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_len(LIST_OF_##VALUE##_BLOB_DATA blob)                                             \
    {                                                                                                                                          \
        LIST_OF_##VALUE list = UNPACK_LIST_OF_##VALUE(blob);                                                                                   \
        return list.size;                                                                                                                      \
    }                                                                                                                                          \
                                                                                                                                               \
    uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_capacity(LIST_OF_##VALUE##_BLOB_DATA blob)                                        \
    {                                                                                                                                          \
        LIST_OF_##VALUE list = UNPACK_LIST_OF_##VALUE(blob);                                                                                   \
        return list.capacity;                                                                                                                  \
    }                                                                                                                                          \
                                                                                                                                               \
    void lib_ruby_parser_containers_##PREFIX##_list_blob_free(                                                                                 \
        LIST_OF_##VALUE##_BLOB_DATA blob, DropPtrInPlace drop_ptr_in_place)                                                                    \
    {                                                                                                                                          \
        LIST_OF_##VALUE list = UNPACK_LIST_OF_##VALUE(blob);                                                                                   \
        for (size_t i = 0; i < list.size; i++)                                                                                                 \
        {                                                                                                                                      \
            drop_ptr_in_place(&list.ptr[i]);                                                                                                   \
        }                                                                                                                                      \
        free(list.ptr);                                                                                                                        \
    }

#include "types/node.h"
DECLARE_BLOB_FOR(Node);
DECLARE_BLOB_FOR_LIST_OF(Node, node);

#include "types/diagnostic.h"
DECLARE_BLOB_FOR(Diagnostic);
DECLARE_BLOB_FOR_LIST_OF(Diagnostic, diagnostic);

#include "types/comment.h"
DECLARE_BLOB_FOR(Comment);
DECLARE_BLOB_FOR_LIST_OF(Comment, comment);

#include "types/magic_comment.h"
DECLARE_BLOB_FOR(MagicComment);
DECLARE_BLOB_FOR_LIST_OF(MagicComment, magic_comment);

#include "types/token.h"
DECLARE_BLOB_FOR(Token);
DECLARE_BLOB_FOR_LIST_OF(Token, token);

#include "types/source_line.h"
DECLARE_BLOB_FOR(SourceLine);
DECLARE_BLOB_FOR_LIST_OF(SourceLine, source_line);

typedef uint8_t Byte;
DECLARE_BLOB_FOR(Byte);
DECLARE_BLOB_FOR_LIST_OF(Byte, byte);

#endif // LIB_RUBY_PARSER_EXTERNAL_C_LIST_H
