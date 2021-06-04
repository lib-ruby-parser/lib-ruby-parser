#ifndef LIB_RUBY_PARSER_EXTERNAL_C_LIST_H
#define LIB_RUBY_PARSER_EXTERNAL_C_LIST_H

#include <stddef.h>
#include <stdlib.h>
#include <string.h>

typedef void(DropPtrInPlace)(void *);

// List<T>

#define DECLARE_LIST_API(VALUE_BLOB_DATA, LIST_BLOB_DATA, PREFIX)                            \
    LIST_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_new();                    \
                                                                                             \
    LIST_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_with_capacity(            \
        uint64_t capacity);                                                                  \
                                                                                             \
    LIST_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_from_raw(                 \
        VALUE_BLOB_DATA *ptr, uint64_t size);                                                \
                                                                                             \
    LIST_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_push(                     \
        LIST_BLOB_DATA blob, VALUE_BLOB_DATA item);                                          \
                                                                                             \
    typedef struct                                                                           \
    {                                                                                        \
        LIST_BLOB_DATA new_blob;                                                             \
        VALUE_BLOB_DATA removed_item;                                                        \
    } LIST_OF_##PREFIX##_REMOVE_RESULT;                                                      \
                                                                                             \
    LIST_OF_##PREFIX##_REMOVE_RESULT lib_ruby_parser_containers_##PREFIX##_list_blob_remove( \
        LIST_BLOB_DATA blob, uint64_t index);                                                \
                                                                                             \
    LIST_BLOB_DATA lib_ruby_parser_containers_##PREFIX##_list_blob_shrink_to_fit(            \
        LIST_BLOB_DATA blob);                                                                \
                                                                                             \
    VALUE_BLOB_DATA *lib_ruby_parser_containers_##PREFIX##_list_blob_as_ptr(                 \
        LIST_BLOB_DATA blob);                                                                \
                                                                                             \
    uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_len(                            \
        LIST_BLOB_DATA blob);                                                                \
                                                                                             \
    uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_capacity(                       \
        LIST_BLOB_DATA blob);                                                                \
                                                                                             \
    void lib_ruby_parser_containers_##PREFIX##_list_blob_free(                               \
        LIST_BLOB_DATA blob, DropPtrInPlace drop_ptr_in_place);

#include "byte.h"
DECLARE_LIST_API(Byte_BLOB_DATA, LIST_OF_Byte_BLOB_DATA, byte);

#include "token.h"
DECLARE_LIST_API(Token_BLOB_DATA, LIST_OF_Token_BLOB_DATA, token);

#include "node.h"
DECLARE_LIST_API(Node_BLOB_DATA, LIST_OF_Node_BLOB_DATA, node);

#include "diagnostic.h"
DECLARE_LIST_API(Diagnostic_BLOB_DATA, LIST_OF_Diagnostic_BLOB_DATA, diagnostic);

#include "comment.h"
DECLARE_LIST_API(Comment_BLOB_DATA, LIST_OF_Comment_BLOB_DATA, comment);

#include "magic_comment.h"
DECLARE_LIST_API(MagicComment_BLOB_DATA, LIST_OF_MagicComment_BLOB_DATA, magic_comment);

#include "source_line.h"
DECLARE_LIST_API(SourceLine_BLOB_DATA, LIST_OF_SourceLine_BLOB_DATA, source_line);

#endif // LIB_RUBY_PARSER_EXTERNAL_C_LIST_H
