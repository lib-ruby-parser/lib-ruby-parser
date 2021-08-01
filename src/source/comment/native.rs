use crate::source::CommentType;
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
    /// Returns Location of the comment (starts with `#` and ends with the last char)
    pub fn location(&self) -> &Loc {
        &self.location
    }

    /// Returns kind of the comment
    pub fn kind(&self) -> &CommentType {
        &self.kind
    }

    pub(crate) fn make(location: Loc, kind: CommentType) -> Self {
        Self { location, kind }
    }
}
