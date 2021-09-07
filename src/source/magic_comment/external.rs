use crate::blobs::{Blob, HasBlob};
use crate::source::MagicCommentKind;
use crate::Loc;

/// An enum of all magic comment kinds
#[repr(C)]
pub struct MagicComment {
    pub(crate) blob: Blob<MagicComment>,
}

extern "C" {
    fn lib_ruby_parser__external__magic_comment__new(
        kind: Blob<MagicCommentKind>,
        key_l: Blob<Loc>,
        value_l: Blob<Loc>,
    ) -> Blob<MagicComment>;
    fn lib_ruby_parser__external__magic_comment__drop(blob: *mut Blob<MagicComment>);
    fn lib_ruby_parser__external__magic_comment__get_kind(
        blob: *const Blob<MagicComment>,
    ) -> *const Blob<MagicCommentKind>;
    fn lib_ruby_parser__external__magic_comment__get_key_l(
        blob: *const Blob<MagicComment>,
    ) -> *const Blob<Loc>;
    fn lib_ruby_parser__external__magic_comment__get_value_l(
        blob: *const Blob<MagicComment>,
    ) -> *const Blob<Loc>;
}

impl MagicComment {
    /// Constructor
    pub fn new(kind: MagicCommentKind, key_l: Loc, value_l: Loc) -> Self {
        let blob = unsafe {
            lib_ruby_parser__external__magic_comment__new(
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
            (lib_ruby_parser__external__magic_comment__get_kind(&self.blob)
                as *const MagicCommentKind)
                .as_ref()
                .unwrap()
        }
    }
    /// Returns location of MagicComment's key
    pub fn key_l(&self) -> &Loc {
        unsafe {
            (lib_ruby_parser__external__magic_comment__get_key_l(&self.blob) as *const Loc)
                .as_ref()
                .unwrap()
        }
    }
    /// Returns location of MagicComment's value
    pub fn value_l(&self) -> &Loc {
        unsafe {
            (lib_ruby_parser__external__magic_comment__get_value_l(&self.blob) as *const Loc)
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
        unsafe { lib_ruby_parser__external__magic_comment__drop(&mut self.blob) }
    }
}
