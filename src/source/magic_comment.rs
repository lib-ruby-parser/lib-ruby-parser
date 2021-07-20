use crate::source::MagicCommentKind;
use crate::Loc;

/// Representation of a magic comment in Ruby
#[derive(Debug, Clone, PartialEq)]
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
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "link-with-external-c-structures")]
    #[test]
    fn test_size_c() {
        use super::MagicComment;
        assert_eq!(std::mem::size_of::<MagicComment>(), 36);
    }

    #[cfg(feature = "link-with-external-cpp-structures")]
    #[test]
    fn test_size_cpp() {
        use super::MagicComment;
        assert_eq!(std::mem::size_of::<MagicComment>(), 36);
    }
}
