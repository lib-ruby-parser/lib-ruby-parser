use crate::source::{CommentType, DecodedInput};
use crate::Loc;

/// A struct that represents a comment in Ruby
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
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
                    CommentType::inline()
                } else if source.starts_with("=begin") {
                    CommentType::document()
                } else {
                    CommentType::unknown()
                }
            }
            None => CommentType::unknown(),
        };
        Self { location, kind }
    }
}
