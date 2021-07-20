#[cfg(not(feature = "compile-with-external-structures"))]
mod magic_comment_kind {
    /// An enum of all magic comment kinds
    #[repr(C)]
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
}

#[cfg(feature = "compile-with-external-structures")]
mod magic_comment_kind {
    use crate::containers::size::MAGIC_COMMENT_KIND_SIZE;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub(crate) struct MagicCommentKindBlob {
        blob: [u8; MAGIC_COMMENT_KIND_SIZE],
    }

    /// An enum of all magic comment kinds
    #[repr(C)]
    pub struct MagicCommentKind {
        pub(crate) blob: MagicCommentKindBlob,
    }

    extern "C" {
        fn lib_ruby_parser__internal__containers__magic_comment_kind__make_encoding(
        ) -> MagicCommentKindBlob;
        fn lib_ruby_parser__internal__containers__magic_comment_kind__make_frozen_string_literal(
        ) -> MagicCommentKindBlob;
        fn lib_ruby_parser__internal__containers__magic_comment_kind__make_warn_indent(
        ) -> MagicCommentKindBlob;
        fn lib_ruby_parser__internal__containers__magic_comment_kind__make_shareable_constant_value(
        ) -> MagicCommentKindBlob;

        fn lib_ruby_parser__internal__containers__magic_comment_kind__is_encoding(
            blob: MagicCommentKindBlob,
        ) -> bool;
        fn lib_ruby_parser__internal__containers__magic_comment_kind__is_frozen_string_literal(
            blob: MagicCommentKindBlob,
        ) -> bool;
        fn lib_ruby_parser__internal__containers__magic_comment_kind__is_warn_indent(
            blob: MagicCommentKindBlob,
        ) -> bool;
        fn lib_ruby_parser__internal__containers__magic_comment_kind__is_shareable_constant_value(
            blob: MagicCommentKindBlob,
        ) -> bool;
    }

    impl MagicCommentKind {
        /// Constructs `Encoding` variant
        pub fn encoding() -> Self {
            let blob = unsafe {
                lib_ruby_parser__internal__containers__magic_comment_kind__make_encoding()
            };
            Self { blob }
        }
        /// Constructs `FrozenStringLiteral` variant
        pub fn frozen_string_literal() -> Self {
            let blob = unsafe {
                lib_ruby_parser__internal__containers__magic_comment_kind__make_frozen_string_literal ()
            };
            Self { blob }
        }
        /// Constructs `WarnIndent` variant
        pub fn warn_indent() -> Self {
            let blob = unsafe {
                lib_ruby_parser__internal__containers__magic_comment_kind__make_warn_indent()
            };
            Self { blob }
        }
        /// Constructs `ShareableConstantValue` variant
        pub fn shareable_constant_value() -> Self {
            let blob = unsafe {
                lib_ruby_parser__internal__containers__magic_comment_kind__make_shareable_constant_value()
            };
            Self { blob }
        }

        /// Returns `true` if variant is `Encoding`
        pub fn is_encoding(&self) -> bool {
            unsafe {
                lib_ruby_parser__internal__containers__magic_comment_kind__is_encoding(self.blob)
            }
        }
        /// Returns `true` if variant is `FrozenStringLiteral`
        pub fn is_frozen_string_literal(&self) -> bool {
            unsafe {
                lib_ruby_parser__internal__containers__magic_comment_kind__is_frozen_string_literal(
                    self.blob,
                )
            }
        }
        /// Returns `true` if variant is `WarnIndent`
        pub fn is_warn_indent(&self) -> bool {
            unsafe {
                lib_ruby_parser__internal__containers__magic_comment_kind__is_warn_indent(self.blob)
            }
        }
        /// Returns `true` if variant is `ShareableConstantValue`
        pub fn is_shareable_constant_value(&self) -> bool {
            unsafe {
                lib_ruby_parser__internal__containers__magic_comment_kind__is_shareable_constant_value(self.blob)
            }
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

pub use magic_comment_kind::MagicCommentKind;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use magic_comment_kind::MagicCommentKindBlob;

#[cfg(test)]
mod tests {
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
}
