use crate::codegen::rust::nodes::helpers::{node_field_name, struct_name};

fn contents(node: &lib_ruby_parser_nodes::Node) -> String {
    format!(
        "// This file is autogenerated by {generator}

crate::use_native_or_external!(Ptr);
crate::use_native_or_external!(MaybePtr);
crate::use_native_or_external!(StringPtr);
crate::use_native_or_external!(MaybeStringPtr);
crate::use_native_or_external!(List);
crate::use_native_or_external!(MaybeLoc);

use crate::{{Node, Loc, Bytes}};
#[allow(unused_imports)]
use super::{struct_name};

fn new_loc() -> Loc {{
    Loc::new(1, 2)
}}

#[allow(dead_code)]
fn new_maybe_loc() -> MaybeLoc {{
    MaybeLoc::some(new_loc())
}}

#[allow(dead_code)]
fn new_node() -> Node {{
    Node::new_retry(new_loc())
}}

#[allow(dead_code)]
fn new_node_ptr() -> Ptr<Node> {{
    Ptr::new(new_node())
}}

#[allow(dead_code)]
fn new_maybe_node_ptr() -> MaybePtr<Node> {{
    MaybePtr::some(new_node())
}}

#[allow(dead_code)]
fn new_string_ptr() -> StringPtr {{
    StringPtr::from(\"foo\")
}}

#[allow(dead_code)]
fn new_maybe_string_ptr() -> MaybeStringPtr {{
    MaybeStringPtr::from(Some(String::from(\"foo\")))
}}

#[allow(dead_code)]
fn new_node_list() -> List<Node> {{
    List::from(vec![new_node()])
}}

#[allow(dead_code)]
fn new_u8() -> u8 {{
    42
}}

#[allow(dead_code)]
fn new_bytes() -> Bytes {{
    Bytes::new(vec![1, 2, 3])
}}

{constructor}

{test_constructor}

{test_is}

{test_debug}

{test_partial_eq}

{test_clone}

{test_getters}

{test_setters}

{test_into_internal}
",
        generator = file!(),
        struct_name = struct_name(node),
        constructor = constructor(&node),
        test_constructor = test_constructor(&node),
        test_is = test_is(&node),
        test_debug = test_debug(&node),
        test_partial_eq = test_partial_eq(&node),
        test_clone = test_clone(&node),
        test_getters = test_getters(&node),
        test_setters = test_setters(&node),
        test_into_internal = test_into_internal(&node),
    )
}

pub(crate) fn codegen(node: &lib_ruby_parser_nodes::Node) {
    let dir = super::filename(node);
    let path = format!("src/nodes/types/{}/tests.rs", dir);
    std::fs::write(&path, contents(node)).unwrap();
}

fn constructor(node: &lib_ruby_parser_nodes::Node) -> String {
    let arglist = node
        .fields
        .map(&|field| format!("{}()", new_field_fn(field)))
        .join(", ");

    format!(
        "fn new_test_node() -> Node {{
    Node::new_{lower_name}({arglist})
}}",
        lower_name = node.lower_name(),
        arglist = arglist
    )
}

fn test_constructor(_node: &lib_ruby_parser_nodes::Node) -> String {
    format!(
        "#[test]
fn test_constructor() {{
    let node = new_test_node();
    drop(node);
}}
"
    )
}
fn test_is(node: &lib_ruby_parser_nodes::Node) -> String {
    let mut others = lib_ruby_parser_nodes::nodes().map(&|node| node.lower_name());
    others.retain(|e| e != &node.lower_name());
    let other_assertions = others
        .iter()
        .map(|lower| format!("assert!(!node.is_{}());", lower))
        .collect::<Vec<_>>()
        .join("\n    ");

    format!(
        "#[test]
fn test_is() {{
    let node = new_test_node();
    assert!(node.is_{lower}());

    {other_assertions}
}}
",
        lower = node.lower_name(),
        other_assertions = other_assertions
    )
}
fn test_debug(node: &lib_ruby_parser_nodes::Node) -> String {
    let d_loc = format!("1...2");
    let d_maybe_loc = format!("Some({})", d_loc);
    let d_node = format!("Retry(Retry {{ expression_l: {} }})", d_loc);
    let d_maybe_node = format!("Some({})", d_node);
    let d_node_list = format!("[{}]", d_node);
    let d_string_ptr = format!("\\\"foo\\\"");
    let d_maybe_string_ptr = format!("Some({})", d_string_ptr);
    let d_bytes = format!("Bytes {{ raw: [1, 2, 3] }}");
    let d_u8 = format!("42");

    let fields = node
        .fields
        .map(&|field| {
            let key = node_field_name(field);
            use lib_ruby_parser_nodes::NodeFieldType;
            let value = match field.field_type {
                NodeFieldType::Node => &d_node,
                NodeFieldType::Nodes => &d_node_list,
                NodeFieldType::MaybeNode { .. } => &d_maybe_node,
                NodeFieldType::Loc => &d_loc,
                NodeFieldType::MaybeLoc => &d_maybe_loc,
                NodeFieldType::Str { .. } => &d_string_ptr,
                NodeFieldType::MaybeStr { .. } => &d_maybe_string_ptr,
                NodeFieldType::StringValue => &d_bytes,
                NodeFieldType::U8 => &d_u8,
            };
            format!("{}: {}", key, value)
        })
        .join(", ");

    format!(
        "#[test]
fn test_debug() {{
    assert_eq!(
        format!(\"{{:?}}\", new_test_node()),
        \"{node_type}({node_type} {{ {fields} }})\"
    )
}}
",
        node_type = struct_name(node),
        fields = fields
    )
}
fn test_partial_eq(_node: &lib_ruby_parser_nodes::Node) -> String {
    format!(
        "#[test]
fn test_partial_eq() {{
    let node = new_test_node();
    let same = new_test_node();
    let other = Node::new_retry(Loc::new(100, 200));

    assert_eq!(node, same);
    assert_ne!(node, other);
}}
"
    )
}
fn test_clone(_node: &lib_ruby_parser_nodes::Node) -> String {
    format!(
        "#[test]
fn test_clone() {{
    let node = new_test_node();
    assert_eq!(
        node,
        node.clone()
    );
}}
"
    )
}
fn test_getters(node: &lib_ruby_parser_nodes::Node) -> String {
    let assertions = node
        .fields
        .map(&|field| {
            let lhs = format!("variant.get_{}()", field.field_name);
            let rhs = format!("&{}()", new_field_fn(field));

            format!("assert_eq!({}, {});", lhs, rhs)
        })
        .join("\n    ");

    format!(
        "#[test]
fn test_getters() {{
    let node = new_test_node();
    let variant = node.into_{lower}();

    {assertions}
}}
",
        assertions = assertions,
        lower = node.lower_name()
    )
}
fn test_setters(node: &lib_ruby_parser_nodes::Node) -> String {
    let assertions = node
        .fields
        .map(&|field| {
            let set_field = format!(
                "variant.set_{}({}())",
                field.field_name,
                new_field_fn(field)
            );

            let lhs = format!("variant.get_{}()", field.field_name);
            let rhs = format!("&{}()", new_field_fn(field));

            format!(
                "{set_field};
    assert_eq!({lhs}, {rhs});",
                set_field = set_field,
                lhs = lhs,
                rhs = rhs
            )
        })
        .join("\n    ");

    format!(
        "#[test]
fn test_setters() {{
    let node = new_test_node();
    let mut variant = node.into_{lower}();

    {assertions}
}}
",
        assertions = assertions,
        lower = node.lower_name()
    )
}
fn test_into_internal(node: &lib_ruby_parser_nodes::Node) -> String {
    let assertions = node
        .fields
        .map(&|field| {
            format!(
                "assert_eq!(&internal.{field_name}, &{new_field_fn}());",
                field_name = node_field_name(field),
                new_field_fn = new_field_fn(field)
            )
        })
        .join("\n    ");

    format!(
        "#[test]
fn test_into_internal() {{
    let node = new_test_node();
    let variant = node.into_{lower}();
    let internal = variant.into_internal();

    {assertions}
}}
",
        lower = node.lower_name(),
        assertions = assertions
    )
}

fn new_field_fn(field: &lib_ruby_parser_nodes::NodeField) -> String {
    use lib_ruby_parser_nodes::NodeFieldType;

    match field.field_type {
        NodeFieldType::Node => "new_node_ptr",
        NodeFieldType::Nodes => "new_node_list",
        NodeFieldType::MaybeNode { .. } => "new_maybe_node_ptr",
        NodeFieldType::Loc => "new_loc",
        NodeFieldType::MaybeLoc => "new_maybe_loc",
        NodeFieldType::Str { .. } => "new_string_ptr",
        NodeFieldType::MaybeStr { .. } => "new_maybe_string_ptr",
        NodeFieldType::StringValue => "new_bytes",
        NodeFieldType::U8 => "new_u8",
    }
    .to_string()
}
