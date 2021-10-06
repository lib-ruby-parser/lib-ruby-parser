pub(crate) mod nodes {
    pub(crate) fn field_name(field: &lib_ruby_parser_nodes::NodeField) -> String {
        lib_ruby_parser_bindings::helpers::nodes::fields::field_name(field)
    }

    pub(crate) fn field_type(field: &lib_ruby_parser_nodes::NodeField) -> &str {
        use lib_ruby_parser_nodes::NodeFieldType;

        match field.field_type {
            NodeFieldType::Node => "NodePtr",
            NodeFieldType::Nodes => "NodeList",
            NodeFieldType::MaybeNode { .. } => "MaybeNodePtr",
            NodeFieldType::Loc => "Loc",
            NodeFieldType::MaybeLoc => "MaybeLoc",
            NodeFieldType::Str { .. } => "StringPtr",
            NodeFieldType::MaybeStr { .. } => "MaybeStringPtr",
            NodeFieldType::StringValue => "Bytes",
            NodeFieldType::U8 => "Byte",
        }
    }
}
