use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
struct InnerStaticEnvironment {
    variables: HashSet<String>,
    stack: Vec<HashSet<String>>,
}

const FORWARD_ARGS: &'static str = "FORWARD_ARGS";

impl InnerStaticEnvironment {
    pub fn new() -> Self {
        Self { variables: HashSet::new(), stack: vec![] }
    }

    pub fn reset(&mut self) {
        self.variables.clear();
        self.stack.clear();
    }

    pub fn extend_static(&mut self) {
        let mut variables: HashSet<String> = HashSet::new();
        std::mem::swap(&mut variables, &mut self.variables);
        self.stack.push(variables);
    }

    pub fn extend_dynamic(&mut self) {
        self.stack.push(self.variables.clone());
    }

    pub fn unextend(&mut self) {
        self.variables = self.stack.pop().unwrap();
    }

    pub fn declare(&mut self, name: &str) {
        self.variables.insert(name.to_owned());
    }

    pub fn is_declared(&self, name: &str) -> bool {
        self.variables.get(name).is_some()
    }

    pub fn declare_forward_args(&mut self) {
        self.declare(FORWARD_ARGS);
    }

    pub fn is_forward_args_declared(&self) -> bool {
        self.is_declared(FORWARD_ARGS)
    }
}

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, Default)]
pub struct StaticEnvironment {
    inner: Rc<RefCell<InnerStaticEnvironment>>
}

impl StaticEnvironment {
    pub fn new() -> Self { Self { inner: Rc::new(RefCell::new(InnerStaticEnvironment::new())) } }
    pub fn reset(&self) { self.inner.borrow_mut().reset() }
    pub fn extend_static(&self) { self.inner.borrow_mut().extend_static() }
    pub fn extend_dynamic(&self) { self.inner.borrow_mut().extend_dynamic() }
    pub fn unextend(&self) { self.inner.borrow_mut().unextend() }
    pub fn declare(&self, name: &str) { self.inner.borrow_mut().declare(name) }
    pub fn is_declared(&self, name: &str) -> bool { self.inner.borrow().is_declared(name) }
    pub fn declare_forward_args(&self) { self.inner.borrow_mut().declare_forward_args() }
    pub fn is_forward_args_declared(&self) -> bool { self.inner.borrow().is_forward_args_declared() }
}
