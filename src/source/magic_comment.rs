use crate::Loc;

/// An enum of all magic comment kinds
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub enum MagicCommentKind {
    /// `# encoding: ... comment`
    Encoding,

    /// `# frozen_string_literal: true/false` comment
    FrozenStringLiteral,

    /// `# warn_indent: true/false` comment
    WarnIndent,

    /// `# shareable_constant_value: ...` comment
    ShareableConstantValue,
}

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
