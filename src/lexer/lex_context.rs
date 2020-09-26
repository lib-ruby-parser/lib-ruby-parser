#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ContextItem {
    Class,
    Module,
    Sclass,
    Def,
    Defs,
    Block,
    Lambda,
    Defined,
}

#[derive(Debug, Clone, Default)]
struct InnerContext {
    stack: Vec<ContextItem>
}

impl InnerContext {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn push(&mut self, item: ContextItem) {
        self.stack.push(item);
    }

    pub fn pop(&mut self) { self.stack.pop(); }

    pub fn is_in(&self, item: ContextItem) -> bool {
        self.stack.last() == Some(&item)
    }

    pub fn reset(&mut self) { self.stack.clear() }

    pub fn is_indirectly_in_def(&self) -> bool {
        self.stack.contains(&ContextItem::Def) || self.stack.contains(&ContextItem::Defs)
    }

    pub fn is_class_definition_allowed(&self) -> bool {
        unimplemented!("is_class_definition_allow")
    }

    pub fn is_module_definition_allowed(&self) -> bool {
        self.is_class_definition_allowed()
    }

    pub fn is_dynamic_const_definition_allowed(&self) -> bool {
        self.is_class_definition_allowed()
    }

    pub fn is_in_dynamic_block(&self) -> bool {
        self.is_in(ContextItem::Block) || self.is_in(ContextItem::Lambda)
    }
}

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, Default)]
pub struct Context {
    inner: Rc<RefCell<InnerContext>>
}

impl Context {
    pub fn push(&self, item: ContextItem) {
        self.inner.borrow_mut().push(item);
    }

    pub fn pop(&self) {
        self.inner.borrow_mut().pop();
    }

    pub fn reset(&self) {
        self.inner.borrow_mut().reset();
    }

    pub fn is_in(&self, item: ContextItem) -> bool {
        self.inner.borrow().is_in(item)
    }

    pub fn is_indirectly_in_def(&self) -> bool {
        self.inner.borrow().is_indirectly_in_def()
    }

    pub fn is_class_definition_allowed(&self) -> bool {
        self.inner.borrow().is_class_definition_allowed()
    }

    pub fn is_module_definition_allowed(&self) -> bool {
        self.inner.borrow().is_module_definition_allowed()
    }

    pub fn is_dynamic_const_definition_allowed(&self) -> bool {
        self.inner.borrow().is_dynamic_const_definition_allowed()
    }

    pub fn is_in_dynamic_block(&self) -> bool {
        self.inner.borrow().is_in_dynamic_block()
    }
}
