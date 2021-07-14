#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_COMMENT_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_COMMENT_HPP

#include "declare_dummy_struct.hpp"
#include "declare_blob.hpp"
#include "declare_list.hpp"

enum class CommentType
{
    DOCUMENT,
    INLINE,
    UNKNOWN,
};
DECLARE_BLOB_FOR(CommentType);
_Static_assert(sizeof(CommentType) == 4, "sizeof(CommentType) == 4");

extern "C"
{
    CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__make_inline();
    CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__make_document();
    CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__make_unknown();
    bool lib_ruby_parser__internal__containers__comment_type__is_inline(CommentType_BLOB blob);
    bool lib_ruby_parser__internal__containers__comment_type__is_document(CommentType_BLOB blob);
    bool lib_ruby_parser__internal__containers__comment_type__is_unknown(CommentType_BLOB blob);
}

DECLARE_DUMMY_STRUCT(Comment, 20);
DECLARE_BLOB_FOR(Comment);

DECLARE_LIST_OF(Comment_BLOB, LIST_OF_Comment);
DECLARE_BLOB_FOR(LIST_OF_Comment);
_Static_assert(sizeof(LIST_OF_Comment) == 24, "sizeof(LIST_OF_Comment) == 24");

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_COMMENT_HPP
