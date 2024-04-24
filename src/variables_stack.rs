use lib_ruby_parser_ast::{Blob, DoubleLinkedIntrusiveList, IntrusiveStrHashMap};

#[derive(Debug)]
pub(crate) struct VariablesStack<'b>(DoubleLinkedIntrusiveList<'b, IntrusiveStrHashMap<'b, ()>>);

impl<'b> VariablesStack<'b> {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn push(&self, blob: &'b Blob<'b>) {
        let new_map = IntrusiveStrHashMap::new_in(blob);
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
        IntrusiveStrHashMap::insert(&mut top, name, (), blob);
        self.0.push(top);
    }

    pub(crate) fn is_declared(&self, name: &str) -> bool {
        self.0
            .last()
            .expect("expected variables_stack to have at least 1 layer")
            .has_member(name)
    }
}
