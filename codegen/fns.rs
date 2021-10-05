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

    pub(crate) fn c_enum_variant_name(node: &Node) -> String {
        format!("LIB_RUBY_PARSER_NODE_{}", node.upper_name())
    }

    pub(crate) fn c_union_member_name(node: &Node) -> String {
        let lower = node.lower_name();
        match &lower[..] {
            "and" | "break" | "case" | "class" | "const" | "false" | "float" | "for" | "if"
            | "int" | "or" | "return" | "true" | "while" => format!("{}_", lower),
            other => other.to_owned(),
        }
    }
}

mod node_fields {
    use super::*;

    pub(crate) fn name(node_with_field: &NodeWithField) -> String {
        node_with_field.field.snakecase_name.to_string()
    }

    #[cfg(feature = "lib-ruby-parser-bindings")]
    pub(crate) fn c_name(node_with_field: &NodeWithField) -> String {
        lib_ruby_parser_bindings::helpers::nodes::fields::field_name(&node_with_field.field)
    }

    pub(crate) fn c_pack_fn_name(node_with_field: &NodeWithField) -> String {
        use lib_ruby_parser_nodes::NodeFieldType::*;

        match node_with_field.field.field_type {
            Node => "PACK_Ptr",
            Nodes => "PACK_NodeList",
            MaybeNode { .. } => "PACK_MaybePtr",
            Loc => "PACK_Loc",
            MaybeLoc => "PACK_MaybeLoc",
            Str { .. } => "PACK_StringPtr",
            MaybeStr { .. } => "PACK_MaybeStringPtr",
            StringValue => "PACK_Bytes",
            U8 => "PACK_Byte",
        }
        .to_string()
    }

    pub(crate) fn c_unpack_fn_name(node_with_field: &NodeWithField) -> String {
        use lib_ruby_parser_nodes::NodeFieldType::*;

        match node_with_field.field.field_type {
            Node => "UNPACK_Ptr",
            Nodes => "UNPACK_NodeList",
            MaybeNode { .. } => "UNPACK_MaybePtr",
            Loc => "UNPACK_Loc",
            MaybeLoc => "UNPACK_MaybeLoc",
            Str { .. } => "UNPACK_StringPtr",
            MaybeStr { .. } => "UNPACK_MaybeStringPtr",
            StringValue => "UNPACK_Bytes",
            U8 => "UNPACK_Byte",
        }
        .to_string()
    }

    pub(crate) fn c_field_type(node_with_field: &NodeWithField) -> String {
        use lib_ruby_parser_nodes::NodeFieldType::*;

        match node_with_field.field.field_type {
            Node => "LIB_RUBY_PARSER_NodePtr",
            Nodes => "LIB_RUBY_PARSER_NodeList",
            MaybeNode { .. } => "LIB_RUBY_PARSER_MaybeNodePtr",
            Loc => "LIB_RUBY_PARSER_Loc",
            MaybeLoc => "LIB_RUBY_PARSER_MaybeLoc",
            Str { .. } => "LIB_RUBY_PARSER_StringPtr",
            MaybeStr { .. } => "LIB_RUBY_PARSER_MaybeStringPtr",
            StringValue => "LIB_RUBY_PARSER_Bytes",
            U8 => "LIB_RUBY_PARSER_Byte",
        }
        .to_string()
    }

    pub(crate) fn c_blob_type(node_with_field: &NodeWithField) -> String {
        use lib_ruby_parser_nodes::NodeFieldType::*;

        match node_with_field.field.field_type {
            Node => "LIB_RUBY_PARSER_Ptr_BLOB",
            Nodes => "LIB_RUBY_PARSER_NodeList_BLOB",
            MaybeNode { .. } => "LIB_RUBY_PARSER_MaybePtr_BLOB",
            Loc => "LIB_RUBY_PARSER_Loc_BLOB",
            MaybeLoc => "LIB_RUBY_PARSER_MaybeLoc_BLOB",
            Str { .. } => "LIB_RUBY_PARSER_StringPtr_BLOB",
            MaybeStr { .. } => "LIB_RUBY_PARSER_MaybeStringPtr_BLOB",
            StringValue => "LIB_RUBY_PARSER_Bytes_BLOB",
            U8 => "LIB_RUBY_PARSER_Byte_BLOB",
        }
        .to_string()
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

    pub(crate) fn name(message_with_field: &MessageWithField) -> String {
        message_with_field.field.snakecase_name.to_owned()
    }

    #[cfg(feature = "lib-ruby-parser-bindings")]
    pub(crate) fn c_name(message_with_field: &MessageWithField) -> String {
        lib_ruby_parser_bindings::helpers::messages::fields::field_name(&message_with_field.field)
            .to_string()
    }

    pub(crate) fn c_field_type(message_with_field: &MessageWithField) -> String {
        use lib_ruby_parser_nodes::MessageFieldType::*;
        match message_with_field.field.field_type {
            Str => "LIB_RUBY_PARSER_StringPtr",
            Byte => "LIB_RUBY_PARSER_Byte",
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
    fns.register_helper("node-c-enum-variant-name", nodes::c_enum_variant_name);
    fns.register_helper("node-c-union-member-name", nodes::c_union_member_name);

    fns.register_helper("node-field-name", node_fields::name);
    #[cfg(feature = "lib-ruby-parser-bindings")]
    fns.register_helper("node-field-c-name", node_fields::c_name);
    fns.register_helper("node-field-c-field-type", node_fields::c_field_type);
    fns.register_helper("node-field-c-blob-type", node_fields::c_blob_type);
    fns.register_helper("node-field-c-pack-fn-name", node_fields::c_pack_fn_name);
    fns.register_helper("node-field-c-unpack-fn-name", node_fields::c_unpack_fn_name);

    fns.register_helper("message-camelcase-name", messages::camelcase_name);
    fns.register_helper("message-upper-name", messages::upper_name);
    fns.register_helper("message-lower-name", messages::lower_name);

    fns.register_helper("message-field-name", message_fields::name);
    #[cfg(feature = "lib-ruby-parser-bindings")]
    fns.register_helper("message-field-c-name", message_fields::c_name);
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
