use std::cell::RefCell;
use std::rc::Rc;
// Stack that holds names of current arguments,
// i.e. while parsing
//   def m1(a = (def m2(b = def m3(c = 1); end); end)); end
//                                   ^
// stack is [:a, :b, :c]
//
// Emulates `p->cur_arg` in MRI's parse.y
//
//
#[derive(Debug, Clone, Default)]
pub(crate) struct CurrentArgStack {
    stack: Rc<RefCell<Vec<Option<String>>>>,
}

impl CurrentArgStack {
    pub(crate) fn new() -> Self {
        Self {
            stack: Rc::new(RefCell::new(vec![])),
        }
    }

    pub(crate) fn push(&self, value: Option<String>) {
        self.stack.borrow_mut().push(value)
    }

    pub(crate) fn set(&self, value: Option<String>) {
        self.pop();
        self.push(value)
    }

    pub(crate) fn pop(&self) {
        self.stack.borrow_mut().pop();
    }

    #[allow(dead_code)]
    pub(crate) fn reset(&self) {
        self.stack.borrow_mut().clear()
    }

    pub(crate) fn top(&self) -> Option<String> {
        match self.stack.borrow().last() {
            Some(Some(value)) => Some(value.clone()),
            _ => None,
        }
    }
}
