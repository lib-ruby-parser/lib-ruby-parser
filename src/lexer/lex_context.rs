#[derive(Debug, Clone, Eq, PartialEq)]
enum Item {
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
pub struct LexContext {
    stack: Vec<Item>
}

macro_rules! has_state {
    (push_fn = $push_fn:ident, is_fn = $is_fn:ident, variant = $value:expr) => {
        pub fn $push_fn(&mut self) {
            self.stack.push($value)
        }

        pub fn $is_fn(&self) -> bool {
            self.stack.last() == Some(&$value)
        }
    };
}

impl LexContext {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    has_state!(push_fn = push_class,   is_fn = is_in_class,   variant = Item::Class);
    has_state!(push_fn = push_module,  is_fn = is_in_module,  variant = Item::Module);
    has_state!(push_fn = push_sclass,  is_fn = is_in_sclass,  variant = Item::Sclass);
    has_state!(push_fn = push_def,     is_fn = is_in_def,     variant = Item::Def);
    has_state!(push_fn = push_defs,    is_fn = is_in_defs,    variant = Item::Defs);
    has_state!(push_fn = push_block,   is_fn = is_in_block,   variant = Item::Block);
    has_state!(push_fn = push_lambda,  is_fn = is_in_lambda,  variant = Item::Lambda);
    has_state!(push_fn = push_defined, is_fn = is_in_defined, variant = Item::Defined);

    pub fn pop(&mut self) { self.stack.pop(); }

    pub fn reset(&mut self) { self.stack.clear() }

    pub fn is_indirectly_in_def(&self) -> bool {
        self.stack.contains(&Item::Def) || self.stack.contains(&Item::Defs)
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
        self.is_in_block() || self.is_in_lambda()
    }
}
