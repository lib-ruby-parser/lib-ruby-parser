#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_LIST_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_LIST_HPP

extern "C" typedef void(DropPtrInPlace)(void *);

// List<T>

#define DECLARE_LIST_API(VALUE_BLOB, LIST_BLOB, PREFIX)                                          \
    extern "C"                                                                                   \
    {                                                                                            \
        LIST_BLOB lib_ruby_parser_containers_##PREFIX##_list_blob_new() noexcept;                \
                                                                                                 \
        LIST_BLOB lib_ruby_parser_containers_##PREFIX##_list_blob_with_capacity(                 \
            uint64_t capacity) noexcept;                                                         \
                                                                                                 \
        LIST_BLOB lib_ruby_parser_containers_##PREFIX##_list_blob_from_raw(                      \
            VALUE_BLOB *ptr, uint64_t size) noexcept;                                            \
                                                                                                 \
        LIST_BLOB lib_ruby_parser_containers_##PREFIX##_list_blob_push(                          \
            LIST_BLOB blob, VALUE_BLOB item) noexcept;                                           \
                                                                                                 \
        typedef struct                                                                           \
        {                                                                                        \
            LIST_BLOB new_blob;                                                                  \
            VALUE_BLOB removed_item;                                                             \
        } LIST_OF_##PREFIX##_REMOVE_RESULT;                                                      \
                                                                                                 \
        LIST_OF_##PREFIX##_REMOVE_RESULT lib_ruby_parser_containers_##PREFIX##_list_blob_remove( \
            LIST_BLOB blob, uint64_t index) noexcept;                                            \
                                                                                                 \
        LIST_BLOB lib_ruby_parser_containers_##PREFIX##_list_blob_shrink_to_fit(                 \
            LIST_BLOB blob) noexcept;                                                            \
                                                                                                 \
        VALUE_BLOB *lib_ruby_parser_containers_##PREFIX##_list_blob_as_ptr(                      \
            LIST_BLOB blob) noexcept;                                                            \
                                                                                                 \
        uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_len(                            \
            LIST_BLOB blob) noexcept;                                                            \
                                                                                                 \
        uint64_t lib_ruby_parser_containers_##PREFIX##_list_blob_capacity(                       \
            LIST_BLOB blob) noexcept;                                                            \
                                                                                                 \
        void lib_ruby_parser_containers_##PREFIX##_list_blob_free(                               \
            LIST_BLOB blob, DropPtrInPlace drop_ptr_in_place) noexcept;                          \
    }

#include "byte.hpp"
DECLARE_LIST_API(Byte_BLOB, LIST_OF_Byte_BLOB, byte);

#include "token.hpp"
DECLARE_LIST_API(Token_BLOB, LIST_OF_Token_BLOB, token);

#include "node.hpp"
DECLARE_LIST_API(Node_BLOB, LIST_OF_Node_BLOB, node);

#include "diagnostic.hpp"
DECLARE_LIST_API(Diagnostic_BLOB, LIST_OF_Diagnostic_BLOB, diagnostic);

#include "comment.hpp"
DECLARE_LIST_API(Comment_BLOB, LIST_OF_Comment_BLOB, comment);

#include "magic_comment.hpp"
DECLARE_LIST_API(MagicComment_BLOB, LIST_OF_MagicComment_BLOB, magic_comment);

#include "source_line.hpp"
DECLARE_LIST_API(SourceLine_BLOB, LIST_OF_SourceLine_BLOB, source_line);

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_LIST_HPP
