use core::cell::Cell;
use lib_ruby_parser_ast_arena::{Blob, DoubleLinkedIntrusiveList, IntrusiveStrHashMap};

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
#[derive(Debug)]
pub struct StaticEnvironment<'b> {
    variables: Cell<&'b IntrusiveStrHashMap<'b, ()>>,
    stack: &'b DoubleLinkedIntrusiveList<'b, IntrusiveStrHashMap<'b, ()>>,
}

const FORWARD_ARGS: &str = "FORWARD_ARGS";
const ANONYMOUS_BLOCKARG: &str = "ANONYMOUS_BLOCKARG";

impl<'b> StaticEnvironment<'b> {
    pub(crate) fn new(blob: &'b Blob<'b>) -> &'b Self {
        let this = blob.alloc_mut::<Self>();
        this.variables.set(IntrusiveStrHashMap::new_in(blob));
        this.stack = blob.alloc_ref();
        this
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Performs a push, doesn't inherit previously declared variables in the new scope
    ///
    /// Handles class/module scopes
    pub fn extend_static(&self, blob: &'b Blob<'b>) {
        self.stack.push(self.variables.get().shallow_copy(blob));
        self.variables.set(IntrusiveStrHashMap::new_in(blob));
    }

    /// Performs a push, inherits previously declared variables in the new scope
    ///
    /// Handles block/lambda scopes
    pub fn extend_dynamic(&self, blob: &'b Blob<'b>) {
        self.stack.push(self.variables.get().deep_clone(blob))
    }

    /// Performs pop
    pub fn unextend(&self) {
        self.variables.set(
            self.stack
                .last()
                .expect("expected static_env to have at least one frame"),
        );
        self.stack.pop()
    }

    /// Declares a new variable in the current scope
    pub fn declare(&self, name: &'b str, blob: &'b Blob<'b>) {
        let mut variables = self.variables.get();
        IntrusiveStrHashMap::insert(&mut variables, name, (), blob);
        self.variables.set(variables)
    }

    /// Returns `true` if variable with a given `name` is declared in the current scope
    pub fn is_declared(&self, name: &str) -> bool {
        self.variables.get().has_member(name)
    }

    pub(crate) fn declare_forward_args(&self, blob: &'b Blob<'b>) {
        self.declare(FORWARD_ARGS, blob);
    }

    pub(crate) fn is_forward_args_declared(&self) -> bool {
        self.is_declared(FORWARD_ARGS)
    }

    pub(crate) fn declare_anonymous_blockarg(&self, blob: &'b Blob<'b>) {
        self.declare(ANONYMOUS_BLOCKARG, blob)
    }

    pub(crate) fn is_anonymous_blockarg_declared(&self) -> bool {
        self.is_declared(ANONYMOUS_BLOCKARG)
    }
}

#[test]
fn test_declare() {
    let mut mem = [0; 100];
    let blob = Blob::from(&mut mem);

    let env = StaticEnvironment::new(&blob);
    assert!(!env.is_declared("foo"));

    env.declare("foo", &blob);
    assert!(env.is_declared("foo"));
}

#[test]
fn test_extend_static() {
    let mut mem = [0; 100];
    let blob = Blob::from(&mut mem);

    let env = StaticEnvironment::new(&blob);

    env.declare("foo", &blob);
    env.extend_static(&blob);
    env.declare("bar", &blob);

    assert!(!env.is_declared("foo"));
    assert!(env.is_declared("bar"));
}

#[test]
fn test_extend_dynamic() {
    let mut mem = [0; 100];
    let blob = Blob::from(&mut mem);

    let env = StaticEnvironment::new(&blob);

    env.declare("foo", &blob);
    env.extend_dynamic(&blob);
    env.declare("bar", &blob);

    assert!(env.is_declared("foo"));
    assert!(env.is_declared("bar"));
}

#[test]
fn test_unextend() {
    let mut mem = [0; 100];
    let blob = Blob::from(&mut mem);

    let env = StaticEnvironment::new(&blob);

    env.declare("foo", &blob);
    env.extend_dynamic(&blob);
    env.declare("bar", &blob);
    env.unextend();

    assert!(env.is_declared("foo"));
    assert!(!env.is_declared("bar"));
}
