use crate::source::CommentType;
use crate::Loc;

use crate::containers::size::COMMENT_SIZE;
use crate::loc::LocBlob;
use crate::source::comment_type::CommentTypeBlob;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct CommentBlob {
    blob: [u8; COMMENT_SIZE],
}

/// A struct that represents a comment in Ruby
#[repr(C)]
pub struct Comment {
    blob: CommentBlob,
}

extern "C" {
    fn lib_ruby_parser__external__comment__new(
        location: LocBlob,
        kind: CommentTypeBlob,
    ) -> CommentBlob;
    fn lib_ruby_parser__external__comment__drop(blob: *mut CommentBlob);
    fn lib_ruby_parser__external__comment__get_location(blob: *const CommentBlob)
        -> *const LocBlob;
    fn lib_ruby_parser__external__comment__get_kind(
        blob: *const CommentBlob,
    ) -> *const CommentTypeBlob;
}

impl Comment {
    /// Returns Location of the comment (starts with `#` and ends with the last char)
    pub fn location(&self) -> &Loc {
        unsafe {
            (lib_ruby_parser__external__comment__get_location(&self.blob) as *const Loc)
                .as_ref()
                .unwrap()
        }
    }

    /// Returns kind of the comment
    pub fn kind(&self) -> &CommentType {
        unsafe {
            (lib_ruby_parser__external__comment__get_kind(&self.blob) as *const CommentType)
                .as_ref()
                .unwrap()
        }
    }

    pub(crate) fn make(location: Loc, kind: CommentType) -> Self {
        let blob = unsafe { lib_ruby_parser__external__comment__new(location.blob, kind.blob) };
        Self { blob }
    }
}

impl std::fmt::Debug for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Comment")
            .field("location", &self.location())
            .field("kind", &self.kind())
            .finish()
    }
}

impl Clone for Comment {
    fn clone(&self) -> Self {
        Self::make(self.location().clone(), self.kind().clone())
    }
}

impl PartialEq for Comment {
    fn eq(&self, other: &Self) -> bool {
        self.location() == other.location() && self.kind() == other.kind()
    }
}

impl Eq for Comment {}

impl Drop for Comment {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__external__comment__drop(&mut self.blob) };
    }
}
