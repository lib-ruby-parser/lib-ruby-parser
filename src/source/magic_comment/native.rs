use crate::source::MagicCommentKind;
use crate::Loc;

/// Representation of a magic comment in Ruby
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct MagicComment {
    /// Kind of a magic comment
    pub kind: MagicCommentKind,

    /// Location of the "key":
    ///
    /// ```text
    /// # encoding: utf-8
    ///   ~~~~~~~~
    /// ```
    pub key_l: Loc,

    /// Location of the "value":
    ///
    /// ```text
    /// # encoding: utf-8
    ///             ~~~~~
    /// ```
    pub value_l: Loc,
}

impl MagicComment {
    /// Constructor
    pub fn new(kind: MagicCommentKind, key_l: Loc, value_l: Loc) -> Self {
        Self {
            kind,
            key_l,
            value_l,
        }
    }

    /// Returns kind of the of the MagicComment
    pub fn kind(&self) -> &MagicCommentKind {
        &self.kind
    }
    /// Returns location of MagicComment's key
    pub fn key_l(&self) -> &Loc {
        &self.key_l
    }
    /// Returns location of MagicComment's value
    pub fn value_l(&self) -> &Loc {
        &self.value_l
    }
}
