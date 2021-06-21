#ifndef LIB_RUBY_PARSER_EXTERNAL_C_LIST_H
#define LIB_RUBY_PARSER_EXTERNAL_C_LIST_H

#include <stddef.h>
#include <stdlib.h>
#include <string.h>

typedef void(DropPtrInPlace)(void *);

// List<T>

#define DECLARE_LIST_API(VALUE_BLOB, LIST_BLOB, PREFIX)                                            \
    LIST_BLOB lib_ruby_parser__internal__containers__list__##PREFIX##__new();                      \
                                                                                                   \
    LIST_BLOB lib_ruby_parser__internal__containers__list__##PREFIX##__with_capacity(              \
        uint64_t capacity);                                                                        \
                                                                                                   \
    LIST_BLOB lib_ruby_parser__internal__containers__list__##PREFIX##__from_raw(                   \
        VALUE_BLOB *ptr, uint64_t size);                                                           \
                                                                                                   \
    LIST_BLOB lib_ruby_parser__internal__containers__list__##PREFIX##__push(                       \
        LIST_BLOB blob, VALUE_BLOB item);                                                          \
                                                                                                   \
    typedef struct                                                                                 \
    {                                                                                              \
        LIST_BLOB new_blob;                                                                        \
        VALUE_BLOB removed_item;                                                                   \
    } LIST_##PREFIX##_REMOVE_RESULT;                                                               \
                                                                                                   \
    LIST_##PREFIX##_REMOVE_RESULT lib_ruby_parser__internal__containers__list__##PREFIX##__remove( \
        LIST_BLOB blob, uint64_t index);                                                           \
                                                                                                   \
    LIST_BLOB lib_ruby_parser__internal__containers__list__##PREFIX##__shrink_to_fit(              \
        LIST_BLOB blob);                                                                           \
                                                                                                   \
    VALUE_BLOB *lib_ruby_parser__internal__containers__list__##PREFIX##__as_ptr(                   \
        LIST_BLOB blob);                                                                           \
                                                                                                   \
    uint64_t lib_ruby_parser__internal__containers__list__##PREFIX##__len(                         \
        LIST_BLOB blob);                                                                           \
                                                                                                   \
    uint64_t lib_ruby_parser__internal__containers__list__##PREFIX##__capacity(                    \
        LIST_BLOB blob);                                                                           \
                                                                                                   \
    void lib_ruby_parser__internal__containers__list__##PREFIX##__free(                            \
        LIST_BLOB blob, DropPtrInPlace drop_ptr_in_place);

#include "byte.h"
DECLARE_LIST_API(Byte_BLOB, LIST_OF_Byte_BLOB, of_bytes);

#include "token.h"
DECLARE_LIST_API(Token_BLOB, LIST_OF_Token_BLOB, of_tokens);

#include "node.h"
DECLARE_LIST_API(Node_BLOB, LIST_OF_Node_BLOB, of_nodes);

#include "diagnostic.h"
DECLARE_LIST_API(Diagnostic_BLOB, LIST_OF_Diagnostic_BLOB, of_diagnostics);

#include "comment.h"
DECLARE_LIST_API(Comment_BLOB, LIST_OF_Comment_BLOB, of_comments);

#include "magic_comment.h"
DECLARE_LIST_API(MagicComment_BLOB, LIST_OF_MagicComment_BLOB, of_magic_comments);

#include "source_line.h"
DECLARE_LIST_API(SourceLine_BLOB, LIST_OF_SourceLine_BLOB, of_source_lines);

#endif // LIB_RUBY_PARSER_EXTERNAL_C_LIST_H
