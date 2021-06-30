#ifndef LIB_RUBY_PARSER_EXTERNAL_C_COMMENT_H
#define LIB_RUBY_PARSER_EXTERNAL_C_COMMENT_H

#include "declare_dummy_struct.h"
#include "declare_blob.h"
#include "declare_list.h"

DECLARE_DUMMY_STRUCT(Comment, 20);
DECLARE_BLOB_FOR(Comment);

DECLARE_LIST_OF(Comment_BLOB, LIST_OF_Comment);
DECLARE_BLOB_FOR(LIST_OF_Comment);
_Static_assert(sizeof(LIST_OF_Comment) == 24, "sizeof(LIST_OF_Comment) == 24");

#endif // LIB_RUBY_PARSER_EXTERNAL_C_COMMENT_H
