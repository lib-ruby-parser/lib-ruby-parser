#include "comment.h"
#include "impl_blob.h"

IMPL_BLOB(CommentType);

CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__make_inline()
{
    return PACK_CommentType(INLINE);
}
CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__make_document()
{
    return PACK_CommentType(DOCUMENT);
}
CommentType_BLOB lib_ruby_parser__internal__containers__comment_type__make_unknown()
{
    return PACK_CommentType(UNKNOWN);
}
bool lib_ruby_parser__internal__containers__comment_type__is_inline(CommentType_BLOB blob)
{
    return UNPACK_CommentType(blob) == INLINE;
}
bool lib_ruby_parser__internal__containers__comment_type__is_document(CommentType_BLOB blob)
{
    return UNPACK_CommentType(blob) == DOCUMENT;
}
bool lib_ruby_parser__internal__containers__comment_type__is_unknown(CommentType_BLOB blob)
{
    return UNPACK_CommentType(blob) == UNKNOWN;
}

IMPL_BLOB(Comment);
IMPL_BLOB(LIST_OF_Comment);
