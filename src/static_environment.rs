use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;

/// Stack of local variables in nested scopes
///
/// Each scope represents a Ruby scope:
///
/// ```test
/// # 1
/// class A
///   # 1, 2
///   def m
///     # 1, 2, 3
///   end
///   # 1, 2
/// end
/// # 1
/// ```
///
/// In the example above comments show what's in the stack.
/// Basically, it's pushed when you enter a new scope
/// and it's popped when exit it.
#[derive(Debug, Clone, Default)]
pub struct StaticEnvironment {
    variables: Rc<RefCell<BTreeSet<String>>>,
    stack: Rc<RefCell<Vec<BTreeSet<String>>>>,
}

const FORWARD_ARGS: &str = "FORWARD_ARGS";
const ANONYMOUS_BLOCKARG: &str = "ANONYMOUS_BLOCKARG";

impl StaticEnvironment {
    /// Constructor
    pub fn new() -> Self {
        Self {
            variables: Rc::new(RefCell::new(BTreeSet::new())),
            stack: Rc::new(RefCell::new(vec![])),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.stack.borrow().is_empty()
    }

    /// Performs a push, doesn't inherit previously declared variables in the new scope
    ///
    /// Handles class/module scopes
    pub fn extend_static(&self) {
        let variables = std::mem::take(&mut *self.variables.borrow_mut());
        self.stack.borrow_mut().push(variables);
    }

    /// Performs a push, inherits previously declared variables in the new scope
    ///
    /// Handles block/lambda scopes
    pub fn extend_dynamic(&self) {
        self.stack
            .borrow_mut()
            .push(self.variables.borrow().clone());
    }

    /// Performs pop
    pub fn unextend(&self) {
        *self.variables.borrow_mut() = self
            .stack
            .borrow_mut()
            .pop()
            .expect("expected static_env to have at least one frame");
    }

    /// Declares a new variable in the current scope
    pub fn declare(&self, name: &str) {
        self.variables.borrow_mut().insert(name.to_string());
    }

    /// Returns `true` if variable with a given `name` is declared in the current scope
    pub fn is_declared(&self, name: &str) -> bool {
        self.variables.borrow().get(name).is_some()
    }

    pub(crate) fn declare_forward_args(&self) {
        self.declare(FORWARD_ARGS);
    }

    pub(crate) fn is_forward_args_declared(&self) -> bool {
        self.is_declared(FORWARD_ARGS)
    }

    pub(crate) fn declare_anonymous_blockarg(&self) {
        self.declare(ANONYMOUS_BLOCKARG)
    }

    pub(crate) fn is_anonymous_blockarg_declared(&self) -> bool {
        self.is_declared(ANONYMOUS_BLOCKARG)
    }
}

#[test]
fn test_declare() {
    let env = StaticEnvironment::new();
    assert!(!env.is_declared("foo"));

    env.declare("foo");
    assert!(env.is_declared("foo"));
}

#[test]
fn test_extend_static() {
    let env = StaticEnvironment::new();

    env.declare("foo");
    env.extend_static();
    env.declare("bar");

    assert!(!env.is_declared("foo"));
    assert!(env.is_declared("bar"));
}

#[test]
fn test_extend_dynamic() {
    let env = StaticEnvironment::new();

    env.declare("foo");
    env.extend_dynamic();
    env.declare("bar");

    assert!(env.is_declared("foo"));
    assert!(env.is_declared("bar"));
}

#[test]
fn test_unextend() {
    let env = StaticEnvironment::new();

    env.declare("foo");
    env.extend_dynamic();
    env.declare("bar");
    env.unextend();

    assert!(env.is_declared("foo"));
    assert!(!env.is_declared("bar"));
}
