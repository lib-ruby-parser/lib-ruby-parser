use crate::blobs::CommentTypeBlob;

/// Enum of all possible comment types
#[repr(C)]
pub struct CommentType {
    pub(crate) blob: CommentTypeBlob,
}

extern "C" {
    fn lib_ruby_parser__external__comment_type__new_inline() -> CommentTypeBlob;
    fn lib_ruby_parser__external__comment_type__new_document() -> CommentTypeBlob;
    fn lib_ruby_parser__external__comment_type__new_unknown() -> CommentTypeBlob;
    fn lib_ruby_parser__external__comment_type__drop(blob: &mut CommentTypeBlob);
    fn lib_ruby_parser__external__comment_type__is_inline(blob: *const CommentTypeBlob) -> bool;
    fn lib_ruby_parser__external__comment_type__is_document(blob: *const CommentTypeBlob) -> bool;
    fn lib_ruby_parser__external__comment_type__is_unknown(blob: *const CommentTypeBlob) -> bool;
}

impl Drop for CommentType {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__external__comment_type__drop(&mut self.blob) }
    }
}

impl CommentType {
    /// Constructs `Inline` variant
    pub fn inline() -> Self {
        let blob = unsafe { lib_ruby_parser__external__comment_type__new_inline() };
        Self { blob }
    }

    /// Constructs `Document` variant
    pub fn document() -> Self {
        let blob = unsafe { lib_ruby_parser__external__comment_type__new_document() };
        Self { blob }
    }

    /// Constructs `Unknown` variant
    pub fn unknown() -> Self {
        let blob = unsafe { lib_ruby_parser__external__comment_type__new_unknown() };
        Self { blob }
    }

    /// Returns `true` if current variant is `Inline`
    pub fn is_inline(&self) -> bool {
        unsafe { lib_ruby_parser__external__comment_type__is_inline(&self.blob) }
    }

    /// Returns `true` if current variant is `Document`
    pub fn is_document(&self) -> bool {
        unsafe { lib_ruby_parser__external__comment_type__is_document(&self.blob) }
    }

    /// Returns `true` if current variant is `Unknown`
    pub fn is_unknown(&self) -> bool {
        unsafe { lib_ruby_parser__external__comment_type__is_unknown(&self.blob) }
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
