crate::use_native_or_external!(Maybe);

use crate::traverse::finder::Finder;
use crate::{debug_level, Parser, ParserOptions};

fn find(src: &str, pattern: &str) -> Option<String> {
    let options = ParserOptions::new(
        "(find_test)".into(),
        debug_level::NONE,
        Maybe::none(),
        Maybe::none(),
        false,
    );
    let parser = Parser::new(src, options);

    let result = parser.do_parse();
    let ast = result.ast().as_ref().expect("expected AST to be Some");
    let node = Finder::run(&pattern, ast).unwrap()?;
    node.expression().source(result.input())
}

#[test]
fn it_finds() {
    let src = "[1,2,3].each { |a| puts a + 1; 42 }";
    let pattern = "root -> body -> stmts -> 0 -> args -> 0";

    assert_eq!(Some("a + 1".to_string()), find(src, pattern))
}

#[test]
fn it_returns_none_if_no_node() {
    let src = "[1,2,3]";
    let pattern = "body";

    assert_eq!(None, find(src, pattern))
}
