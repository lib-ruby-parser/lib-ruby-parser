#[derive(Debug, Clone, Default)]
struct InnerMaxNumparamStack {
    stack: Vec<i32>,
}

impl InnerMaxNumparamStack {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn set_has_ordinary_params(&mut self) {
        self.set(-1)
    }

    pub fn has_ordinary_params(&self) -> bool {
        self.top() < 0
    }

    pub fn has_numparams(&self) -> bool {
        self.top() > 0
    }

    pub fn register(&mut self, numparam: i32) {
        self.set(std::cmp::max(self.top(), numparam))
    }

    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap_or(&std::i32::MIN)
    }

    pub fn push(&mut self) {
        self.stack.push(0)
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    fn set(&mut self, value: i32) {
        self.stack.pop();
        self.stack.push(value)
    }

    pub fn inner_clone(&self) -> Vec<i32> {
        self.stack.clone()
    }
}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Default)]
pub struct MaxNumparamStack {
    inner: Rc<RefCell<InnerMaxNumparamStack>>,
}

impl MaxNumparamStack {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerMaxNumparamStack::new())),
        }
    }

    pub fn set_has_ordinary_params(&self) {
        self.inner.borrow_mut().set_has_ordinary_params()
    }
    pub fn has_ordinary_params(&self) -> bool {
        self.inner.borrow().has_ordinary_params()
    }
    pub fn has_numparams(&self) -> bool {
        self.inner.borrow().has_numparams()
    }
    pub fn register(&self, numparam: i32) {
        self.inner.borrow_mut().register(numparam)
    }
    pub fn top(&self) -> i32 {
        self.inner.borrow().top()
    }
    pub fn push(&self) {
        self.inner.borrow_mut().push()
    }
    pub fn pop(&self) {
        self.inner.borrow_mut().pop()
    }
    pub fn inner_clone(&self) -> Vec<i32> {
        self.inner.borrow().inner_clone()
    }
}
