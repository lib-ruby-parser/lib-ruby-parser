#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ContextItem {
    Class,
    Module,
    Sclass,
    Def,
    Defs,
    Block,
    Lambda,
}

#[derive(Debug, Clone, Default)]
pub struct InnerContext {
    pub stack: Vec<ContextItem>
}

impl InnerContext {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    fn push(&mut self, item: ContextItem) {
        self.stack.push(item);
    }

    pub fn push_class(&mut self)  { self.push(ContextItem::Class) }
    pub fn push_module(&mut self) { self.push(ContextItem::Module) }
    pub fn push_sclass(&mut self) { self.push(ContextItem::Sclass) }
    pub fn push_def(&mut self)    { self.push(ContextItem::Def) }
    pub fn push_defs(&mut self)   { self.push(ContextItem::Defs) }
    pub fn push_block(&mut self)  { self.push(ContextItem::Block) }
    pub fn push_lambda(&mut self) { self.push(ContextItem::Lambda) }

    pub fn pop(&mut self) { self.stack.pop(); }

    fn is_in(&self, item: ContextItem) -> bool {
        self.stack.last() == Some(&item)
    }

    pub fn is_in_class(&self)  -> bool { self.is_in(ContextItem::Class) }
    pub fn is_in_module(&self) -> bool { self.is_in(ContextItem::Module) }
    pub fn is_in_sclass(&self) -> bool { self.is_in(ContextItem::Sclass) }
    pub fn is_in_def(&self)    -> bool { self.is_in(ContextItem::Def) }
    pub fn is_in_defs(&self)   -> bool { self.is_in(ContextItem::Defs) }
    pub fn is_in_block(&self)  -> bool { self.is_in(ContextItem::Block) }
    pub fn is_in_lambda(&self) -> bool { self.is_in(ContextItem::Lambda) }

    pub fn reset(&mut self) { self.stack.clear() }

    pub fn is_indirectly_in_def(&self) -> bool {
        self.stack.contains(&ContextItem::Def) || self.stack.contains(&ContextItem::Defs)
    }

    pub fn is_class_definition_allowed(&self) -> bool {
        let def_index: Option<usize> = self.stack.iter().rev().position(|i| *i == ContextItem::Def || *i == ContextItem::Defs);
        let sclass_index: Option<usize> = self.stack.iter().rev().position(|i| *i == ContextItem::Sclass);

        match (def_index, sclass_index) {
            (None, _) => true,
            (Some(_), None) => false,
            (Some(def_index), Some(sclass_index)) => sclass_index < def_index
        }
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

    pub fn inner_clone(&self) -> Vec<ContextItem> {
        self.stack.clone()
    }
}

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, Default)]
pub struct Context {
    inner: Rc<RefCell<InnerContext>>
}

impl Context {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerContext::new()))
        }
    }

    pub fn push_class(&self)  { self.inner.borrow_mut().push_class() }
    pub fn push_module(&self) { self.inner.borrow_mut().push_module() }
    pub fn push_sclass(&self) { self.inner.borrow_mut().push_sclass() }
    pub fn push_def(&self)    { self.inner.borrow_mut().push_def() }
    pub fn push_defs(&self)   { self.inner.borrow_mut().push_defs() }
    pub fn push_block(&self)  { self.inner.borrow_mut().push_block() }
    pub fn push_lambda(&self) { self.inner.borrow_mut().push_lambda() }

    pub fn pop(&self) {
        self.inner.borrow_mut().pop();
    }

    pub fn reset(&self) {
        self.inner.borrow_mut().reset();
    }

    pub fn is_in_class(&self)  -> bool { self.inner.borrow().is_in_class() }
    pub fn is_in_module(&self) -> bool { self.inner.borrow().is_in_module() }
    pub fn is_in_sclass(&self) -> bool { self.inner.borrow().is_in_sclass() }
    pub fn is_in_def(&self)    -> bool { self.inner.borrow().is_in_def() }
    pub fn is_in_defs(&self)   -> bool { self.inner.borrow().is_in_defs() }
    pub fn is_in_block(&self)  -> bool { self.inner.borrow().is_in_block() }
    pub fn is_in_lambda(&self) -> bool { self.inner.borrow().is_in_lambda() }

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

    pub fn inner_clone(&self) -> Vec<ContextItem> {
        self.inner.borrow().inner_clone()
    }
}
