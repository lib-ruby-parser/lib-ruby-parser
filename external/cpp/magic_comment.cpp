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

MagicComment::MagicComment(MagicCommentKind kind, Loc key_l, Loc value_l) : kind(kind), key_l(key_l), value_l(value_l) {}

extern "C"
{
    MagicComment_BLOB lib_ruby_parser__internal__containers__magic_comment__make(MagicCommentKind_BLOB kind, Loc_BLOB key_l, Loc_BLOB value_l)
    {
        MagicComment magic_comment(UNPACK(kind), UNPACK(key_l), UNPACK(value_l));
        return PACK(magic_comment);
    }
    const MagicCommentKind_BLOB *lib_ruby_parser__internal__containers__magic_comment__get_kind(const MagicComment_BLOB *blob)
    {
        MagicComment *magic_comment = (MagicComment *)blob;
        MagicCommentKind *kind = &(magic_comment->kind);
        MagicCommentKind_BLOB *kind_blob = (MagicCommentKind_BLOB *)kind;
        return kind_blob;
    }
    const Loc_BLOB *lib_ruby_parser__internal__containers__magic_comment__get_key_l(const MagicComment_BLOB *blob)
    {
        MagicComment *magic_comment = (MagicComment *)blob;
        Loc *key_l = &(magic_comment->key_l);
        Loc_BLOB *key_l_blob = (Loc_BLOB *)key_l;
        return key_l_blob;
    }
    const Loc_BLOB *lib_ruby_parser__internal__containers__magic_comment__get_value_l(const MagicComment_BLOB *blob)
    {
        MagicComment *magic_comment = (MagicComment *)blob;
        Loc *value_l = &(magic_comment->value_l);
        Loc_BLOB *value_l_blob = (Loc_BLOB *)value_l;
        return value_l_blob;
    }
}

IMPL_BLOB(LIST_OF_MagicComment);
