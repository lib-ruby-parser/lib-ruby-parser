use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;

#[derive(Debug, Clone, Default)]
pub(crate) struct VariablesStack {
    stack: Rc<RefCell<Vec<BTreeSet<String>>>>,
}

impl VariablesStack {
    pub(crate) fn new() -> Self {
        Self {
            stack: Rc::new(RefCell::new(vec![])),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.stack.borrow().is_empty()
    }

    pub(crate) fn push(&self) {
        self.stack.borrow_mut().push(BTreeSet::new())
    }

    pub(crate) fn pop(&self) {
        self.stack.borrow_mut().pop();
    }

    #[allow(dead_code)]
    pub(crate) fn reset(&self) {
        self.stack.borrow_mut().clear()
    }

    pub(crate) fn declare(&self, name: &str) {
        self.stack
            .borrow_mut()
            .last_mut()
            .expect("expected variables_stack to have at least 1 layer")
            .insert(name.to_owned());
    }

    pub(crate) fn is_declared(&self, name: &str) -> bool {
        self.stack
            .borrow()
            .last()
            .expect("expected variables_stack to have at least 1 layer")
            .contains(name)
    }
}
