use ruby_parser::StaticEnvironment;

#[test]
fn test_declare() {
    let env = StaticEnvironment::new();
    assert!(!env.is_declared(&b"foo".to_vec()));

    env.declare(&b"foo".to_vec());
    assert!(env.is_declared(&b"foo".to_vec()));
}

#[test]
fn test_extend_static() {
    let env = StaticEnvironment::new();

    env.declare(&b"foo".to_vec());
    env.extend_static();
    env.declare(&b"bar".to_vec());

    assert!(!env.is_declared(&b"foo".to_vec()));
    assert!(env.is_declared(&b"bar".to_vec()));
}

#[test]
fn test_extend_dynamic() {
    let env = StaticEnvironment::new();

    env.declare(&b"foo".to_vec());
    env.extend_dynamic();
    env.declare(&b"bar".to_vec());

    assert!(env.is_declared(&b"foo".to_vec()));
    assert!(env.is_declared(&b"bar".to_vec()));
}

#[test]
fn test_unextend() {
    let env = StaticEnvironment::new();

    env.declare(&b"foo".to_vec());
    env.extend_dynamic();
    env.declare(&b"bar".to_vec());
    env.unextend();

    assert!(env.is_declared(&b"foo".to_vec()));
    assert!(!env.is_declared(&b"bar".to_vec()));
}
