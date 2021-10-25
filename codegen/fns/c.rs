use lib_ruby_parser_nodes::{template::TemplateFns, template::F, MessageField, Node, NodeField};

pub(crate) mod nodes {
    use super::*;

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

pub(crate) mod node_fields {
    use super::*;

    #[cfg(feature = "lib-ruby-parser-bindings")]
    pub(crate) fn c_name(node_field: &NodeField) -> String {
        lib_ruby_parser_bindings::helpers::nodes::fields::field_name(&node_field)
    }

    pub(crate) fn c_pack_fn_name(node_field: &NodeField) -> String {
        use lib_ruby_parser_nodes::NodeFieldType::*;

        match node_field.field_type {
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

    pub(crate) fn c_unpack_fn_name(node_field: &NodeField) -> String {
        use lib_ruby_parser_nodes::NodeFieldType::*;

        match node_field.field_type {
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

    pub(crate) fn c_field_type(node_field: &NodeField) -> String {
        use lib_ruby_parser_nodes::NodeFieldType::*;
        match node_field.field_type {
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

    pub(crate) fn c_blob_type(node_field: &NodeField) -> String {
        use lib_ruby_parser_nodes::NodeFieldType::*;

        match node_field.field_type {
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

    pub(crate) fn drop_fn_name(node_field: &NodeField) -> String {
        use lib_ruby_parser_nodes::NodeFieldType::*;

        match node_field.field_type {
            Node => "LIB_RUBY_PARSER_drop_node_ptr",
            Nodes => "LIB_RUBY_PARSER_drop_node_list",
            MaybeNode { .. } => "LIB_RUBY_PARSER_drop_maybe_node_ptr",
            Loc => "LIB_RUBY_PARSER_drop_loc",
            MaybeLoc => "LIB_RUBY_PARSER_drop_maybe_loc",

            Str { .. } => "LIB_RUBY_PARSER_drop_string_ptr",

            MaybeStr { .. } => "LIB_RUBY_PARSER_drop_maybe_string_ptr",
            StringValue => "LIB_RUBY_PARSER_drop_bytes",
            U8 => "LIB_RUBY_PARSER_drop_byte",
        }
        .to_string()
    }
}

pub(crate) mod message_fields {
    use super::*;

    #[cfg(feature = "lib-ruby-parser-bindings")]
    pub(crate) fn c_name(message_field: &MessageField) -> String {
        lib_ruby_parser_bindings::helpers::messages::fields::field_name(&message_field).to_string()
    }

    pub(crate) fn c_field_type(message_field: &MessageField) -> String {
        use lib_ruby_parser_nodes::MessageFieldType::*;
        match message_field.field_type {
            Str => "LIB_RUBY_PARSER_StringPtr",
            Byte => "LIB_RUBY_PARSER_Byte",
        }
        .to_string()
    }

    pub(crate) fn c_blob_type(message_field: &MessageField) -> String {
        format!("{}_BLOB", c_field_type(message_field))
    }

    pub(crate) fn c_pack_fn_name(message_field: &MessageField) -> String {
        use lib_ruby_parser_nodes::MessageFieldType::*;
        match message_field.field_type {
            Str => "PACK_StringPtr",
            Byte => "PACK_Byte",
        }
        .to_string()
    }
    pub(crate) fn c_unpack_fn_name(message_field: &MessageField) -> String {
        use lib_ruby_parser_nodes::MessageFieldType::*;
        match message_field.field_type {
            Str => "UNPACK_StringPtr",
            Byte => "UNPACK_Byte",
        }
        .to_string()
    }

    pub(crate) fn drop_fn_name(message_field: &MessageField) -> String {
        use lib_ruby_parser_nodes::MessageFieldType::*;
        match message_field.field_type {
            Str => "LIB_RUBY_PARSER_drop_string_ptr",
            Byte => "LIB_RUBY_PARSER_drop_byte",
        }
        .to_string()
    }
}

pub(crate) fn build() -> TemplateFns {
    let mut fns = TemplateFns::new();

    fns.register::<Node, F::Helper>("node-c-enum-variant-name", nodes::c_enum_variant_name);
    fns.register::<Node, F::Helper>("node-c-union-member-name", nodes::c_union_member_name);

    #[cfg(feature = "lib-ruby-parser-bindings")]
    fns.register::<NodeField, F::Helper>("node-field-c-name", node_fields::c_name);
    fns.register::<NodeField, F::Helper>("node-field-c-field-type", node_fields::c_field_type);
    fns.register::<NodeField, F::Helper>("node-field-c-blob-type", node_fields::c_blob_type);
    fns.register::<NodeField, F::Helper>("node-field-c-pack-fn-name", node_fields::c_pack_fn_name);
    fns.register::<NodeField, F::Helper>(
        "node-field-c-unpack-fn-name",
        node_fields::c_unpack_fn_name,
    );
    fns.register::<NodeField, F::Helper>("node-field-drop-fn-name", node_fields::drop_fn_name);

    #[cfg(feature = "lib-ruby-parser-bindings")]
    fns.register::<MessageField, F::Helper>("message-field-c-name", message_fields::c_name);
    fns.register::<MessageField, F::Helper>(
        "message-field-c-field-type",
        message_fields::c_field_type,
    );
    fns.register::<MessageField, F::Helper>(
        "message-field-c-blob-type",
        message_fields::c_blob_type,
    );
    fns.register::<MessageField, F::Helper>(
        "message-field-c-pack-fn-name",
        message_fields::c_pack_fn_name,
    );
    fns.register::<MessageField, F::Helper>(
        "message-field-c-unpack-fn-name",
        message_fields::c_unpack_fn_name,
    );
    fns.register::<MessageField, F::Helper>(
        "message-field-drop-fn-name",
        message_fields::drop_fn_name,
    );

    fns
}
