use lib_ruby_parser_nodes::{
    template::TemplateFns, Message, MessageWithField, Node, NodeWithField,
};

mod nodes {
    use super::*;

    pub(crate) fn camelcase_name(node: &Node) -> String {
        node.camelcase_name.to_owned()
    }

    pub(crate) fn upper_name(node: &Node) -> String {
        node.upper_name()
    }

    pub(crate) fn lower_name(node: &Node) -> String {
        node.lower_name()
    }
}

mod node_fields {
    use super::*;

    pub(crate) fn name(node_with_field: &NodeWithField) -> String {
        node_with_field.field.snakecase_name.to_string()
    }
}

mod messages {
    use super::*;

    pub(crate) fn camelcase_name(message: &Message) -> String {
        message.camelcase_name.to_owned()
    }

    pub(crate) fn upper_name(message: &Message) -> String {
        message.upper_name()
    }

    pub(crate) fn lower_name(message: &Message) -> String {
        message.lower_name()
    }
}

mod message_fields {
    use super::*;

    #[cfg(feature = "lib-ruby-parser-bindings")]
    pub(crate) fn name(message_with_field: &MessageWithField) -> String {
        lib_ruby_parser_bindings::helpers::messages::fields::field_name(&message_with_field.field)
            .to_string()
    }

    pub(crate) fn c_field_type(message_with_field: &MessageWithField) -> String {
        match message_with_field.field.field_type {
            lib_ruby_parser_nodes::MessageFieldType::Str => "LIB_RUBY_PARSER_StringPtr",
            lib_ruby_parser_nodes::MessageFieldType::Byte => "LIB_RUBY_PARSER_Byte",
        }
        .to_string()
    }

    pub(crate) fn c_blob_type(message_with_field: &MessageWithField) -> String {
        format!("{}_BLOB", c_field_type(message_with_field))
    }
}

pub(crate) fn build() -> TemplateFns {
    let mut fns = TemplateFns::new();

    fns.register_helper("node-camelcase-name", nodes::camelcase_name);
    fns.register_helper("node-upper-name", nodes::upper_name);
    fns.register_helper("node-lower-name", nodes::lower_name);

    fns.register_helper("node-field-name", node_fields::name);

    fns.register_helper("message-camelcase-name", messages::camelcase_name);
    fns.register_helper("message-upper-name", messages::upper_name);
    fns.register_helper("message-lower-name", messages::lower_name);

    #[cfg(feature = "lib-ruby-parser-bindings")]
    fns.register_helper("message-field-name", message_fields::name);
    fns.register_helper("message-field-c-field-type", message_fields::c_field_type);
    fns.register_helper("message-field-c-blob-type", message_fields::c_blob_type);

    fns
}

macro_rules! default_fns {
    () => {{
        fn generated_by(_: &lib_ruby_parser_nodes::template::GlobalContext) -> String {
            file!().to_string()
        }
        let mut fns = $crate::codegen::fns::build();
        fns.register_helper("generated-by", generated_by);
        fns
    }};
}
pub(crate) use default_fns;
