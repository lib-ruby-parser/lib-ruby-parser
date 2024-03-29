use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub(crate) struct SharedContext {
    value: Rc<RefCell<Context>>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub(crate) struct Context {
    value: usize,
}

macro_rules! context_flag {
    ($upper:ident, $getter:ident, $setter: ident, $value:expr) => {
        impl Context {
            const $upper: usize = $value;

            pub(crate) fn $getter(&self) -> bool {
                (self.value & Self::$upper) != 0
            }

            pub(crate) fn $setter(&mut self, value: bool) {
                if cfg!(feature = "debug-parser") {
                    println!("{}({})", stringify!($setter), value);
                }
                if value {
                    self.value |= Self::$upper;
                } else {
                    self.value &= !Self::$upper;
                }
            }
        }

        impl SharedContext {
            #[allow(dead_code)]
            pub(crate) fn $getter(&self) -> bool {
                self.value.borrow().$getter()
            }

            pub(crate) fn $setter(&mut self, value: bool) {
                self.value.borrow_mut().$setter(value)
            }
        }
    };
}

context_flag!(IN_DEFINED, in_defined, set_in_defined, 1 << 0);
context_flag!(IN_KWARG, in_kwarg, set_in_kwarg, 1 << 1);
context_flag!(IN_ARGDEF, in_argdef, set_in_argdef, 1 << 2);
context_flag!(IN_DEF, in_def, set_in_def, 1 << 3);
context_flag!(IN_CLASS, in_class, set_in_class, 1 << 4);
context_flag!(IN_LAMBDA, in_lambda, set_in_lambda, 1 << 5);
context_flag!(IN_BLOCK, in_block, set_in_block, 1 << 6);

impl SharedContext {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn dump(&self) -> Context {
        *self.value.borrow()
    }

    pub(crate) fn is_in_dynamic_block(&self) -> bool {
        self.in_block() || self.in_lambda()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.value.borrow().is_empty()
    }
}

impl Context {
    fn is_empty(&self) -> bool {
        if cfg!(debug_assertions) && self.value != 0 {
            println!(
                "Context is not empty;
    value = {};
    IN_DEFINED = {}
    IN_KWARG = {}
    IN_ARGDEF = {}
    IN_DEF = {}
    IN_CLASS = {}
    IN_LAMBDA = {}
    IN_BLOCK = {}",
                self.value,
                self.in_defined(),
                self.in_kwarg(),
                self.in_argdef(),
                self.in_def(),
                self.in_class(),
                self.in_lambda(),
                self.in_block(),
            );
        }
        self.value == 0
    }
}

#[test]
fn test_context() {
    let mut context = Context::default();

    context.set_in_def(true);
    context.set_in_class(true);
    assert!(context.in_def());
    assert!(context.in_class());

    context.set_in_def(false);
    assert!(!context.in_def());
    assert!(context.in_class());

    context.set_in_class(false);
    assert!(!context.in_def());
    assert!(!context.in_class());

    assert!(context.is_empty());
}
