use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
struct InnerVariablesStack {
    stack: Vec<HashSet<String>>,
}

impl InnerVariablesStack {
    pub(crate) fn new() -> Self {
        let mut instance = Self { stack: vec![] };
        instance.push();
        instance
    }

    pub(crate) fn push(&mut self) {
        self.stack.push(HashSet::new())
    }

    pub(crate) fn pop(&mut self) {
        self.stack.pop();
    }

    pub(crate) fn reset(&mut self) {
        self.stack.clear()
    }

    pub(crate) fn declare(&mut self, name: &str) {
        self.stack.last_mut().unwrap().insert(name.to_owned());
    }

    pub(crate) fn is_declared(&mut self, name: &str) -> bool {
        self.stack.last().unwrap().contains(name)
    }
}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Default)]
pub(crate) struct VariablesStack {
    inner: Rc<RefCell<InnerVariablesStack>>,
}

impl VariablesStack {
    pub(crate) fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerVariablesStack::new())),
        }
    }

    pub(crate) fn push(&mut self) {
        self.inner.borrow_mut().push()
    }
    pub(crate) fn pop(&mut self) {
        self.inner.borrow_mut().pop()
    }
    pub(crate) fn reset(&mut self) {
        self.inner.borrow_mut().reset()
    }
    pub(crate) fn declare(&mut self, name: &str) {
        self.inner.borrow_mut().declare(name)
    }
    pub(crate) fn is_declared(&mut self, name: &str) -> bool {
        self.inner.borrow_mut().is_declared(name)
    }
}
