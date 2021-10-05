pub(crate) mod messages {
    pub(crate) mod fields {
        pub(crate) fn field_type(field: &lib_ruby_parser_nodes::MessageField) -> &str {
            match field.field_type {
                lib_ruby_parser_nodes::MessageFieldType::Str => "LIB_RUBY_PARSER_StringPtr",
                lib_ruby_parser_nodes::MessageFieldType::Byte => "LIB_RUBY_PARSER_Byte",
            }
        }
    }
}
