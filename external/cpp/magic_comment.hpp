#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_MAGIC_COMMENT_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_MAGIC_COMMENT_HPP

#include "declare_dummy_struct.hpp"
#include "declare_blob.hpp"
#include "declare_list.hpp"

DECLARE_DUMMY_STRUCT(MagicComment, 40);
DECLARE_BLOB_FOR(MagicComment);

DECLARE_LIST_OF(MagicComment_BLOB, LIST_OF_MagicComment);
DECLARE_BLOB_FOR(LIST_OF_MagicComment);
_Static_assert(sizeof(LIST_OF_MagicComment) == 24, "sizeof(LIST_OF_MagicComment) == 24");

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_MAGIC_COMMENT_HPP
