#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_COMMENT_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_COMMENT_HPP

#include "declare_dummy_struct.hpp"
#include "declare_blob.hpp"
#include "declare_list.hpp"
#include "loc.hpp"

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

class Comment
{
public:
    Comment(Loc location, CommentType kind);
    Loc location;
    CommentType kind;
};
DECLARE_BLOB_FOR(Comment);

extern "C"
{
    Loc_BLOB *lib_ruby_parser__internal__containers__comment__get_location(Comment_BLOB *blob);
    CommentType_BLOB *lib_ruby_parser__internal__containers__comment__get_kind(Comment_BLOB *blob);
    Comment_BLOB lib_ruby_parser__internal__containers__comment__make(Loc_BLOB location, CommentType_BLOB kind);
}

DECLARE_LIST_OF(Comment_BLOB, LIST_OF_Comment);
DECLARE_BLOB_FOR(LIST_OF_Comment);
_Static_assert(sizeof(LIST_OF_Comment) == 24, "sizeof(LIST_OF_Comment) == 24");

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_COMMENT_HPP
