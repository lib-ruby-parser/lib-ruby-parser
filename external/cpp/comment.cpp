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

Comment::Comment(Loc location, CommentType kind) : location(location), kind(kind) {}

extern "C"
{
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
        Comment comment(UNPACK(location), UNPACK(kind));
        return PACK(comment);
    }
}

IMPL_BLOB(LIST_OF_Comment);
