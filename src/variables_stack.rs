use lib_ruby_parser_ast::{Blob, DoubleLinkedIntrusiveList, IntrusiveHashMap};

#[derive(Debug)]
pub(crate) struct VariablesStack<'b>(
    DoubleLinkedIntrusiveList<'b, IntrusiveHashMap<'b, &'b str, ()>>,
);

impl<'b> VariablesStack<'b> {
    pub(crate) fn new(blob: &Blob<'b>) -> &'b Self {
        let this = blob.alloc_uninitialized_mut::<Self>();
        this.0 = DoubleLinkedIntrusiveList::new_in_place();
        this
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn push(&self, blob: &'b Blob<'b>) {
        let new_map = IntrusiveHashMap::new_in(blob);
        self.0.push(new_map)
    }

    pub(crate) fn pop(&self) {
        self.0.pop();
    }

    pub(crate) fn declare(&self, name: &'b str, blob: &'b Blob<'b>) {
        let mut top = self
            .0
            .last()
            .expect("expected variables_stack to have at least 1 layer");
        self.pop();
        IntrusiveHashMap::insert(&mut top, name, (), blob);
        self.0.push(top);
    }

    pub(crate) fn is_declared(&self, name: &'b str) -> bool {
        self.0
            .last()
            .expect("expected variables_stack to have at least 1 layer")
            .has_member(name)
    }
}
