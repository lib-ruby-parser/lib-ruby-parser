use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub enum CommentType {
    Inline,
    Document,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Comment {
    pub location: Range,
}

impl Comment {
    pub fn new(location: Range) -> Self {
        Self { location }
    }

    pub fn kind(&self) -> CommentType {
        match self.location.source() {
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
        }
    }

    pub fn is_inline(&self) -> bool {
        self.kind() == CommentType::Inline
    }

    pub fn is_document(&self) -> bool {
        self.kind() == CommentType::Document
    }
}
