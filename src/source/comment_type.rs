#[cfg(not(feature = "compile-with-external-structures"))]
mod comment_type {
    /// Enum of all possible comment types
    #[repr(C)]
    pub enum CommentType {
        /// Inline comment like
        ///
        /// ```text
        /// # comment
        /// ```
        Inline,

        /// Document comment like
        ///
        /// ```text
        /// =begin
        /// comment
        /// =end
        /// ```
        Document,

        /// Uknknown comment type,
        /// most probably means that either `Loc` or given `Input` is invalid
        Unknown,
    }

    impl CommentType {
        /// Constructs `Inline` variant
        pub fn inline() -> Self {
            Self::Inline
        }

        /// Constructs `Document` variant
        pub fn document() -> Self {
            Self::Document
        }

        /// Constructs `Unknown` variant
        pub fn unknown() -> Self {
            Self::Unknown
        }

        /// Returns `true` if current variant is `Inline`
        pub fn is_inline(&self) -> bool {
            matches!(self, Self::Inline)
        }

        /// Returns `true` if current variant is `Document`
        pub fn is_document(&self) -> bool {
            matches!(self, Self::Document)
        }

        /// Returns `true` if current variant is `Unknown`
        pub fn is_unknown(&self) -> bool {
            matches!(self, Self::Unknown)
        }
    }
}

#[cfg(feature = "compile-with-external-structures")]
mod comment_type {
    use crate::containers::size::COMMENT_TYPE_SIZE;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CommentTypeBlob {
        blob: [u8; COMMENT_TYPE_SIZE],
    }

    /// Enum of all possible comment types
    #[repr(C)]
    pub struct CommentType {
        blob: CommentTypeBlob,
    }

    extern "C" {
        fn lib_ruby_parser__internal__containers__comment_type__make_inline() -> CommentTypeBlob;
        fn lib_ruby_parser__internal__containers__comment_type__make_document() -> CommentTypeBlob;
        fn lib_ruby_parser__internal__containers__comment_type__make_unknown() -> CommentTypeBlob;

        fn lib_ruby_parser__internal__containers__comment_type__is_inline(
            blob: CommentTypeBlob,
        ) -> bool;
        fn lib_ruby_parser__internal__containers__comment_type__is_document(
            blob: CommentTypeBlob,
        ) -> bool;
        fn lib_ruby_parser__internal__containers__comment_type__is_unknown(
            blob: CommentTypeBlob,
        ) -> bool;
    }

    impl CommentType {
        /// Constructs `Inline` variant
        pub fn inline() -> Self {
            let blob =
                unsafe { lib_ruby_parser__internal__containers__comment_type__make_inline() };
            Self { blob }
        }

        /// Constructs `Document` variant
        pub fn document() -> Self {
            let blob =
                unsafe { lib_ruby_parser__internal__containers__comment_type__make_document() };
            Self { blob }
        }

        /// Constructs `Unknown` variant
        pub fn unknown() -> Self {
            let blob =
                unsafe { lib_ruby_parser__internal__containers__comment_type__make_unknown() };
            Self { blob }
        }

        /// Returns `true` if current variant is `Inline`
        pub fn is_inline(&self) -> bool {
            unsafe { lib_ruby_parser__internal__containers__comment_type__is_inline(self.blob) }
        }

        /// Returns `true` if current variant is `Document`
        pub fn is_document(&self) -> bool {
            unsafe { lib_ruby_parser__internal__containers__comment_type__is_document(self.blob) }
        }

        /// Returns `true` if current variant is `Unknown`
        pub fn is_unknown(&self) -> bool {
            unsafe { lib_ruby_parser__internal__containers__comment_type__is_unknown(self.blob) }
        }
    }
}

impl std::fmt::Debug for CommentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_inline() {
            write!(f, "Inline")
        } else if self.is_document() {
            write!(f, "Document")
        } else if self.is_unknown() {
            write!(f, "Unknown")
        } else {
            unreachable!("Only document/inline/unknown options are supported")
        }
    }
}

impl Clone for CommentType {
    fn clone(&self) -> Self {
        if self.is_inline() {
            Self::inline()
        } else if self.is_document() {
            Self::document()
        } else if self.is_unknown() {
            Self::unknown()
        } else {
            unreachable!("Only document/inline/unknown options are supported")
        }
    }
}

impl PartialEq for CommentType {
    fn eq(&self, other: &Self) -> bool {
        if self.is_inline() {
            other.is_inline()
        } else if self.is_document() {
            other.is_document()
        } else if self.is_unknown() {
            other.is_unknown()
        } else {
            unreachable!("Only document/inline/unknown are supported")
        }
    }
}

impl Eq for CommentType {}

pub use comment_type::CommentType;

#[cfg(test)]
mod tests {
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
}
