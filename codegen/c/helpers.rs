use lib_ruby_parser_bindings::helpers::nodes as default_node_helpers;

pub(crate) mod messages {
    pub(crate) mod fields {
        pub(crate) fn field_type(field: &lib_ruby_parser_nodes::MessageField) -> &str {
            match field.field_type {
                lib_ruby_parser_nodes::MessageFieldType::Str => "LIB_RUBY_PARSER_StringPtr",
                lib_ruby_parser_nodes::MessageFieldType::Byte => "LIB_RUBY_PARSER_Byte",
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
                NodeFieldType::Node => "LIB_RUBY_PARSER_NodePtr",
                NodeFieldType::Nodes => "LIB_RUBY_PARSER_NodeList",
                NodeFieldType::MaybeNode { .. } => "LIB_RUBY_PARSER_MaybeNodePtr",
                NodeFieldType::Loc => "LIB_RUBY_PARSER_Loc",
                NodeFieldType::MaybeLoc => "LIB_RUBY_PARSER_MaybeLoc",
                NodeFieldType::Str { .. } => "LIB_RUBY_PARSER_StringPtr",
                NodeFieldType::MaybeStr { .. } => "LIB_RUBY_PARSER_MaybeStringPtr",
                NodeFieldType::StringValue => "LIB_RUBY_PARSER_Bytes",
                NodeFieldType::U8 => "LIB_RUBY_PARSER_Byte",
            }
        }

        pub(crate) fn internal_field_type(field: &lib_ruby_parser_nodes::NodeField) -> &str {
            use lib_ruby_parser_nodes::NodeFieldType;

            match field.field_type {
                NodeFieldType::Node => "LIB_RUBY_PARSER_Ptr",
                NodeFieldType::Nodes => "LIB_RUBY_PARSER_NodeList",
                NodeFieldType::MaybeNode { .. } => "LIB_RUBY_PARSER_MaybePtr",
                NodeFieldType::Loc => "LIB_RUBY_PARSER_Loc",
                NodeFieldType::MaybeLoc => "LIB_RUBY_PARSER_MaybeLoc",
                NodeFieldType::Str { .. } => "LIB_RUBY_PARSER_StringPtr",
                NodeFieldType::MaybeStr { .. } => "LIB_RUBY_PARSER_MaybeStringPtr",
                NodeFieldType::StringValue => "LIB_RUBY_PARSER_Bytes",
                NodeFieldType::U8 => "LIB_RUBY_PARSER_Byte",
            }
        }

        pub(crate) fn pack_field_fn(field: &lib_ruby_parser_nodes::NodeField) -> &str {
            use lib_ruby_parser_nodes::NodeFieldType;

            match field.field_type {
                NodeFieldType::Node => "PACK_Ptr",
                NodeFieldType::Nodes => "PACK_NodeList",
                NodeFieldType::MaybeNode { .. } => "PACK_MaybePtr",
                NodeFieldType::Loc => "PACK_Loc",
                NodeFieldType::MaybeLoc => "PACK_MaybeLoc",
                NodeFieldType::Str { .. } => "PACK_StringPtr",
                NodeFieldType::MaybeStr { .. } => "PACK_MaybeStringPtr",
                NodeFieldType::StringValue => "PACK_Bytes",
                NodeFieldType::U8 => "PACK_Byte",
            }
        }

        pub(crate) fn unpack_field_fn(field: &lib_ruby_parser_nodes::NodeField) -> &str {
            use lib_ruby_parser_nodes::NodeFieldType;

            match field.field_type {
                NodeFieldType::Node => "UNPACK_Ptr",
                NodeFieldType::Nodes => "UNPACK_NodeList",
                NodeFieldType::MaybeNode { .. } => "UNPACK_MaybePtr",
                NodeFieldType::Loc => "UNPACK_Loc",
                NodeFieldType::MaybeLoc => "UNPACK_MaybeLoc",
                NodeFieldType::Str { .. } => "UNPACK_StringPtr",
                NodeFieldType::MaybeStr { .. } => "UNPACK_MaybeStringPtr",
                NodeFieldType::StringValue => "UNPACK_Bytes",
                NodeFieldType::U8 => "UNPACK_Byte",
            }
        }

        pub(crate) fn blob_type(field: &lib_ruby_parser_nodes::NodeField) -> String {
            format!("{}_BLOB", internal_field_type(field))
        }
    }

    pub(crate) fn enum_variant_name(node: &lib_ruby_parser_nodes::Node) -> String {
        format!("LIB_RUBY_PARSER_NODE_{}", node.upper_name())
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
