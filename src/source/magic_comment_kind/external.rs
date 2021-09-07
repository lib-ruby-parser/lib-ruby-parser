use crate::blobs::Blob;

/// An enum of all magic comment kinds
#[repr(C)]
pub struct MagicCommentKind {
    pub(crate) blob: Blob<MagicCommentKind>,
}

extern "C" {
    fn lib_ruby_parser__external__magic_comment_kind__new_encoding() -> Blob<MagicCommentKind>;
    fn lib_ruby_parser__external__magic_comment_kind__new_frozen_string_literal(
    ) -> Blob<MagicCommentKind>;
    fn lib_ruby_parser__external__magic_comment_kind__new_warn_indent() -> Blob<MagicCommentKind>;
    fn lib_ruby_parser__external__magic_comment_kind__new_shareable_constant_value(
    ) -> Blob<MagicCommentKind>;
    fn lib_ruby_parser__external__magic_comment_kind__drop(blob: *mut Blob<MagicCommentKind>);
    fn lib_ruby_parser__external__magic_comment_kind__is_encoding(
        blob: *const Blob<MagicCommentKind>,
    ) -> bool;
    fn lib_ruby_parser__external__magic_comment_kind__is_frozen_string_literal(
        blob: *const Blob<MagicCommentKind>,
    ) -> bool;
    fn lib_ruby_parser__external__magic_comment_kind__is_warn_indent(
        blob: *const Blob<MagicCommentKind>,
    ) -> bool;
    fn lib_ruby_parser__external__magic_comment_kind__is_shareable_constant_value(
        blob: *const Blob<MagicCommentKind>,
    ) -> bool;
}

impl Drop for MagicCommentKind {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__external__magic_comment_kind__drop(&mut self.blob) }
    }
}

impl MagicCommentKind {
    /// Constructs `Encoding` variant
    pub fn encoding() -> Self {
        let blob = unsafe { lib_ruby_parser__external__magic_comment_kind__new_encoding() };
        Self { blob }
    }
    /// Constructs `FrozenStringLiteral` variant
    pub fn frozen_string_literal() -> Self {
        let blob =
            unsafe { lib_ruby_parser__external__magic_comment_kind__new_frozen_string_literal() };
        Self { blob }
    }
    /// Constructs `WarnIndent` variant
    pub fn warn_indent() -> Self {
        let blob = unsafe { lib_ruby_parser__external__magic_comment_kind__new_warn_indent() };
        Self { blob }
    }
    /// Constructs `ShareableConstantValue` variant
    pub fn shareable_constant_value() -> Self {
        let blob = unsafe {
            lib_ruby_parser__external__magic_comment_kind__new_shareable_constant_value()
        };
        Self { blob }
    }

    /// Returns `true` if variant is `Encoding`
    pub fn is_encoding(&self) -> bool {
        unsafe { lib_ruby_parser__external__magic_comment_kind__is_encoding(&self.blob) }
    }
    /// Returns `true` if variant is `FrozenStringLiteral`
    pub fn is_frozen_string_literal(&self) -> bool {
        unsafe {
            lib_ruby_parser__external__magic_comment_kind__is_frozen_string_literal(&self.blob)
        }
    }
    /// Returns `true` if variant is `WarnIndent`
    pub fn is_warn_indent(&self) -> bool {
        unsafe { lib_ruby_parser__external__magic_comment_kind__is_warn_indent(&self.blob) }
    }
    /// Returns `true` if variant is `ShareableConstantValue`
    pub fn is_shareable_constant_value(&self) -> bool {
        unsafe {
            lib_ruby_parser__external__magic_comment_kind__is_shareable_constant_value(&self.blob)
        }
    }
}

impl std::fmt::Debug for MagicCommentKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_encoding() {
            write!(f, "Encoding")
        } else if self.is_frozen_string_literal() {
            write!(f, "FrozenStringLiteral")
        } else if self.is_warn_indent() {
            write!(f, "WarnIndent")
        } else if self.is_shareable_constant_value() {
            write!(f, "ShareableConstantValue")
        } else {
            unreachable!("Only encoding/frozen_string_literal/warn_indent/shareable_constant_value variants are supported")
        }
    }
}

impl Clone for MagicCommentKind {
    fn clone(&self) -> Self {
        if self.is_encoding() {
            Self::encoding()
        } else if self.is_frozen_string_literal() {
            Self::frozen_string_literal()
        } else if self.is_warn_indent() {
            Self::warn_indent()
        } else if self.is_shareable_constant_value() {
            Self::shareable_constant_value()
        } else {
            unreachable!("Only encoding/frozen_string_literal/warn_indent/shareable_constant_value variants are supported")
        }
    }
}

impl PartialEq for MagicCommentKind {
    fn eq(&self, other: &Self) -> bool {
        if self.is_encoding() {
            other.is_encoding()
        } else if self.is_frozen_string_literal() {
            other.is_frozen_string_literal()
        } else if self.is_warn_indent() {
            other.is_warn_indent()
        } else if self.is_shareable_constant_value() {
            other.is_shareable_constant_value()
        } else {
            unreachable!("Only encoding/frozen_string_literal/warn_indent/shareable_constant_value variants are supported")
        }
    }
}
