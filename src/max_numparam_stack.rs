use core::{cell::Cell, ptr::NonNull};
use lib_ruby_parser_ast_arena::{Blob, DoubleLinkedIntrusiveList, DoubleLinkedIntrusiveListItem};

pub(crate) struct MaxNumparamStackItem {
    pub(crate) value: Cell<i32>,
    pub(crate) is_static: bool,

    prev: Cell<Option<NonNull<Self>>>,
    next: Cell<Option<NonNull<Self>>>,
}
impl core::fmt::Debug for MaxNumparamStackItem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MaxNumparamStackItem")
            .field("value", &self.value)
            .field("is_static", &self.is_static)
            .finish()
    }
}
impl DoubleLinkedIntrusiveListItem for MaxNumparamStackItem {
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

#[derive(Debug)]
pub(crate) struct MaxNumparamStack<'b>(DoubleLinkedIntrusiveList<'b, MaxNumparamStackItem>);

impl<'b> MaxNumparamStack<'b> {
    const ORDINARY_PARAMS: i32 = -1;

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn set_has_ordinary_params(&self) {
        self.set(Self::ORDINARY_PARAMS)
    }

    pub(crate) fn has_ordinary_params(&self) -> bool {
        self.top() == Self::ORDINARY_PARAMS
    }

    pub(crate) fn has_numparams(&self) -> bool {
        self.top() > 0
    }

    pub(crate) fn register(&self, numparam: i32) {
        self.set(core::cmp::max(self.top(), numparam))
    }

    pub(crate) fn top(&self) -> i32 {
        self.0.last().map(|i| i.value.get()).unwrap_or(i32::MIN)
    }

    pub(crate) fn push(&self, is_static: bool, blob: &Blob<'b>) {
        let item = blob.alloc_mut::<MaxNumparamStackItem>();
        *item = MaxNumparamStackItem {
            value: Cell::new(0),
            is_static,
            prev: Cell::new(None),
            next: Cell::new(None),
        };
        self.0.push(item)
    }

    pub(crate) fn pop(&self) {
        self.0.pop();
    }

    fn set(&self, value: i32) {
        let second_last = self.0.last().unwrap();
        second_last.value.set(value);
    }

    pub(crate) fn iter(&self) -> impl DoubleEndedIterator<Item = &'b MaxNumparamStackItem> {
        self.0.iter()
    }
}
