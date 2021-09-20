#[allow(dead_code)]
pub(crate) fn field_type(field: &lib_ruby_parser_nodes::NodeField) -> &str {
    use lib_ruby_parser_nodes::NodeFieldType;

    match field.field_type {
        NodeFieldType::Node => "Ptr<Node>",
        NodeFieldType::Nodes => "List<Node>",
        NodeFieldType::MaybeNode { .. } => "Maybe<Ptr<Node>>",
        NodeFieldType::Loc => "Loc",
        NodeFieldType::MaybeLoc => "Maybe<Loc>",
        NodeFieldType::Str { .. } => "StringPtr",
        NodeFieldType::MaybeStr { .. } => "Maybe<StringPtr>",
        NodeFieldType::StringValue => "Bytes",
        NodeFieldType::U8 => "u8",
    }
}

pub(crate) fn filename(node: &lib_ruby_parser_nodes::Node) -> String {
    let lower = node.lower_name();

    match &lower[..] {
        "self" | "break" | "const" | "false" | "for" | "if" | "return" | "str" | "super"
        | "true" | "while" | "yield" => format!("{}_", lower),
        _ => lower,
    }
}

pub(crate) fn struct_name(node: &lib_ruby_parser_nodes::Node) -> String {
    let camelcase_name = node.camelcase_name.to_owned();

    match &camelcase_name[..] {
        "Self" => format!("{}_", camelcase_name),
        _ => camelcase_name,
    }
}

pub(crate) fn node_field_name(field: &lib_ruby_parser_nodes::NodeField) -> String {
    let name = field.field_name.to_owned();

    match &name[..] {
        "const" | "as" | "else" => format!("{}_", name),
        _ => name,
    }
}
