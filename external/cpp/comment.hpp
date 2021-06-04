#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_COMMENT_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_COMMENT_HPP

#include "declare_dummy_struct.hpp"
#include "declare_blob.hpp"
#include "declare_list.hpp"

DECLARE_DUMMY_STRUCT(Comment, 24);
DECLARE_BLOB_FOR(Comment);

DECLARE_LIST_OF(Comment_BLOB_DATA, LIST_OF_Comment);
DECLARE_BLOB_FOR(LIST_OF_Comment);
_Static_assert(sizeof(LIST_OF_Comment) == 24, "sizeof(LIST_OF_Comment) == 24");

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_COMMENT_HPP
