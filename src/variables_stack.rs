use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
struct InnerVariablesStack {
    stack: Vec<HashSet<String>>
}

impl InnerVariablesStack {
    pub fn new() -> Self {
        let mut instance = Self { stack: vec![] };
        instance.push();
        instance
    }

    pub fn push(&mut self) {
        self.stack.push(HashSet::new())
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn reset(&mut self) {
        self.stack.clear()
    }

    pub fn declare(&mut self, name: &str) {
        self.stack.last_mut().unwrap().insert(name.to_owned());
    }

    pub fn is_declared(&mut self, name: &str) -> bool {
        self.stack.last().unwrap().contains(name)
    }
}

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, Default)]
pub struct VariablesStack {
    inner: Rc<RefCell<InnerVariablesStack>>
}

impl VariablesStack {
    pub fn new() -> Self {
        Self { inner: Rc::new(RefCell::new(InnerVariablesStack::new())) }
    }

    pub fn push(&mut self) { self.inner.borrow_mut().push() }
    pub fn pop(&mut self) { self.inner.borrow_mut().pop() }
    pub fn reset(&mut self) { self.inner.borrow_mut().reset() }
    pub fn declare(&mut self, name: &str) { self.inner.borrow_mut().declare(name) }
    pub fn is_declared(&mut self, name: &str) -> bool { self.inner.borrow_mut().is_declared(name) }
}
