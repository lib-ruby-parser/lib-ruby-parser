#ifndef LIB_RUBY_PARSER_EXTERNAL_C_COMMENT_H
#define LIB_RUBY_PARSER_EXTERNAL_C_COMMENT_H

#include "declare_dummy_struct.h"
#include "declare_blob.h"
#include "declare_list.h"
#include <stdbool.h>
#include "loc.h"

typedef enum
{
    DOCUMENT,
    INLINE,
    UNKNOWN,
} CommentType;
DECLARE_BLOB_FOR(CommentType);
_Static_assert(sizeof(CommentType) == 4, "sizeof(CommentType) == 4");

CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__make_inline();
CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__make_document();
CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__make_unknown();
bool lib_ruby_parser__internal__containers__comment_type__is_inline(CommentType_BLOB blob);
bool lib_ruby_parser__internal__containers__comment_type__is_document(CommentType_BLOB blob);
bool lib_ruby_parser__internal__containers__comment_type__is_unknown(CommentType_BLOB blob);

typedef struct
{
    Loc location;
    CommentType kind;
} Comment;
DECLARE_BLOB_FOR(Comment);

Loc_BLOB *lib_ruby_parser__internal__containers__comment__get_location(Comment_BLOB *blob);
CommentType_BLOB *lib_ruby_parser__internal__containers__comment__get_kind(Comment_BLOB *blob);
Comment_BLOB lib_ruby_parser__internal__containers__comment__make(Loc_BLOB location, CommentType_BLOB kind);

DECLARE_LIST_OF(Comment_BLOB, LIST_OF_Comment);
DECLARE_BLOB_FOR(LIST_OF_Comment);
_Static_assert(sizeof(LIST_OF_Comment) == 24, "sizeof(LIST_OF_Comment) == 24");

#endif // LIB_RUBY_PARSER_EXTERNAL_C_COMMENT_H
