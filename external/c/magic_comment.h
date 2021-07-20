#ifndef LIB_RUBY_PARSER_EXTERNAL_C_MAGIC_COMMENT_H
#define LIB_RUBY_PARSER_EXTERNAL_C_MAGIC_COMMENT_H

#include "declare_dummy_struct.h"
#include "declare_blob.h"
#include "declare_list.h"
#include "loc.h"
#include <stdbool.h>

typedef enum
{
    ENCODING,
    FROZEN_STRING_LITERAL,
    WARN_INDENT,
    SHAREABLE_CONSTANT_VALUE,
} MagicCommentKind;
DECLARE_BLOB_FOR(MagicCommentKind);
_Static_assert(sizeof(MagicCommentKind) == 4, "sizeof(MagicCommentKind) == 4");

MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__make_encoding();
MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__make_frozen_string_literal();
MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__make_warn_indent();
MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__make_shareable_constant_value();

bool lib_ruby_parser__internal__containers__magic_comment_kind__is_encoding(MagicCommentKind_BLOB blob);
bool lib_ruby_parser__internal__containers__magic_comment_kind__is_frozen_string_literal(MagicCommentKind_BLOB blob);
bool lib_ruby_parser__internal__containers__magic_comment_kind__is_warn_indent(MagicCommentKind_BLOB blob);
bool lib_ruby_parser__internal__containers__magic_comment_kind__is_shareable_constant_value(MagicCommentKind_BLOB blob);

typedef struct
{
    MagicCommentKind kind;
    Loc key_l;
    Loc value_l;
} MagicComment;
DECLARE_BLOB_FOR(MagicComment);
_Static_assert(sizeof(MagicComment) == 40, "sizeof(MagicComment) == 40");

MagicComment_BLOB lib_ruby_parser__internal__containers__magic_comment__make(MagicCommentKind_BLOB kind, Loc_BLOB key_l, Loc_BLOB value_l);
const MagicCommentKind_BLOB *lib_ruby_parser__internal__containers__magic_comment__get_kind(const MagicComment_BLOB *blob);
const Loc_BLOB *lib_ruby_parser__internal__containers__magic_comment__get_key_l(const MagicComment_BLOB *blob);
const Loc_BLOB *lib_ruby_parser__internal__containers__magic_comment__get_value_l(const MagicComment_BLOB *blob);

DECLARE_LIST_OF(MagicComment_BLOB, LIST_OF_MagicComment);
DECLARE_BLOB_FOR(LIST_OF_MagicComment);
_Static_assert(sizeof(LIST_OF_MagicComment) == 24, "sizeof(LIST_OF_MagicComment) == 24");

#endif // LIB_RUBY_PARSER_EXTERNAL_C_MAGIC_COMMENT_H
