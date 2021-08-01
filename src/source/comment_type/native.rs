/// Enum of all possible comment types
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
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
