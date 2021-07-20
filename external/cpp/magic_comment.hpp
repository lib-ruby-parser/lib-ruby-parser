#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_MAGIC_COMMENT_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_MAGIC_COMMENT_HPP

#include "declare_dummy_struct.hpp"
#include "declare_blob.hpp"
#include "declare_list.hpp"
#include "loc.hpp"

enum class MagicCommentKind
{
    ENCODING,
    FROZEN_STRING_LITERAL,
    WARN_INDENT,
    SHAREABLE_CONSTANT_VALUE,
};
DECLARE_BLOB_FOR(MagicCommentKind);
_Static_assert(sizeof(MagicCommentKind) == 4, "sizeof(MagicCommentKind) == 4");

extern "C"
{
    MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__make_encoding();
    MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__make_frozen_string_literal();
    MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__make_warn_indent();
    MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__make_shareable_constant_value();

    bool lib_ruby_parser__internal__containers__magic_comment_kind__is_encoding(MagicCommentKind_BLOB blob);
    bool lib_ruby_parser__internal__containers__magic_comment_kind__is_frozen_string_literal(MagicCommentKind_BLOB blob);
    bool lib_ruby_parser__internal__containers__magic_comment_kind__is_warn_indent(MagicCommentKind_BLOB blob);
    bool lib_ruby_parser__internal__containers__magic_comment_kind__is_shareable_constant_value(MagicCommentKind_BLOB blob);
}

class MagicComment
{
public:
    MagicComment(MagicCommentKind kind, Loc key_l, Loc value_l);
    MagicCommentKind kind;
    Loc key_l;
    Loc value_l;
};
DECLARE_BLOB_FOR(MagicComment);

DECLARE_LIST_OF(MagicComment_BLOB, LIST_OF_MagicComment);
DECLARE_BLOB_FOR(LIST_OF_MagicComment);
_Static_assert(sizeof(LIST_OF_MagicComment) == 24, "sizeof(LIST_OF_MagicComment) == 24");

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_MAGIC_COMMENT_HPP
