pub(crate) fn field_type(field: &lib_ruby_parser_nodes::NodeField) -> &str {
    use lib_ruby_parser_nodes::NodeFieldType;

    match field.field_type {
        NodeFieldType::Node => "Ptr<Node>",
        NodeFieldType::Nodes => "List<Node>",
        NodeFieldType::MaybeNode | NodeFieldType::RegexOptions => "MaybePtr<Node>",
        NodeFieldType::Loc => "Loc",
        NodeFieldType::MaybeLoc => "MaybeLoc",
        NodeFieldType::Str | NodeFieldType::RawString => "StringPtr",
        NodeFieldType::MaybeStr | NodeFieldType::Chars => "MaybeStringPtr",
        NodeFieldType::StringValue => "Bytes",
        NodeFieldType::U8 => "Byte",
        NodeFieldType::Usize => unreachable!(),
    }
}

pub(crate) fn blob_type(field: &lib_ruby_parser_nodes::NodeField) -> &str {
    use lib_ruby_parser_nodes::NodeFieldType;

    match field.field_type {
        NodeFieldType::Node => "PtrBlob",
        NodeFieldType::Nodes => "ListBlob",
        NodeFieldType::MaybeNode | NodeFieldType::RegexOptions => "MaybePtrBlob",
        NodeFieldType::Loc => "LocBlob",
        NodeFieldType::MaybeLoc => "MaybeLocBlob",
        NodeFieldType::Str | NodeFieldType::RawString => "StringPtrBlob",
        NodeFieldType::MaybeStr | NodeFieldType::Chars => "MaybeStringPtrBlob",
        NodeFieldType::StringValue => "BytesBlob",
        NodeFieldType::U8 => "ByteBlob",
        NodeFieldType::Usize => unreachable!(),
    }
}
