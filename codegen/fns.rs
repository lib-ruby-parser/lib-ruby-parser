use lib_ruby_parser_nodes::{
    template::TemplateFns, template::F, Message, MessageField, Node, NodeField,
};

pub(crate) mod nodes {
    use super::*;

    pub(crate) fn camelcase_name(node: &Node) -> String {
        node.camelcase_name.to_owned()
    }

    pub(crate) fn comment(node: &Node) -> String {
        node.render_comment("///", 0)
    }

    pub(crate) fn str_type(node: &Node) -> String {
        node.wqp_name.to_string()
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

    pub(crate) fn rust_camelcase_name(node: &Node) -> String {
        let camelcase_name = node.camelcase_name.to_owned();

        // match &camelcase_name[..] {
        //     "Self" => format!("{}_", camelcase_name),
        //     _ => camelcase_name,
        // }
        camelcase_name
    }

    pub(crate) fn is_last(node: &Node) -> bool {
        lib_ruby_parser_nodes::template::ALL_DATA
            .nodes
            .last()
            .unwrap()
            == &node
    }
}

pub(crate) mod node_fields {
    use super::*;

    pub(crate) fn name(node_field: &NodeField) -> String {
        node_field.snakecase_name.to_string()
    }

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

    pub(crate) fn cpp_pack_fn_name(node_field: &NodeField) -> String {
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
    pub(crate) fn cpp_unpack_fn_name(node_field: &NodeField) -> String {
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

    pub(crate) fn cpp_field_type(node_field: &NodeField) -> String {
        use lib_ruby_parser_nodes::NodeFieldType::*;
        match node_field.field_type {
            Node => "NodePtr",
            Nodes => "NodeList",
            MaybeNode { .. } => "MaybeNodePtr",
            Loc => "Loc",
            MaybeLoc => "MaybeLoc",
            Str { .. } => "StringPtr",
            MaybeStr { .. } => "MaybeStringPtr",
            StringValue => "Bytes",
            U8 => "Byte",
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

    pub(crate) fn cpp_blob_type(node_field: &NodeField) -> String {
        use lib_ruby_parser_nodes::NodeFieldType::*;
        match node_field.field_type {
            Node => "Ptr_BLOB",
            Nodes => "NodeList_BLOB",
            MaybeNode { .. } => "MaybePtr_BLOB",
            Loc => "Loc_BLOB",
            MaybeLoc => "MaybeLoc_BLOB",
            Str { .. } => "StringPtr_BLOB",
            MaybeStr { .. } => "MaybeStringPtr_BLOB",
            StringValue => "Bytes_BLOB",
            U8 => "Byte_BLOB",
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

    pub(crate) fn rust_field_name(node_field: &NodeField) -> String {
        let name = node_field.snakecase_name.to_owned();

        match &name[..] {
            "const" | "as" | "else" => format!("{}_", name),
            _ => name,
        }
    }

    pub(crate) fn rust_field_type(node_field: &NodeField) -> String {
        use lib_ruby_parser_nodes::NodeFieldType::*;

        match node_field.field_type {
            Node => "Ptr<Node>",
            Nodes => "List<Node>",
            MaybeNode { .. } => "Maybe<Ptr<Node>>",
            Loc => "Loc",
            MaybeLoc => "Maybe<Loc>",
            Str { .. } => "StringPtr",
            MaybeStr { .. } => "Maybe<StringPtr>",
            StringValue => "Bytes",
            U8 => "u8",
        }
        .to_string()
    }

    pub(crate) fn comment(node_field: &NodeField) -> String {
        node_field.render_comment("///", 4)
    }

    pub(crate) fn is_last(node_field: &NodeField) -> bool {
        node_field.node.fields.last().unwrap() == &node_field
    }
}

pub(crate) mod messages {
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

    pub(crate) fn has_no_fields(message: &Message) -> bool {
        message.fields.is_empty()
    }

    pub(crate) fn comment(message: &Message) -> String {
        message.render_comment("///", 0)
    }

    pub(crate) fn is_last(message: &Message) -> bool {
        lib_ruby_parser_nodes::template::ALL_DATA
            .messages
            .last()
            .unwrap()
            == &message
    }
}

pub(crate) mod message_fields {
    use super::*;

    pub(crate) fn name(message_field: &MessageField) -> String {
        message_field.snakecase_name.to_owned()
    }

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

    pub(crate) fn cpp_field_type(message_field: &MessageField) -> String {
        use lib_ruby_parser_nodes::MessageFieldType::*;
        match message_field.field_type {
            Str => "StringPtr",
            Byte => "Byte",
        }
        .to_string()
    }

    pub(crate) fn c_blob_type(message_field: &MessageField) -> String {
        format!("{}_BLOB", c_field_type(message_field))
    }

    pub(crate) fn cpp_blob_type(message_field: &MessageField) -> String {
        format!("{}_BLOB", cpp_field_type(message_field))
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

    pub(crate) fn rust_field_type(message_field: &MessageField) -> String {
        use lib_ruby_parser_nodes::MessageFieldType::*;
        match message_field.field_type {
            Str => "StringPtr",
            Byte => "u8",
        }
        .to_string()
    }

    pub(crate) fn rust_blob_type(message_field: &MessageField) -> String {
        use lib_ruby_parser_nodes::MessageFieldType::*;
        match message_field.field_type {
            Str => "Blob<StringPtr>",
            Byte => "u8",
        }
        .to_string()
    }

    pub(crate) fn comment(message_field: &MessageField) -> String {
        message_field.render_comment("///", 4)
    }

    pub(crate) fn is_last(message_field: &MessageField) -> bool {
        message_field.message.fields.last().unwrap() == &message_field
    }

    pub(crate) fn is_byte(message_field: &MessageField) -> bool {
        message_field.field_type == lib_ruby_parser_nodes::MessageFieldType::Byte
    }
}

pub(crate) fn build() -> TemplateFns {
    let mut fns = TemplateFns::new();

    fns.register::<Node, F::Helper>("node-camelcase-name", nodes::camelcase_name);
    fns.register::<Node, F::Helper>("node-comment", nodes::comment);
    fns.register::<Node, F::Helper>("node-str-type", nodes::str_type);
    fns.register::<Node, F::Helper>("node-upper-name", nodes::upper_name);
    fns.register::<Node, F::Helper>("node-lower-name", nodes::lower_name);
    fns.register::<Node, F::Helper>("node-c-enum-variant-name", nodes::c_enum_variant_name);
    fns.register::<Node, F::Helper>("node-c-union-member-name", nodes::c_union_member_name);
    fns.register::<Node, F::Predicate>("node-is-last", nodes::is_last);

    fns.register::<NodeField, F::Helper>("node-field-name", node_fields::name);
    fns.register::<NodeField, F::Helper>("node-field-comment", node_fields::comment);
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
    fns.register::<NodeField, F::Helper>("node-field-cpp-field-type", node_fields::cpp_field_type);
    fns.register::<NodeField, F::Helper>("node-field-cpp-blob-type", node_fields::cpp_blob_type);
    fns.register::<NodeField, F::Helper>(
        "node-field-cpp-pack-fn-name",
        node_fields::cpp_pack_fn_name,
    );
    fns.register::<NodeField, F::Helper>(
        "node-field-cpp-unpack-fn-name",
        node_fields::cpp_unpack_fn_name,
    );
    fns.register::<NodeField, F::Helper>(
        "node-field-rust-field-type",
        node_fields::rust_field_type,
    );
    fns.register::<NodeField, F::Helper>(
        "node-field-rust-field-name",
        node_fields::rust_field_name,
    );
    fns.register::<NodeField, F::Predicate>("node-field-is-last", node_fields::is_last);

    fns.register::<Message, F::Helper>("message-camelcase-name", messages::camelcase_name);
    fns.register::<Message, F::Helper>("message-upper-name", messages::upper_name);
    fns.register::<Message, F::Helper>("message-lower-name", messages::lower_name);
    fns.register::<Message, F::Helper>("message-comment", messages::comment);
    fns.register::<Message, F::Predicate>("message-is-last", messages::is_last);
    fns.register::<Message, F::Predicate>("message-has-no-fields", messages::has_no_fields);

    fns.register::<MessageField, F::Helper>("message-field-name", message_fields::name);
    fns.register::<MessageField, F::Helper>("mesage-field-comment", message_fields::comment);
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
        "message-field-cpp-field-type",
        message_fields::cpp_field_type,
    );
    fns.register::<MessageField, F::Helper>(
        "message-field-cpp-blob-type",
        message_fields::cpp_blob_type,
    );
    fns.register::<MessageField, F::Helper>(
        "message-field-drop-fn-name",
        message_fields::drop_fn_name,
    );
    fns.register::<MessageField, F::Helper>(
        "message-field-rust-field-type",
        message_fields::rust_field_type,
    );
    fns.register::<MessageField, F::Helper>(
        "message-field-rust-blob-type",
        message_fields::rust_blob_type,
    );
    fns.register::<MessageField, F::Predicate>("message-field-is-last", message_fields::is_last);
    fns.register::<MessageField, F::Predicate>("message-field-is-byte", message_fields::is_byte);

    fns
}

macro_rules! default_fns {
    () => {{
        fn generated_by(_: &lib_ruby_parser_nodes::template::GlobalContext) -> String {
            file!().to_string()
        }
        fn generated_by_for_node(_: &lib_ruby_parser_nodes::Node) -> String {
            file!().to_string()
        }
        let mut fns = $crate::codegen::fns::build();
        fns.register::<GlobalContext, F::Helper>("generated-by", generated_by);
        fns.register::<lib_ruby_parser_nodes::Node, F::Helper>(
            "generated-by",
            generated_by_for_node,
        );
        fns
    }};
}
pub(crate) use default_fns;
