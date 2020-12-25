use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, Default)]
pub struct StaticEnvironment {
    variables: Rc<RefCell<HashSet<String>>>,
    stack: Rc<RefCell<Vec<HashSet<String>>>>,
}

const FORWARD_ARGS: &str = "FORWARD_ARGS";

impl StaticEnvironment {
    pub fn new() -> Self {
        Self {
            variables: Rc::new(RefCell::new(HashSet::new())),
            stack: Rc::new(RefCell::new(vec![])),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.stack.borrow().is_empty()
    }

    #[allow(dead_code)]
    pub(crate) fn reset(&self) {
        self.variables.borrow_mut().clear();
        self.stack.borrow_mut().clear();
    }

    pub fn extend_static(&self) {
        let variables = std::mem::take(&mut *self.variables.borrow_mut());
        self.stack.borrow_mut().push(variables);
    }

    pub fn extend_dynamic(&self) {
        self.stack
            .borrow_mut()
            .push(self.variables.borrow().clone());
    }

    pub fn unextend(&self) {
        *self.variables.borrow_mut() = self
            .stack
            .borrow_mut()
            .pop()
            .expect("expected static_env to have at least one frame");
    }

    pub fn declare(&self, name: &str) {
        self.variables.borrow_mut().insert(name.to_owned());
    }

    pub fn is_declared(&self, name: &str) -> bool {
        self.variables.borrow().get(name).is_some()
    }

    pub(crate) fn declare_forward_args(&self) {
        self.declare(FORWARD_ARGS);
    }

    pub(crate) fn is_forward_args_declared(&self) -> bool {
        self.is_declared(FORWARD_ARGS)
    }
}
