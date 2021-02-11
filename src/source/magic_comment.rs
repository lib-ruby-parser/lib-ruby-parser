use crate::Loc;

#[derive(Debug, Clone, PartialEq)]
pub enum MagicCommentKind {
    Encoding,
    FrozenStringLiteral,
    WarnIndent,
    ShareableContstantValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MagicComment {
    pub kind: MagicCommentKind,
    pub key_l: Loc,
    pub value_l: Loc,
}

impl MagicComment {
    pub fn new(kind: MagicCommentKind, key_l: Loc, value_l: Loc) -> Self {
        Self {
            kind,
            key_l,
            value_l,
        }
    }
}
