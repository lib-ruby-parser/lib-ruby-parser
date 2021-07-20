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

Loc_BLOB *lib_ruby_parser__internal__containers__comment__get_location(Comment_BLOB *blob)
{
    Comment *comment = (Comment *)blob;
    Loc *loc = &(comment->location);
    Loc_BLOB *loc_blob = (Loc_BLOB *)loc;
    return loc_blob;
}
CommentType_BLOB *lib_ruby_parser__internal__containers__comment__get_kind(Comment_BLOB *blob)
{
    Comment *comment = (Comment *)blob;
    CommentType *comment_type = &(comment->kind);
    return (CommentType_BLOB *)comment_type;
}
Comment_BLOB lib_ruby_parser__internal__containers__comment__make(Loc_BLOB location, CommentType_BLOB kind)
{
    Comment comment = {.location = UNPACK_Loc(location), .kind = UNPACK_CommentType(kind)};
    return PACK_Comment(comment);
}

IMPL_BLOB(LIST_OF_Comment);
