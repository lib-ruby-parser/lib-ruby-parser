#include "magic_comment.hpp"
#include "impl_blob.hpp"

IMPL_BLOB(MagicCommentKind);

extern "C"
{
    MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__make_encoding()
    {
        return PACK(MagicCommentKind::ENCODING);
    }
    MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__make_frozen_string_literal()
    {
        return PACK(MagicCommentKind::FROZEN_STRING_LITERAL);
    }
    MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__make_warn_indent()
    {
        return PACK(MagicCommentKind::WARN_INDENT);
    }
    MagicCommentKind_BLOB lib_ruby_parser__internal__containers__magic_comment_kind__make_shareable_constant_value()
    {
        return PACK(MagicCommentKind::SHAREABLE_CONSTANT_VALUE);
    }

    bool lib_ruby_parser__internal__containers__magic_comment_kind__is_encoding(MagicCommentKind_BLOB blob)
    {
        return UNPACK(blob) == MagicCommentKind::ENCODING;
    }
    bool lib_ruby_parser__internal__containers__magic_comment_kind__is_frozen_string_literal(MagicCommentKind_BLOB blob)
    {
        return UNPACK(blob) == MagicCommentKind::FROZEN_STRING_LITERAL;
    }
    bool lib_ruby_parser__internal__containers__magic_comment_kind__is_warn_indent(MagicCommentKind_BLOB blob)
    {
        return UNPACK(blob) == MagicCommentKind::WARN_INDENT;
    }
    bool lib_ruby_parser__internal__containers__magic_comment_kind__is_shareable_constant_value(MagicCommentKind_BLOB blob)
    {
        return UNPACK(blob) == MagicCommentKind::SHAREABLE_CONSTANT_VALUE;
    }
}

IMPL_BLOB(MagicComment);
IMPL_BLOB(LIST_OF_MagicComment);
