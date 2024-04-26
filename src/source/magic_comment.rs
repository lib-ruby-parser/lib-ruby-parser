use core::cell::Cell;
use lib_ruby_parser_ast::{Blob, ConstNonNull};

use crate::Loc;

/// An enum of all magic comment kinds
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MagicCommentKind {
    /// `# encoding: ... comment`
    Encoding,

    /// `# frozen_string_literal: true/false` comment
    FrozenStringLiteral,

    /// `# warn_indent: true/false` comment
    WarnIndent,

    /// `# shareable_constant_value: ...` comment
    ShareableConstantValue,
}

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

    next: Cell<Option<ConstNonNull<Self>>>,
}

impl core::fmt::Debug for MagicComment {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MagicComment")
            .field("kind", &self.kind)
            .field("key_l", &self.key_l)
            .field("value_l", &self.value_l)
            .finish()
    }
}

impl PartialEq for MagicComment {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.key_l == other.key_l && self.value_l == other.value_l
    }
}

impl lib_ruby_parser_ast::SingleLinkedIntrusiveListItem for MagicComment {
    fn next(&self) -> Option<ConstNonNull<Self>> {
        self.next.get()
    }

    fn set_next(&self, new_next: Option<ConstNonNull<Self>>) {
        self.next.set(new_next)
    }
}

impl MagicComment {
    pub(crate) fn new<'b>(
        kind: MagicCommentKind,
        key_l: Loc,
        value_l: Loc,
        blob: &Blob<'b>,
    ) -> &'b Self {
        let this = blob.alloc_uninitialized_mut::<Self>();
        *this = Self {
            kind,
            key_l,
            value_l,
            next: Cell::new(None),
        };
        this
    }
}
