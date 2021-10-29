use std::cell::RefCell;
use std::rc::Rc;

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
pub(crate) struct Context {
    pub(crate) stack: Rc<RefCell<Vec<ContextItem>>>,
}

impl Context {
    pub(crate) fn new() -> Self {
        Self {
            stack: Rc::new(RefCell::new(vec![])),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.stack.borrow().is_empty()
    }

    fn push(&self, item: ContextItem) {
        self.stack.borrow_mut().push(item);
    }

    pub(crate) fn push_class(&self) {
        self.push(ContextItem::Class)
    }

    pub(crate) fn push_module(&self) {
        self.push(ContextItem::Module)
    }

    pub(crate) fn push_sclass(&self) {
        self.push(ContextItem::Sclass)
    }

    pub(crate) fn push_def(&self) {
        self.push(ContextItem::Def)
    }

    pub(crate) fn push_defs(&self) {
        self.push(ContextItem::Defs)
    }

    pub(crate) fn push_block(&self) {
        self.push(ContextItem::Block)
    }

    pub(crate) fn push_lambda(&self) {
        self.push(ContextItem::Lambda)
    }

    pub(crate) fn pop(&self) {
        self.stack.borrow_mut().pop();
    }

    fn is_in(&self, item: ContextItem) -> bool {
        self.stack.borrow().last() == Some(&item)
    }

    pub(crate) fn is_in_class(&self) -> bool {
        self.is_in(ContextItem::Class)
    }

    #[allow(dead_code)]
    pub(crate) fn is_in_module(&self) -> bool {
        self.is_in(ContextItem::Module)
    }

    #[allow(dead_code)]
    pub(crate) fn is_in_sclass(&self) -> bool {
        self.is_in(ContextItem::Sclass)
    }

    pub(crate) fn is_in_def(&self) -> bool {
        self.is_in(ContextItem::Def)
    }

    #[allow(dead_code)]
    pub(crate) fn is_in_defs(&self) -> bool {
        self.is_in(ContextItem::Defs)
    }

    #[allow(dead_code)]
    pub(crate) fn is_in_block(&self) -> bool {
        self.is_in(ContextItem::Block)
    }

    #[allow(dead_code)]
    pub(crate) fn is_in_lambda(&self) -> bool {
        self.is_in(ContextItem::Lambda)
    }

    #[allow(dead_code)]
    pub(crate) fn is_indirectly_in_def(&self) -> bool {
        let stack = self.stack.borrow();
        stack.contains(&ContextItem::Def) || stack.contains(&ContextItem::Defs)
    }

    pub(crate) fn is_class_definition_allowed(&self) -> bool {
        let stack = self.stack.borrow();
        let def_index: Option<usize> = stack
            .iter()
            .rev()
            .position(|i| *i == ContextItem::Def || *i == ContextItem::Defs);
        let sclass_index: Option<usize> =
            stack.iter().rev().position(|i| *i == ContextItem::Sclass);

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
        self.stack.borrow().clone()
    }
}
