use crate::source::MagicCommentKind;
use crate::Loc;

#[cfg(not(feature = "compile-with-external-structures"))]
mod magic_comment {
    use super::{Loc, MagicCommentKind};

    /// Representation of a magic comment in Ruby
    #[repr(C)]
    pub struct MagicComment {
        /// Kind of a magic comment
        pub kind: MagicCommentKind,

        /// Location of the "key":
        ///
        /// ```text
        /// # encoding: utf-8
        ///   ~~~~~~~~
        /// ```
        pub key_l: Loc,

        /// Location of the "value":
        ///
        /// ```text
        /// # encoding: utf-8
        ///             ~~~~~
        /// ```
        pub value_l: Loc,
    }

    impl MagicComment {
        /// Constructor
        pub fn new(kind: MagicCommentKind, key_l: Loc, value_l: Loc) -> Self {
            Self {
                kind,
                key_l,
                value_l,
            }
        }

        /// Returns kind of the of the MagicComment
        pub fn kind(&self) -> &MagicCommentKind {
            &self.kind
        }
        /// Returns location of MagicComment's key
        pub fn key_l(&self) -> &Loc {
            &self.key_l
        }
        /// Returns location of MagicComment's value
        pub fn value_l(&self) -> &Loc {
            &self.value_l
        }
    }
}

#[cfg(feature = "compile-with-external-structures")]
mod magic_comment {
    use super::{Loc, MagicCommentKind};
    use crate::containers::size::MAGIC_COMMENT_SIZE;
    use crate::loc::LocBlob;
    use crate::source::magic_comment_kind::MagicCommentKindBlob;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub(crate) struct MagicCommentBlob {
        blob: [u8; MAGIC_COMMENT_SIZE],
    }

    /// An enum of all magic comment kinds
    #[repr(C)]
    pub struct MagicComment {
        pub(crate) blob: MagicCommentBlob,
    }

    extern "C" {
        fn lib_ruby_parser__internal__containers__magic_comment__make(
            kind: MagicCommentKindBlob,
            key_l: LocBlob,
            value_l: LocBlob,
        ) -> MagicCommentBlob;
        fn lib_ruby_parser__internal__containers__magic_comment__get_kind(
            blob: *const MagicCommentBlob,
        ) -> *const MagicCommentKindBlob;
        fn lib_ruby_parser__internal__containers__magic_comment__get_key_l(
            blob: *const MagicCommentBlob,
        ) -> *const LocBlob;
        fn lib_ruby_parser__internal__containers__magic_comment__get_value_l(
            blob: *const MagicCommentBlob,
        ) -> *const LocBlob;
    }

    impl MagicComment {
        /// Constructor
        pub fn new(kind: MagicCommentKind, key_l: Loc, value_l: Loc) -> Self {
            let blob = unsafe {
                lib_ruby_parser__internal__containers__magic_comment__make(
                    kind.blob,
                    key_l.blob,
                    value_l.blob,
                )
            };
            Self { blob }
        }

        /// Returns kind of the of the MagicComment
        pub fn kind(&self) -> &MagicCommentKind {
            let self_blob_ptr: *const MagicCommentBlob = &self.blob;
            let kind_blob_ptr = unsafe {
                lib_ruby_parser__internal__containers__magic_comment__get_kind(self_blob_ptr)
            };
            unsafe { (kind_blob_ptr as *const MagicCommentKind).as_ref().unwrap() }
        }
        /// Returns location of MagicComment's key
        pub fn key_l(&self) -> &Loc {
            let self_blob_ptr: *const MagicCommentBlob = &self.blob;
            let key_l_blob_ptr = unsafe {
                lib_ruby_parser__internal__containers__magic_comment__get_key_l(self_blob_ptr)
            };
            unsafe { (key_l_blob_ptr as *const Loc).as_ref().unwrap() }
        }
        /// Returns location of MagicComment's value
        pub fn value_l(&self) -> &Loc {
            let self_blob_ptr: *const MagicCommentBlob = &self.blob;
            let value_l_blob_ptr = unsafe {
                lib_ruby_parser__internal__containers__magic_comment__get_value_l(self_blob_ptr)
            };
            unsafe { (value_l_blob_ptr as *const Loc).as_ref().unwrap() }
        }
    }
}

pub use magic_comment::MagicComment;

impl std::fmt::Debug for MagicComment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MagicComment")
            .field("kind", &self.kind())
            .field("key_l", &self.key_l())
            .field("value_l", &self.value_l())
            .finish()
    }
}
impl Clone for MagicComment {
    fn clone(&self) -> Self {
        Self::new(
            self.kind().clone(),
            self.key_l().clone(),
            self.value_l().clone(),
        )
    }
}
impl PartialEq for MagicComment {
    fn eq(&self, other: &Self) -> bool {
        (self.kind() == other.kind())
            && (self.key_l() == other.key_l())
            && (self.value_l() == other.value_l())
    }
}

#[cfg(test)]
mod tests {
    use super::{Loc, MagicComment, MagicCommentKind};

    fn new_magic_comment() -> MagicComment {
        MagicComment::new(
            MagicCommentKind::frozen_string_literal(),
            Loc::new(1, 2),
            Loc::new(3, 4),
        )
    }

    #[test]
    fn test_new() {
        let magic_comment = new_magic_comment();

        assert!(magic_comment.kind().is_frozen_string_literal());
        assert_eq!(magic_comment.key_l().begin(), 1);
        assert_eq!(magic_comment.key_l().end(), 2);
        assert_eq!(magic_comment.value_l().begin(), 3);
        assert_eq!(magic_comment.value_l().end(), 4);
    }

    #[test]
    fn test_debug() {
        let magic_comment = new_magic_comment();

        assert_eq!(
            format!("{:?}", magic_comment),
            "MagicComment { kind: FrozenStringLiteral, key_l: 1...2, value_l: 3...4 }"
        )
    }

    #[test]
    fn test_cmp() {
        let magic_comment = new_magic_comment();

        assert_eq!(
            magic_comment,
            MagicComment::new(
                MagicCommentKind::frozen_string_literal(),
                Loc::new(1, 2),
                Loc::new(3, 4),
            )
        );

        assert_ne!(
            magic_comment,
            MagicComment::new(MagicCommentKind::encoding(), Loc::new(1, 2), Loc::new(3, 4),)
        );

        assert_ne!(
            magic_comment,
            MagicComment::new(
                MagicCommentKind::frozen_string_literal(),
                Loc::new(0, 2),
                Loc::new(3, 4),
            )
        );

        assert_ne!(
            magic_comment,
            MagicComment::new(
                MagicCommentKind::frozen_string_literal(),
                Loc::new(1, 0),
                Loc::new(3, 4),
            )
        );

        assert_ne!(
            magic_comment,
            MagicComment::new(
                MagicCommentKind::frozen_string_literal(),
                Loc::new(1, 2),
                Loc::new(0, 4),
            )
        );

        assert_ne!(
            magic_comment,
            MagicComment::new(
                MagicCommentKind::frozen_string_literal(),
                Loc::new(1, 2),
                Loc::new(3, 0),
            )
        );
    }

    #[test]
    fn test_clone() {
        let magic_comment = new_magic_comment();

        assert_eq!(magic_comment, magic_comment.clone())
    }
}
