use lib_ruby_parser_nodes::{template::TemplateFns, template::F, MessageField, NodeField};

pub(crate) mod node_fields {
    use super::*;

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
            Node => "Box<Node>",
            Nodes => "Vec<Node>",
            MaybeNode { .. } => "Option<Box<Node>>",
            Loc => "Loc",
            MaybeLoc => "Option<Loc>",
            Str { .. } => "String",
            MaybeStr { .. } => "Option<String>",
            StringValue => "Bytes",
            U8 => "u8",
        }
        .to_string()
    }
}

pub(crate) mod message_fields {
    use super::*;

    pub(crate) fn rust_field_type(message_field: &MessageField) -> String {
        use lib_ruby_parser_nodes::MessageFieldType::*;
        match message_field.field_type {
            Str => "String",
            Byte => "u8",
        }
        .to_string()
    }

    pub(crate) fn rust_blob_type(message_field: &MessageField) -> String {
        use lib_ruby_parser_nodes::MessageFieldType::*;
        match message_field.field_type {
            Str => "Blob<String>",
            Byte => "u8",
        }
        .to_string()
    }
}

pub(crate) fn build() -> TemplateFns {
    let mut fns = TemplateFns::new();

    fns.register::<NodeField, F::Helper>(
        "node-field-rust-field-type",
        node_fields::rust_field_type,
    );
    fns.register::<NodeField, F::Helper>(
        "node-field-rust-field-name",
        node_fields::rust_field_name,
    );

    fns.register::<MessageField, F::Helper>(
        "message-field-rust-field-type",
        message_fields::rust_field_type,
    );
    fns.register::<MessageField, F::Helper>(
        "message-field-rust-blob-type",
        message_fields::rust_blob_type,
    );

    fns
}
