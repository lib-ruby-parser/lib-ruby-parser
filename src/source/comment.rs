use crate::source::DecodedInput;
use crate::Loc;

/// Enum of all possible comment types
#[derive(Debug, Clone, PartialEq, Eq)]
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

/// A struct that represents a comment in Ruby
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Comment {
    /// Location of the comment (starts with `#` and ends with the last char)
    pub location: Loc,

    /// Kind of the comment
    pub kind: CommentType,
}

impl Comment {
    /// Constructs a new comment by `Loc` and `Input`
    pub fn new(location: Loc, input: &DecodedInput) -> Self {
        let kind = match location.source(input) {
            Some(source) => {
                if source.starts_with('#') {
                    CommentType::Inline
                } else if source.starts_with("=begin") {
                    CommentType::Document
                } else {
                    CommentType::Unknown
                }
            }
            None => CommentType::Unknown,
        };
        Self { location, kind }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "link-with-external-c-structures")]
    #[test]
    fn test_size_c() {
        use super::Comment;
        assert_eq!(std::mem::size_of::<Comment>(), 20);
    }

    #[cfg(feature = "link-with-external-cpp-structures")]
    #[test]
    fn test_size_cpp() {
        use super::Comment;
        assert_eq!(std::mem::size_of::<Comment>(), 20);
    }
}
