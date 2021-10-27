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
