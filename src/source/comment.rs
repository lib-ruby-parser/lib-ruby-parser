use core::{cell::Cell, ptr::NonNull};

use lib_ruby_parser_ast_arena::Blob;

use crate::loc_ext::LocExt;
use crate::source::DecodedInput;
use crate::Loc;

/// Enum of all possible comment types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[repr(C)]
pub struct Comment {
    /// Location of the comment (starts with `#` and ends with the last char)
    pub location: Loc,

    /// Kind of the comment
    pub kind: CommentType,

    next: Cell<Option<NonNull<Self>>>,
}

impl core::fmt::Debug for Comment {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Comment")
            .field("location", &self.location)
            .field("kind", &self.kind)
            .finish()
    }
}

impl PartialEq for Comment {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location && self.kind == other.kind
    }
}

impl Comment {
    /// Constructs a new comment by `Loc` and `Input`
    pub fn from_loc_and_input<'b>(
        location: Loc,
        input: &DecodedInput,
        blob: &'b Blob<'b>,
    ) -> &'b Self {
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
        Self::new(location, kind, blob)
    }

    pub(crate) fn new<'b>(location: Loc, kind: CommentType, blob: &'b Blob<'b>) -> &'b Self {
        let this = blob.alloc_mut();
        *this = Self {
            location,
            kind,
            next: Cell::new(None),
        };
        this
    }
}

impl lib_ruby_parser_ast_arena::SingleLinkedIntrusiveListItem for Comment {
    fn next(&self) -> Option<NonNull<Self>> {
        self.next.get()
    }

    fn set_next(&self, new_next: Option<NonNull<Self>>) {
        self.next.set(new_next)
    }
}
