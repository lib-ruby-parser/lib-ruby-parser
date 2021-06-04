#ifndef LIB_RUBY_PARSER_EXTERNAL_C_MAGIC_COMMENT_H
#define LIB_RUBY_PARSER_EXTERNAL_C_MAGIC_COMMENT_H

#include "declare_dummy_struct.h"
#include "declare_blob.h"
#include "declare_list.h"

DECLARE_DUMMY_STRUCT(MagicComment, 40);
DECLARE_BLOB_FOR(MagicComment);

DECLARE_LIST_OF(MagicComment_BLOB_DATA, LIST_OF_MagicComment);
DECLARE_BLOB_FOR(LIST_OF_MagicComment);
_Static_assert(sizeof(LIST_OF_MagicComment) == 24, "sizeof(LIST_OF_MagicComment) == 24");

#endif // LIB_RUBY_PARSER_EXTERNAL_C_MAGIC_COMMENT_H
