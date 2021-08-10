use crate::source::MagicCommentKind;
use crate::Loc;

use crate::containers::size::MAGIC_COMMENT_SIZE;
use crate::loc::LocBlob;
use crate::source::magic_comment_kind::MagicCommentKindBlob;

use crate::containers::helpers::IntoBlob;

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
    fn lib_ruby_parser__internal__containers__magic_comment__new(
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
    fn lib_ruby_parser__internal__containers__magic_comment__drop(blob: *mut MagicCommentBlob);
}

impl MagicComment {
    /// Constructor
    pub fn new(kind: MagicCommentKind, key_l: Loc, value_l: Loc) -> Self {
        let blob = unsafe {
            lib_ruby_parser__internal__containers__magic_comment__new(
                kind.into_blob(),
                key_l.into_blob(),
                value_l.into_blob(),
            )
        };
        Self { blob }
    }

    /// Returns kind of the of the MagicComment
    pub fn kind(&self) -> &MagicCommentKind {
        unsafe {
            (lib_ruby_parser__internal__containers__magic_comment__get_kind(&self.blob)
                as *const MagicCommentKind)
                .as_ref()
                .unwrap()
        }
    }
    /// Returns location of MagicComment's key
    pub fn key_l(&self) -> &Loc {
        unsafe {
            (lib_ruby_parser__internal__containers__magic_comment__get_key_l(&self.blob)
                as *const Loc)
                .as_ref()
                .unwrap()
        }
    }
    /// Returns location of MagicComment's value
    pub fn value_l(&self) -> &Loc {
        unsafe {
            (lib_ruby_parser__internal__containers__magic_comment__get_value_l(&self.blob)
                as *const Loc)
                .as_ref()
                .unwrap()
        }
    }
}

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

impl Drop for MagicComment {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__internal__containers__magic_comment__drop(&mut self.blob) }
    }
}
