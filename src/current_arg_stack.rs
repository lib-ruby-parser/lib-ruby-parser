use core::{cell::Cell, ptr::NonNull};

use lib_ruby_parser_ast_arena::{Blob, DoubleLinkedIntrusiveList};
// Stack that holds names of current arguments,
// i.e. while parsing
//   def m1(a = (def m2(b = def m3(c = 1); end); end)); end
//                                   ^
// stack is [:a, :b, :c]
//
// Emulates `p->cur_arg` in MRI's parse.y
//
//
#[derive(Debug)]
pub(crate) struct CurrentArgStack<'b>(DoubleLinkedIntrusiveList<'b, CurrentArgStackItem<'b>>);

struct CurrentArgStackItem<'b> {
    s: Option<&'b str>,
    prev: Cell<Option<NonNull<Self>>>,
    next: Cell<Option<NonNull<Self>>>,
}

impl core::fmt::Debug for CurrentArgStackItem<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CurrentArgStackItem")
            .field("s", &self.s)
            .finish()
    }
}

impl lib_ruby_parser_ast_arena::DoubleLinkedIntrusiveListItem for CurrentArgStackItem<'_> {
    fn prev(&self) -> Option<NonNull<Self>> {
        self.prev.get()
    }

    fn set_prev(&self, new_prev: Option<NonNull<Self>>) {
        self.prev.set(new_prev)
    }

    fn next(&self) -> Option<NonNull<Self>> {
        self.next.get()
    }

    fn set_next(&self, new_next: Option<NonNull<Self>>) {
        self.next.set(new_next)
    }
}

impl<'b> CurrentArgStack<'b> {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn push(&self, value: Option<&'b str>, blob: &Blob<'b>) {
        let item = blob.alloc_mut::<CurrentArgStackItem>();
        *item = CurrentArgStackItem {
            s: value,
            prev: Cell::new(None),
            next: Cell::new(None),
        };
        self.0.push(item);
    }

    pub(crate) fn set(&self, value: Option<&'b str>, blob: &Blob<'b>) {
        self.pop();
        self.push(value, blob)
    }

    pub(crate) fn pop(&self) {
        self.0.pop()
    }

    pub(crate) fn top(&self) -> Option<&'b str> {
        self.0.last().and_then(|i| i.s)
    }
}
