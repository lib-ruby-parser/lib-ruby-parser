pub(crate) fn filename(node: &lib_ruby_parser_nodes::Node) -> String {
    let lower = node.lower_name();

    match &lower[..] {
        "self" | "break" | "const" | "false" | "for" | "if" | "return" | "str" | "super"
        | "true" | "while" | "yield" => format!("{}_", lower),
        _ => lower,
    }
}
