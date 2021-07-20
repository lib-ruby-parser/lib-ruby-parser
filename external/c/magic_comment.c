#include "magic_comment.h"
#include "impl_blob.h"

IMPL_BLOB(MagicCommentKind);

MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__make_encoding()
{
    return PACK_MagicCommentKind(ENCODING);
}
MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__make_frozen_string_literal()
{
    return PACK_MagicCommentKind(FROZEN_STRING_LITERAL);
}
MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__make_warn_indent()
{
    return PACK_MagicCommentKind(WARN_INDENT);
}
MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__make_shareable_constant_value()
{
    return PACK_MagicCommentKind(SHAREABLE_CONSTANT_VALUE);
}

bool lib_ruby_parser__internal__containers__magic_comment_kind__is_encoding(MagicCommentKind_BLOB blob)
{
    return UNPACK_MagicCommentKind(blob) == ENCODING;
}
bool lib_ruby_parser__internal__containers__magic_comment_kind__is_frozen_string_literal(MagicCommentKind_BLOB blob)
{
    return UNPACK_MagicCommentKind(blob) == FROZEN_STRING_LITERAL;
}
bool lib_ruby_parser__internal__containers__magic_comment_kind__is_warn_indent(MagicCommentKind_BLOB blob)
{
    return UNPACK_MagicCommentKind(blob) == WARN_INDENT;
}
bool lib_ruby_parser__internal__containers__magic_comment_kind__is_shareable_constant_value(MagicCommentKind_BLOB blob)
{
    return UNPACK_MagicCommentKind(blob) == SHAREABLE_CONSTANT_VALUE;
}

IMPL_BLOB(MagicComment);
IMPL_BLOB(LIST_OF_MagicComment);
