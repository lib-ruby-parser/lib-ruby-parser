use super::Comment;
use crate::source::CommentType;
use crate::Loc;

#[test]
fn test_comment_type() {
    let comment = Comment::make(Loc { begin: 1, end: 2 }, CommentType::inline());

    assert_eq!(comment.location().begin, 1);
    assert_eq!(comment.location().end, 2);
    assert!(comment.kind().is_inline());
}

fn comment() -> Comment {
    Comment::make(Loc { begin: 1, end: 2 }, CommentType::document())
}

#[test]
fn test_debug() {
    assert_eq!(
        format!("{:?}", comment()),
        "Comment { location: 1...2, kind: Document }"
    )
}

#[test]
fn test_compare() {
    assert_eq!(
        Comment::make(Loc { begin: 1, end: 2 }, CommentType::document()),
        comment()
    );

    assert_ne!(
        Comment::make(Loc { begin: 2, end: 2 }, CommentType::document()),
        comment()
    );

    assert_ne!(
        Comment::make(Loc { begin: 1, end: 3 }, CommentType::document()),
        comment()
    );

    assert_ne!(
        Comment::make(Loc { begin: 1, end: 2 }, CommentType::inline()),
        comment()
    );
}

#[test]
fn test_clone() {
    let comment = comment().clone();
    assert_eq!(comment.location(), &Loc { begin: 1, end: 2 });
    assert_eq!(comment.kind(), &CommentType::document());
}
