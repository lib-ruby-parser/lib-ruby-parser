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

impl MagicCommentKind {
    /// Constructs `Encoding` variant
    pub fn encoding() -> Self {
        Self::Encoding
    }
    /// Constructs `FrozenStringLiteral` variant
    pub fn frozen_string_literal() -> Self {
        Self::FrozenStringLiteral
    }
    /// Constructs `WarnIndent` variant
    pub fn warn_indent() -> Self {
        Self::WarnIndent
    }
    /// Constructs `ShareableConstantValue` variant
    pub fn shareable_constant_value() -> Self {
        Self::ShareableConstantValue
    }

    /// Returns `true` if variant is `Encoding`
    pub fn is_encoding(&self) -> bool {
        matches!(self, Self::Encoding)
    }
    /// Returns `true` if variant is `FrozenStringLiteral`
    pub fn is_frozen_string_literal(&self) -> bool {
        matches!(self, Self::FrozenStringLiteral)
    }
    /// Returns `true` if variant is `WarnIndent`
    pub fn is_warn_indent(&self) -> bool {
        matches!(self, Self::WarnIndent)
    }
    /// Returns `true` if variant is `ShareableConstantValue`
    pub fn is_shareable_constant_value(&self) -> bool {
        matches!(self, Self::ShareableConstantValue)
    }
}
