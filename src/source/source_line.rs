use core::cell::Cell;

use lib_ruby_parser_ast::{Blob, ConstNonNull};

#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
/// Representation of a source line in a source file
pub struct SourceLine {
    /// Start of the line (in bytes)
    pub start: usize,

    /// End of the line (in bytes)
    pub end: usize,

    /// `true` if line ends with EOF char (which is true for the last line in the file)
    pub ends_with_eof: bool,

    next: Cell<Option<ConstNonNull<Self>>>,
}

impl SourceLine {
    pub(crate) fn new<'b>(
        start: usize,
        end: usize,
        ends_with_eof: bool,
        blob: &'b Blob<'b>,
    ) -> &'b mut Self {
        let this = blob.alloc_uninitialized_mut();
        *this = Self {
            start,
            end,
            ends_with_eof,
            next: Cell::new(None),
        };
        this
    }
    /// Returns length of the line
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns true if SourceLine is empty (i.e. has `len = 0`)
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns location of the last non-EOF, non-EOL character
    pub fn line_end(&self) -> usize {
        let mut result = self.end;
        if !self.ends_with_eof {
            result -= 1 // exclude trailing \n
        }
        result
    }
}

impl lib_ruby_parser_ast::SingleLinkedIntrusiveListItem for SourceLine {
    fn next(&self) -> Option<ConstNonNull<Self>> {
        self.next.get()
    }

    fn set_next(&self, new_next: Option<ConstNonNull<Self>>) {
        self.next.set(new_next)
    }
}
