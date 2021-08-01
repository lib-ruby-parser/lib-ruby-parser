pub(crate) mod messages {
    pub(crate) fn constructor_name(message: &lib_ruby_parser_nodes::Message) -> String {
        format!(
            "lib_ruby_parser__internal__containers__diagnostic_message__make_{name}",
            name = message.lower_name()
        )
    }

    pub(crate) fn field_name(field: &lib_ruby_parser_nodes::MessageField) -> &str {
        match &field.name[..] {
            "operator" => "operator_",
            other => other,
        }
    }

    pub(crate) fn getter_name(
        message: &lib_ruby_parser_nodes::Message,
        field: &lib_ruby_parser_nodes::MessageField,
    ) -> String {
        format!(
            "{prefix}__{variant}__get_{field_name}",
            prefix = "lib_ruby_parser__internal__containers__diagnostic_message",
            variant = message.lower_name(),
            field_name = field_name(field)
        )
    }

    pub(crate) fn type_predicate_name(message: &lib_ruby_parser_nodes::Message) -> String {
        format!(
            "lib_ruby_parser__internal__containers__diagnostic_message__is_{name}",
            name = message.lower_name()
        )
    }

    pub(crate) fn field_type(field: &lib_ruby_parser_nodes::MessageField) -> &str {
        match field.field_type {
            lib_ruby_parser_nodes::MessageFieldType::Str => "STRING_PTR",
            lib_ruby_parser_nodes::MessageFieldType::Byte => "Byte",
        }
    }

    pub(crate) fn field_blob_type(field: &lib_ruby_parser_nodes::MessageField) -> &str {
        match field.field_type {
            lib_ruby_parser_nodes::MessageFieldType::Str => "STRING_PTR_BLOB",
            lib_ruby_parser_nodes::MessageFieldType::Byte => "Byte_BLOB",
        }
    }

    pub(crate) fn constructor_signature(message: &lib_ruby_parser_nodes::Message) -> String {
        let arglist = message
            .fields
            .map(&|field| {
                format!(
                    "{blob_t} {name}",
                    blob_t = field_blob_type(field),
                    name = field_name(field)
                )
            })
            .join(", ");

        format!(
            "DiagnosticMessage_BLOB {name}({arglist})",
            name = constructor_name(message),
            arglist = arglist
        )
    }

    pub(crate) fn getter_signature(
        message: &lib_ruby_parser_nodes::Message,
        field: &lib_ruby_parser_nodes::MessageField,
    ) -> String {
        let return_type = match field.field_type {
            lib_ruby_parser_nodes::MessageFieldType::Str => "STRING_PTR_BLOB *",
            lib_ruby_parser_nodes::MessageFieldType::Byte => "Byte",
        };

        format!(
            "{return_type} {getter_name}(DiagnosticMessage_BLOB *blob)",
            return_type = return_type,
            getter_name = getter_name(message, field)
        )
    }

    pub(crate) fn type_predicate_signature(message: &lib_ruby_parser_nodes::Message) -> String {
        format!(
            "bool {name}(DiagnosticMessage_BLOB *blob)",
            name = type_predicate_name(message)
        )
    }
}
