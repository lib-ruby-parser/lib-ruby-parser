use super::Comment;
use crate::source::{CommentType, DecodedInput};
use crate::Loc;

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
        Self::make(location, kind)
    }
}
