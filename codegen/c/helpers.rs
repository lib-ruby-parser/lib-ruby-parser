use lib_ruby_parser_bindings::helpers::nodes as default_node_helpers;

pub(crate) mod messages {
    pub(crate) mod fields {
        pub(crate) fn field_type(field: &lib_ruby_parser_nodes::MessageField) -> &str {
            match field.field_type {
                lib_ruby_parser_nodes::MessageFieldType::Str => "StringPtr",
                lib_ruby_parser_nodes::MessageFieldType::Byte => "Byte",
            }
        }

        pub(crate) fn blob_type(field: &lib_ruby_parser_nodes::MessageField) -> String {
            format!("{}_BLOB", field_type(field))
        }
    }
}

pub(crate) mod nodes {
    use super::default_node_helpers;

    pub(crate) mod fields {
        use super::default_node_helpers;

        pub(crate) fn field_name(field: &lib_ruby_parser_nodes::NodeField) -> String {
            default_node_helpers::fields::field_name(field)
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

        pub(crate) fn internal_field_type(field: &lib_ruby_parser_nodes::NodeField) -> &str {
            use lib_ruby_parser_nodes::NodeFieldType;

            match field.field_type {
                NodeFieldType::Node => "Ptr",
                NodeFieldType::Nodes => "NodeList",
                NodeFieldType::MaybeNode { .. } => "MaybePtr",
                NodeFieldType::Loc => "Loc",
                NodeFieldType::MaybeLoc => "MaybeLoc",
                NodeFieldType::Str { .. } => "StringPtr",
                NodeFieldType::MaybeStr { .. } => "MaybeStringPtr",
                NodeFieldType::StringValue => "Bytes",
                NodeFieldType::U8 => "Byte",
            }
        }

        pub(crate) fn pack_field_fn(field: &lib_ruby_parser_nodes::NodeField) -> String {
            format!("PACK_{}", internal_field_type(field))
        }

        pub(crate) fn unpack_field_fn(field: &lib_ruby_parser_nodes::NodeField) -> String {
            format!("UNPACK_{}", internal_field_type(field))
        }

        pub(crate) fn blob_type(field: &lib_ruby_parser_nodes::NodeField) -> String {
            format!("{}_BLOB", internal_field_type(field))
        }
    }

    pub(crate) fn enum_variant_name(node: &lib_ruby_parser_nodes::Node) -> String {
        format!("NODE_{}", node.upper_name())
    }

    pub(crate) fn union_member_name(node: &lib_ruby_parser_nodes::Node) -> String {
        let lower = node.lower_name();
        match &lower[..] {
            "and" | "break" | "case" | "class" | "const" | "false" | "float" | "for" | "if"
            | "int" | "or" | "return" | "true" | "while" => format!("{}_", lower),
            other => other.to_owned(),
        }
    }
}
