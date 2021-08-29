pub(crate) mod messages {
    pub(crate) fn field_name(field: &lib_ruby_parser_nodes::MessageField) -> &str {
        match &field.name[..] {
            "operator" => "operator_",
            other => other,
        }
    }

    pub(crate) fn field_type(field: &lib_ruby_parser_nodes::MessageField) -> &str {
        match field.field_type {
            lib_ruby_parser_nodes::MessageFieldType::Str => "StringPtr",
            lib_ruby_parser_nodes::MessageFieldType::Byte => "Byte",
        }
    }

    pub(crate) fn constructor_arglist(message: &lib_ruby_parser_nodes::Message) -> String {
        message
            .fields
            .map(&|field| {
                format!(
                    "{t} {name}",
                    t = field_type(field),
                    name = field_name(field)
                )
            })
            .join(", ")
    }

    pub(crate) fn blob_type(field: &lib_ruby_parser_nodes::MessageField) -> String {
        format!("{}_BLOB", field_type(field))
    }
}

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
