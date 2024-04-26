use core::cell::Cell;

use lib_ruby_parser_ast::{
    Blob, ConstNonNull, DoubleLinkedIntrusiveList, DoubleLinkedIntrusiveListItem,
};
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
    prev: Cell<Option<ConstNonNull<Self>>>,
    next: Cell<Option<ConstNonNull<Self>>>,
}

impl<'b> CurrentArgStackItem<'b> {
    fn new(value: Option<&'b str>, blob: &Blob<'b>) -> &'b Self {
        let this = blob.alloc_uninitialized_mut::<Self>();
        *this = Self {
            s: value,
            prev: Cell::new(None),
            next: Cell::new(None),
        };
        this
    }
}

impl core::fmt::Debug for CurrentArgStackItem<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CurrentArgStackItem")
            .field("s", &self.s)
            .finish()
    }
}

impl DoubleLinkedIntrusiveListItem for CurrentArgStackItem<'_> {
    fn prev(&self) -> Option<ConstNonNull<Self>> {
        self.prev.get()
    }

    fn set_prev(&self, new_prev: Option<ConstNonNull<Self>>) {
        self.prev.set(new_prev)
    }

    fn next(&self) -> Option<ConstNonNull<Self>> {
        self.next.get()
    }

    fn set_next(&self, new_next: Option<ConstNonNull<Self>>) {
        self.next.set(new_next)
    }
}

impl<'b> CurrentArgStack<'b> {
    pub(crate) fn new(blob: &Blob<'b>) -> &'b Self {
        let this = blob.alloc_uninitialized_mut::<Self>();
        this.0 = DoubleLinkedIntrusiveList::new_in_place();
        this
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn push(&self, value: Option<&'b str>, blob: &Blob<'b>) {
        self.0.push(CurrentArgStackItem::new(value, blob));
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
