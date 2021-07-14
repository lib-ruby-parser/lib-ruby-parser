#include "comment.hpp"
#include "impl_blob.hpp"

IMPL_BLOB(CommentType);

extern "C"
{
    CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__make_inline()
    {
        return PACK(CommentType::INLINE);
    }
    CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__make_document()
    {
        return PACK(CommentType::DOCUMENT);
    }
    CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__make_unknown()
    {
        return PACK(CommentType::UNKNOWN);
    }
    bool lib_ruby_parser__internal__containers__comment_type__is_inline(CommentType_BLOB blob)
    {
        return UNPACK(blob) == CommentType::INLINE;
    }
    bool lib_ruby_parser__internal__containers__comment_type__is_document(CommentType_BLOB blob)
    {
        return UNPACK(blob) == CommentType::DOCUMENT;
    }
    bool lib_ruby_parser__internal__containers__comment_type__is_unknown(CommentType_BLOB blob)
    {
        return UNPACK(blob) == CommentType::UNKNOWN;
    }
}

IMPL_BLOB(Comment);
IMPL_BLOB(LIST_OF_Comment);
