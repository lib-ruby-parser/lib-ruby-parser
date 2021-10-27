use super::MagicComment;
use crate::source::MagicCommentKind;
use crate::Loc;

fn new_magic_comment() -> MagicComment {
    MagicComment::new(
        MagicCommentKind::frozen_string_literal(),
        Loc { begin: 1, end: 2 },
        Loc { begin: 3, end: 4 },
    )
}

#[test]
fn test_new() {
    let magic_comment = new_magic_comment();

    assert!(magic_comment.kind().is_frozen_string_literal());
    assert_eq!(magic_comment.key_l().begin, 1);
    assert_eq!(magic_comment.key_l().end, 2);
    assert_eq!(magic_comment.value_l().begin, 3);
    assert_eq!(magic_comment.value_l().end, 4);
}

#[test]
fn test_debug() {
    let magic_comment = new_magic_comment();

    assert_eq!(
        format!("{:?}", magic_comment),
        "MagicComment { kind: FrozenStringLiteral, key_l: 1...2, value_l: 3...4 }"
    )
}

#[test]
fn test_cmp() {
    let magic_comment = new_magic_comment();

    assert_eq!(
        magic_comment,
        MagicComment::new(
            MagicCommentKind::frozen_string_literal(),
            Loc { begin: 1, end: 2 },
            Loc { begin: 3, end: 4 },
        )
    );

    assert_ne!(
        magic_comment,
        MagicComment::new(
            MagicCommentKind::encoding(),
            Loc { begin: 1, end: 2 },
            Loc { begin: 3, end: 4 },
        )
    );

    assert_ne!(
        magic_comment,
        MagicComment::new(
            MagicCommentKind::frozen_string_literal(),
            Loc { begin: 0, end: 2 },
            Loc { begin: 3, end: 4 },
        )
    );

    assert_ne!(
        magic_comment,
        MagicComment::new(
            MagicCommentKind::frozen_string_literal(),
            Loc { begin: 1, end: 0 },
            Loc { begin: 3, end: 4 },
        )
    );

    assert_ne!(
        magic_comment,
        MagicComment::new(
            MagicCommentKind::frozen_string_literal(),
            Loc { begin: 1, end: 2 },
            Loc { begin: 0, end: 4 },
        )
    );

    assert_ne!(
        magic_comment,
        MagicComment::new(
            MagicCommentKind::frozen_string_literal(),
            Loc { begin: 1, end: 2 },
            Loc { begin: 3, end: 0 },
        )
    );
}

#[test]
fn test_clone() {
    let magic_comment = new_magic_comment();

    assert_eq!(magic_comment, magic_comment.clone())
}
