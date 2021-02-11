use crate::source::buffer::Input;
use crate::Loc;

#[derive(Debug, Clone, PartialEq)]
pub enum CommentType {
    Inline,
    Document,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Comment {
    pub location: Loc,
    pub kind: CommentType,
}

impl Comment {
    pub fn new(location: Loc, input: &Input) -> Self {
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
