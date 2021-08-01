use super::MagicCommentKind;

#[test]
fn test_encoding() {
    let v = MagicCommentKind::encoding();

    assert_eq!(v, MagicCommentKind::encoding());
    assert_ne!(v, MagicCommentKind::frozen_string_literal());
    assert_ne!(v, MagicCommentKind::warn_indent());
    assert_ne!(v, MagicCommentKind::shareable_constant_value());

    assert_eq!(format!("{:?}", v), "Encoding");

    assert_eq!(v.clone(), v);
}

#[test]
fn test_frozen_string_literal() {
    let v = MagicCommentKind::frozen_string_literal();

    assert_ne!(v, MagicCommentKind::encoding());
    assert_eq!(v, MagicCommentKind::frozen_string_literal());
    assert_ne!(v, MagicCommentKind::warn_indent());
    assert_ne!(v, MagicCommentKind::shareable_constant_value());

    assert_eq!(format!("{:?}", v), "FrozenStringLiteral");

    assert_eq!(v.clone(), v);
}

#[test]
fn test_warn_indent() {
    let v = MagicCommentKind::warn_indent();

    assert_ne!(v, MagicCommentKind::encoding());
    assert_ne!(v, MagicCommentKind::frozen_string_literal());
    assert_eq!(v, MagicCommentKind::warn_indent());
    assert_ne!(v, MagicCommentKind::shareable_constant_value());

    assert_eq!(format!("{:?}", v), "WarnIndent");

    assert_eq!(v.clone(), v);
}

#[test]
fn test_shareable_constant_value() {
    let v = MagicCommentKind::shareable_constant_value();

    assert_ne!(v, MagicCommentKind::encoding());
    assert_ne!(v, MagicCommentKind::frozen_string_literal());
    assert_ne!(v, MagicCommentKind::warn_indent());
    assert_eq!(v, MagicCommentKind::shareable_constant_value());

    assert_eq!(format!("{:?}", v), "ShareableConstantValue");

    assert_eq!(v.clone(), v);
}
