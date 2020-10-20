#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum ContextItem {
    Class,
    Module,
    Sclass,
    Def,
    Defs,
    Block,
    Lambda,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct InnerContext {
    pub(crate) stack: Vec<ContextItem>,
}

impl InnerContext {
    pub(crate) fn new() -> Self {
        Self { stack: vec![] }
    }

    fn push(&mut self, item: ContextItem) {
        self.stack.push(item);
    }

    pub(crate) fn push_class(&mut self) {
        self.push(ContextItem::Class)
    }
    pub(crate) fn push_module(&mut self) {
        self.push(ContextItem::Module)
    }
    pub(crate) fn push_sclass(&mut self) {
        self.push(ContextItem::Sclass)
    }
    pub(crate) fn push_def(&mut self) {
        self.push(ContextItem::Def)
    }
    pub(crate) fn push_defs(&mut self) {
        self.push(ContextItem::Defs)
    }
    pub(crate) fn push_block(&mut self) {
        self.push(ContextItem::Block)
    }
    pub(crate) fn push_lambda(&mut self) {
        self.push(ContextItem::Lambda)
    }

    pub(crate) fn pop(&mut self) {
        self.stack.pop();
    }

    fn is_in(&self, item: ContextItem) -> bool {
        self.stack.last() == Some(&item)
    }

    pub(crate) fn is_in_class(&self) -> bool {
        self.is_in(ContextItem::Class)
    }
    pub(crate) fn is_in_module(&self) -> bool {
        self.is_in(ContextItem::Module)
    }
    pub(crate) fn is_in_sclass(&self) -> bool {
        self.is_in(ContextItem::Sclass)
    }
    pub(crate) fn is_in_def(&self) -> bool {
        self.is_in(ContextItem::Def)
    }
    pub(crate) fn is_in_defs(&self) -> bool {
        self.is_in(ContextItem::Defs)
    }
    pub(crate) fn is_in_block(&self) -> bool {
        self.is_in(ContextItem::Block)
    }
    pub(crate) fn is_in_lambda(&self) -> bool {
        self.is_in(ContextItem::Lambda)
    }

    pub(crate) fn reset(&mut self) {
        self.stack.clear()
    }

    pub(crate) fn is_indirectly_in_def(&self) -> bool {
        self.stack.contains(&ContextItem::Def) || self.stack.contains(&ContextItem::Defs)
    }

    pub(crate) fn is_class_definition_allowed(&self) -> bool {
        let def_index: Option<usize> = self
            .stack
            .iter()
            .rev()
            .position(|i| *i == ContextItem::Def || *i == ContextItem::Defs);
        let sclass_index: Option<usize> = self
            .stack
            .iter()
            .rev()
            .position(|i| *i == ContextItem::Sclass);

        match (def_index, sclass_index) {
            (None, _) => true,
            (Some(_), None) => false,
            (Some(def_index), Some(sclass_index)) => sclass_index < def_index,
        }
    }

    pub(crate) fn is_module_definition_allowed(&self) -> bool {
        self.is_class_definition_allowed()
    }

    pub(crate) fn is_dynamic_const_definition_allowed(&self) -> bool {
        self.is_class_definition_allowed()
    }

    pub(crate) fn is_in_dynamic_block(&self) -> bool {
        self.is_in(ContextItem::Block) || self.is_in(ContextItem::Lambda)
    }

    pub(crate) fn inner_clone(&self) -> Vec<ContextItem> {
        self.stack.clone()
    }
}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Default)]
pub(crate) struct Context {
    inner: Rc<RefCell<InnerContext>>,
}

impl Context {
    pub(crate) fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerContext::new())),
        }
    }

    pub(crate) fn push_class(&self) {
        self.inner.borrow_mut().push_class()
    }
    pub(crate) fn push_module(&self) {
        self.inner.borrow_mut().push_module()
    }
    pub(crate) fn push_sclass(&self) {
        self.inner.borrow_mut().push_sclass()
    }
    pub(crate) fn push_def(&self) {
        self.inner.borrow_mut().push_def()
    }
    pub(crate) fn push_defs(&self) {
        self.inner.borrow_mut().push_defs()
    }
    pub(crate) fn push_block(&self) {
        self.inner.borrow_mut().push_block()
    }
    pub(crate) fn push_lambda(&self) {
        self.inner.borrow_mut().push_lambda()
    }

    pub(crate) fn pop(&self) {
        self.inner.borrow_mut().pop();
    }

    pub(crate) fn reset(&self) {
        self.inner.borrow_mut().reset();
    }

    pub(crate) fn is_in_class(&self) -> bool {
        self.inner.borrow().is_in_class()
    }
    pub(crate) fn is_in_module(&self) -> bool {
        self.inner.borrow().is_in_module()
    }
    pub(crate) fn is_in_sclass(&self) -> bool {
        self.inner.borrow().is_in_sclass()
    }
    pub(crate) fn is_in_def(&self) -> bool {
        self.inner.borrow().is_in_def()
    }
    pub(crate) fn is_in_defs(&self) -> bool {
        self.inner.borrow().is_in_defs()
    }
    pub(crate) fn is_in_block(&self) -> bool {
        self.inner.borrow().is_in_block()
    }
    pub(crate) fn is_in_lambda(&self) -> bool {
        self.inner.borrow().is_in_lambda()
    }

    pub(crate) fn is_indirectly_in_def(&self) -> bool {
        self.inner.borrow().is_indirectly_in_def()
    }

    pub(crate) fn is_class_definition_allowed(&self) -> bool {
        self.inner.borrow().is_class_definition_allowed()
    }

    pub(crate) fn is_module_definition_allowed(&self) -> bool {
        self.inner.borrow().is_module_definition_allowed()
    }

    pub(crate) fn is_dynamic_const_definition_allowed(&self) -> bool {
        self.inner.borrow().is_dynamic_const_definition_allowed()
    }

    pub(crate) fn is_in_dynamic_block(&self) -> bool {
        self.inner.borrow().is_in_dynamic_block()
    }

    pub(crate) fn inner_clone(&self) -> Vec<ContextItem> {
        self.inner.borrow().inner_clone()
    }
}
