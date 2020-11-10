use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Default)]
pub(crate) struct MaxNumparamStack {
    stack: Rc<RefCell<Vec<i32>>>,
}

impl MaxNumparamStack {
    pub(crate) fn new() -> Self {
        Self {
            stack: Rc::new(RefCell::new(vec![])),
        }
    }

    pub(crate) fn set_has_ordinary_params(&self) {
        self.set(-1)
    }

    pub(crate) fn has_ordinary_params(&self) -> bool {
        self.top() < 0
    }

    pub(crate) fn has_numparams(&self) -> bool {
        self.top() > 0
    }

    pub(crate) fn register(&self, numparam: i32) {
        self.set(std::cmp::max(self.top(), numparam))
    }

    pub(crate) fn top(&self) -> i32 {
        *self.stack.borrow().last().unwrap_or(&std::i32::MIN)
    }

    pub(crate) fn push(&self) {
        self.stack.borrow_mut().push(0)
    }

    pub(crate) fn pop(&self) {
        self.stack.borrow_mut().pop();
    }

    fn set(&self, value: i32) {
        let mut stack = self.stack.borrow_mut();
        stack.pop();
        stack.push(value)
    }

    pub(crate) fn inner_clone(&self) -> Vec<i32> {
        self.stack.borrow().clone()
    }
}
