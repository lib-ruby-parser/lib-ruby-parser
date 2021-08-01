use super::CommentType;

#[test]
fn test_debug() {
    assert_eq!(format!("{:?}", CommentType::inline()), "Inline");
    assert_eq!(format!("{:?}", CommentType::document()), "Document");
    assert_eq!(format!("{:?}", CommentType::unknown()), "Unknown");
}

#[test]
fn test_inline() {
    let comment_type = CommentType::inline();
    assert!(comment_type.is_inline());
    assert!(!comment_type.is_document());
    assert!(!comment_type.is_unknown());
}

#[test]
fn test_document() {
    let comment_type = CommentType::document();
    assert!(!comment_type.is_inline());
    assert!(comment_type.is_document());
    assert!(!comment_type.is_unknown());
}

#[test]
fn test_unknown() {
    let comment_type = CommentType::unknown();
    assert!(!comment_type.is_inline());
    assert!(!comment_type.is_document());
    assert!(comment_type.is_unknown());
}
