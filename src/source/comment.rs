use crate::source::{CommentType, DecodedInput};
use crate::Loc;

#[cfg(not(feature = "compile-with-external-structures"))]
mod comment {
    use super::{CommentType, Loc};

    /// A struct that represents a comment in Ruby
    #[repr(C)]
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
}

#[cfg(feature = "compile-with-external-structures")]
mod comment {
    use super::{CommentType, Loc};
    use crate::containers::size::COMMENT_SIZE;
    use crate::loc::LocBlob;
    use crate::source::comment_type::CommentTypeBlob;

    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    struct CommentBlob {
        blob: [u8; COMMENT_SIZE],
    }

    /// A struct that represents a comment in Ruby
    #[repr(C)]
    pub struct Comment {
        blob: CommentBlob,
    }

    extern "C" {
        fn lib_ruby_parser__internal__containers__comment__get_location(
            blob: *const CommentBlob,
        ) -> *const LocBlob;
        fn lib_ruby_parser__internal__containers__comment__get_kind(
            blob: *const CommentBlob,
        ) -> *const CommentTypeBlob;
        fn lib_ruby_parser__internal__containers__comment__make(
            location: LocBlob,
            kind: CommentTypeBlob,
        ) -> CommentBlob;
    }

    impl Comment {
        /// Returns Location of the comment (starts with `#` and ends with the last char)
        pub fn location(&self) -> &Loc {
            let comment_blob_ptr: *const CommentBlob = &self.blob;
            let loc_ptr = unsafe {
                lib_ruby_parser__internal__containers__comment__get_location(comment_blob_ptr)
                    as *const Loc
            };
            unsafe { loc_ptr.as_ref().unwrap() }
        }

        /// Returns kind of the comment
        pub fn kind(&self) -> &CommentType {
            let comment_blob_ptr: *const CommentBlob = &self.blob;
            let coment_type_ptr = unsafe {
                lib_ruby_parser__internal__containers__comment__get_kind(comment_blob_ptr)
                    as *const CommentType
            };
            unsafe { coment_type_ptr.as_ref().unwrap() }
        }

        pub(crate) fn make(location: Loc, kind: CommentType) -> Self {
            let blob = unsafe {
                lib_ruby_parser__internal__containers__comment__make(location.blob, kind.blob)
            };
            Self { blob }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{Comment, CommentType, Loc};

        #[test]
        fn test_comment_type() {
            let comment = Comment::make(Loc::new(1, 2), CommentType::inline());

            assert_eq!(comment.location().begin(), 1);
            assert_eq!(comment.location().end(), 2);
            assert!(comment.kind().is_inline());
        }
    }
}

pub use comment::Comment;

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

#[cfg(test)]
mod tests {
    use super::{Comment, CommentType, Loc};

    fn comment() -> Comment {
        Comment::make(Loc::new(1, 2), CommentType::document())
    }

    #[test]
    fn test_debug() {
        assert_eq!(
            format!("{:?}", comment()),
            "Comment { location: 1...2, kind: Document }"
        )
    }

    #[test]
    fn test_compare() {
        assert_eq!(
            Comment::make(Loc::new(1, 2), CommentType::document()),
            comment()
        );

        assert_ne!(
            Comment::make(Loc::new(2, 2), CommentType::document()),
            comment()
        );

        assert_ne!(
            Comment::make(Loc::new(1, 3), CommentType::document()),
            comment()
        );

        assert_ne!(
            Comment::make(Loc::new(1, 2), CommentType::inline()),
            comment()
        );
    }

    #[test]
    fn test_clone() {
        let comment = comment().clone();
        assert_eq!(comment.location(), &Loc::new(1, 2));
        assert_eq!(comment.kind(), &CommentType::document());
    }
}
