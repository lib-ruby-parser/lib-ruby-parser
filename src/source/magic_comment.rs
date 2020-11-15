use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub enum MagicCommentKind {
    Encoding,
    FrozenStringLiteral,
    WarnIndent,
}

#[derive(Debug, Clone)]
pub struct MagicComment {
    pub kind: MagicCommentKind,
    pub key_l: Range,
    pub value_l: Range,
}

impl MagicComment {
    pub fn new(kind: MagicCommentKind, key_l: Range, value_l: Range) -> Self {
        Self {
            kind,
            key_l,
            value_l,
        }
    }
}
