use ruby_parser::StaticEnvironment;

#[test]
fn test_declare() {
    let mut env = StaticEnvironment::new();
    assert!(!env.is_declared("foo"));

    env.declare("foo");
    assert!(env.is_declared("foo"));
}

#[test]
fn test_extend_static() {
    let mut env = StaticEnvironment::new();

    env.declare("foo");
    env.extend_static();
    env.declare("bar");

    assert!(!env.is_declared("foo"));
    assert!(env.is_declared("bar"));
}

#[test]
fn test_extend_dynamic() {
    let mut env = StaticEnvironment::new();

    env.declare("foo");
    env.extend_dynamic();
    env.declare("bar");

    assert!(env.is_declared("foo"));
    assert!(env.is_declared("bar"));
}

#[test]
fn test_unextend() {
    let mut env = StaticEnvironment::new();

    env.declare("foo");
    env.extend_dynamic();
    env.declare("bar");
    env.unextend();

    assert!(env.is_declared("foo"));
    assert!(!env.is_declared("bar"));
}
