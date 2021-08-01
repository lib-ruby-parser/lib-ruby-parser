pub(crate) mod messages {
    pub(crate) fn field_name(field: &lib_ruby_parser_nodes::MessageField) -> &str {
        match &field.name[..] {
            "operator" => "operator_",
            other => other,
        }
    }

    pub(crate) fn field_type(field: &lib_ruby_parser_nodes::MessageField) -> &str {
        match field.field_type {
            lib_ruby_parser_nodes::MessageFieldType::Str => "STRING_PTR",
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
}
