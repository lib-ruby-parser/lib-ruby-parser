pub(crate) mod messages {
    pub(crate) const NS: &str = "lib_ruby_parser__internal__containers__diagnostic_message";

    pub(crate) mod fields {
        pub(crate) fn field_name(field: &lib_ruby_parser_nodes::MessageField) -> &str {
            match &field.name[..] {
                "operator" => "operator_",
                other => other,
            }
        }

        pub(crate) fn field_type(field: &lib_ruby_parser_nodes::MessageField) -> &str {
            match field.field_type {
                lib_ruby_parser_nodes::MessageFieldType::Str => "StringPtr",
                lib_ruby_parser_nodes::MessageFieldType::Byte => "Byte",
            }
        }

        pub(crate) fn blob_type(field: &lib_ruby_parser_nodes::MessageField) -> String {
            format!("{}_BLOB", field_type(field))
        }
    }

    pub(crate) mod constructor {
        use super::{fields, NS};

        pub(crate) fn name(message: &lib_ruby_parser_nodes::Message) -> String {
            format!(
                "{ns}__make_{variant}",
                ns = NS,
                variant = message.lower_name()
            )
        }

        pub(crate) fn sig(message: &lib_ruby_parser_nodes::Message) -> String {
            let arglist = message
                .fields
                .map(&|field| {
                    format!(
                        "{blob_t} {name}",
                        blob_t = fields::blob_type(field),
                        name = fields::field_name(field)
                    )
                })
                .join(", ");

            format!(
                "DiagnosticMessage_BLOB {name}({arglist})",
                name = name(message),
                arglist = arglist
            )
        }
    }

    pub(crate) mod getter {
        use super::{fields, NS};

        pub(crate) fn name(
            message: &lib_ruby_parser_nodes::Message,
            field: &lib_ruby_parser_nodes::MessageField,
        ) -> String {
            format!(
                "{ns}__{variant}__get_{field_name}",
                ns = NS,
                variant = message.lower_name(),
                field_name = fields::field_name(field)
            )
        }

        pub(crate) fn sig(
            message: &lib_ruby_parser_nodes::Message,
            field: &lib_ruby_parser_nodes::MessageField,
        ) -> String {
            format!(
                "const {blob_type} *{getter_name}(const {variant}_BLOB *blob)",
                blob_type = fields::blob_type(field),
                variant = message.camelcase_name(),
                getter_name = name(message, field)
            )
        }
    }

    pub(crate) mod type_predicate {
        use super::NS;

        pub(crate) fn name(message: &lib_ruby_parser_nodes::Message) -> String {
            format!(
                "{ns}__is_{variant}",
                ns = NS,
                variant = message.lower_name()
            )
        }

        pub(crate) fn sig(message: &lib_ruby_parser_nodes::Message) -> String {
            format!(
                "bool {name}(const DiagnosticMessage_BLOB *blob)",
                name = name(message)
            )
        }
    }

    pub(crate) mod variant_getter {
        use super::NS;

        pub(crate) fn name(message: &lib_ruby_parser_nodes::Message) -> String {
            format!(
                "{ns}__as_{variant}",
                ns = NS,
                variant = message.lower_name()
            )
        }

        pub(crate) fn sig(message: &lib_ruby_parser_nodes::Message) -> String {
            format!(
                "const {variant}_BLOB *{name}(const DiagnosticMessage_BLOB *blob)",
                name = name(message),
                variant = message.camelcase_name()
            )
        }
    }
}

pub(crate) mod nodes {
    const NS: &str = "lib_ruby_parser__internal__containers__nodes";

    pub(crate) mod fields {
        pub(crate) fn field_name(field: &lib_ruby_parser_nodes::NodeField) -> String {
            match &field.field_name[..] {
                "default" => "default_",
                "operator" => "operator_",
                "else" => "else_",
                "const" => "const_",
                other => other,
            }
            .to_owned()
        }

        pub(crate) fn field_type(field: &lib_ruby_parser_nodes::NodeField) -> &str {
            use lib_ruby_parser_nodes::NodeFieldType;

            match field.field_type {
                NodeFieldType::Node => "NodePtr",
                NodeFieldType::Nodes => "NodeList",
                NodeFieldType::MaybeNode { .. } => "MaybeNodePtr",
                NodeFieldType::Loc => "Loc",
                NodeFieldType::MaybeLoc => "MaybeLoc",
                NodeFieldType::Str { .. } => "StringPtr",
                NodeFieldType::MaybeStr { .. } => "MaybeStringPtr",
                NodeFieldType::StringValue => "Bytes",
                NodeFieldType::U8 => "Byte",
            }
        }

        pub(crate) fn internal_field_type(field: &lib_ruby_parser_nodes::NodeField) -> &str {
            use lib_ruby_parser_nodes::NodeFieldType;

            match field.field_type {
                NodeFieldType::Node => "Ptr",
                NodeFieldType::Nodes => "NodeList",
                NodeFieldType::MaybeNode { .. } => "MaybePtr",
                NodeFieldType::Loc => "Loc",
                NodeFieldType::MaybeLoc => "MaybeLoc",
                NodeFieldType::Str { .. } => "StringPtr",
                NodeFieldType::MaybeStr { .. } => "MaybeStringPtr",
                NodeFieldType::StringValue => "Bytes",
                NodeFieldType::U8 => "Byte",
            }
        }

        pub(crate) fn pack_field_fn(field: &lib_ruby_parser_nodes::NodeField) -> String {
            format!("PACK_{}", internal_field_type(field))
        }

        pub(crate) fn unpack_field_fn(field: &lib_ruby_parser_nodes::NodeField) -> String {
            format!("UNPACK_{}", internal_field_type(field))
        }

        pub(crate) fn blob_type(field: &lib_ruby_parser_nodes::NodeField) -> String {
            format!("{}_BLOB", internal_field_type(field))
        }
    }

    pub(crate) fn enum_variant_name(node: &lib_ruby_parser_nodes::Node) -> String {
        format!("NODE_{}", node.upper_name())
    }

    pub(crate) fn union_member_name(node: &lib_ruby_parser_nodes::Node) -> String {
        let lower = node.lower_name();
        match &lower[..] {
            "and" | "break" | "case" | "class" | "const" | "false" | "float" | "for" | "if"
            | "int" | "or" | "return" | "true" | "while" => format!("{}_", lower),
            other => other.to_owned(),
        }
    }

    pub(crate) mod constructor {
        use super::{fields, NS};

        pub(crate) fn name(node: &lib_ruby_parser_nodes::Node) -> String {
            format!("{ns}__make_{lower}", ns = NS, lower = node.lower_name(),)
        }

        pub(crate) fn sig(node: &lib_ruby_parser_nodes::Node) -> String {
            let arglist = node
                .fields
                .map(&|field| {
                    format!(
                        "{field_type} {name}",
                        field_type = fields::blob_type(field),
                        name = fields::field_name(field)
                    )
                })
                .join(", ");

            format!(
                "Node_BLOB {name}({arglist})",
                name = name(node),
                arglist = arglist
            )
        }
    }

    pub(crate) mod getter {
        use super::{fields, NS};

        pub(crate) fn name(
            node: &lib_ruby_parser_nodes::Node,
            field: &lib_ruby_parser_nodes::NodeField,
        ) -> String {
            format!(
                "{ns}__{lower}__get_{field_name}",
                ns = NS,
                lower = node.lower_name(),
                field_name = field.field_name,
            )
        }
        pub(crate) fn sig(
            node: &lib_ruby_parser_nodes::Node,
            field: &lib_ruby_parser_nodes::NodeField,
        ) -> String {
            let return_type = format!("{} *", fields::blob_type(field));

            format!(
                "{return_type} {name}({struct_name}_BLOB *blob)",
                return_type = return_type,
                name = name(node, field),
                struct_name = node.camelcase_name
            )
        }
    }

    pub(crate) mod setter {
        use super::{fields, NS};

        pub(crate) fn name(
            node: &lib_ruby_parser_nodes::Node,
            field: &lib_ruby_parser_nodes::NodeField,
        ) -> String {
            format!(
                "{ns}__{lower}__set_{field_name}",
                ns = NS,
                lower = node.lower_name(),
                field_name = field.field_name,
            )
        }
        pub(crate) fn sig(
            node: &lib_ruby_parser_nodes::Node,
            field: &lib_ruby_parser_nodes::NodeField,
        ) -> String {
            format!(
                "void {setter_name}({struct_name}_BLOB* blob, {value_blob_type} {field_name})",
                setter_name = name(node, field),
                struct_name = node.camelcase_name,
                value_blob_type = fields::blob_type(field),
                field_name = fields::field_name(field)
            )
        }
    }

    pub(crate) mod variant_predicate {
        use super::NS;

        pub(crate) fn name(node: &lib_ruby_parser_nodes::Node) -> String {
            format!("{ns}__is_{lower}", ns = NS, lower = node.lower_name())
        }
        pub(crate) fn sig(node: &lib_ruby_parser_nodes::Node) -> String {
            format!("bool {}(const Node_BLOB* blob)", name(node))
        }
    }

    pub(crate) mod variant_getter {
        use super::NS;

        pub(crate) fn name(node: &lib_ruby_parser_nodes::Node) -> String {
            format!("{ns}__get_{lower}", ns = NS, lower = node.lower_name())
        }
        pub(crate) fn sig(node: &lib_ruby_parser_nodes::Node) -> String {
            format!(
                "{struct_name}_BLOB *{getter_name}(Node_BLOB *blob)",
                struct_name = node.camelcase_name,
                getter_name = name(node)
            )
        }
    }

    pub(crate) mod into_internal {
        use super::NS;

        pub(crate) fn name(node: &lib_ruby_parser_nodes::Node) -> String {
            format!(
                "{ns}__{lower}__into_internal",
                ns = NS,
                lower = node.lower_name()
            )
        }
        pub(crate) fn sig(node: &lib_ruby_parser_nodes::Node) -> String {
            format!(
                "Internal{struct_name} {name}({struct_name}_BLOB blob)",
                struct_name = node.camelcase_name,
                name = name(node)
            )
        }
    }

    pub(crate) mod into_variant {
        use super::NS;

        pub(crate) fn name(node: &lib_ruby_parser_nodes::Node) -> String {
            format!("{ns}__into_{lower}", ns = NS, lower = node.lower_name())
        }
        pub(crate) fn sig(node: &lib_ruby_parser_nodes::Node) -> String {
            format!(
                "{struct_name}_BLOB {name}(Node_BLOB blob)",
                struct_name = node.camelcase_name,
                name = name(node)
            )
        }
    }

    pub(crate) mod drop_variant {
        use super::NS;

        pub(crate) fn name(node: &lib_ruby_parser_nodes::Node) -> String {
            format!("{ns}__{lower}__drop", ns = NS, lower = node.lower_name())
        }
        pub(crate) fn sig(node: &lib_ruby_parser_nodes::Node) -> String {
            format!(
                "void {name}({struct_name}_BLOB *blob)",
                struct_name = node.camelcase_name,
                name = name(node)
            )
        }
    }
}
