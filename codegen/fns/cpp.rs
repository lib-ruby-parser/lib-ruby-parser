use lib_ruby_parser_nodes::{template::TemplateFns, template::F, MessageField, NodeField};

pub(crate) mod node_fields {
    use super::*;

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
}

pub(crate) mod message_fields {
    use super::*;

    pub(crate) fn cpp_field_type(message_field: &MessageField) -> String {
        use lib_ruby_parser_nodes::MessageFieldType::*;
        match message_field.field_type {
            Str => "StringPtr",
            Byte => "Byte",
        }
        .to_string()
    }

    pub(crate) fn cpp_blob_type(message_field: &MessageField) -> String {
        format!("{}_BLOB", cpp_field_type(message_field))
    }
}

pub(crate) fn build() -> TemplateFns {
    let mut fns = TemplateFns::new();

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

    fns.register::<MessageField, F::Helper>(
        "message-field-cpp-field-type",
        message_fields::cpp_field_type,
    );
    fns.register::<MessageField, F::Helper>(
        "message-field-cpp-blob-type",
        message_fields::cpp_blob_type,
    );

    fns
}
