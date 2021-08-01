use lib_ruby_parser_nodes::{Node, NodeField, NodeFieldType};

pub(crate) struct NodeFile<'a> {
    node: &'a Node,
}

impl<'a> NodeFile<'a> {
    pub(crate) fn new(node: &'a Node) -> Self {
        Self { node }
    }

    pub(crate) fn write(&self) {
        std::fs::write(
            &format!("src/nodes/types/{}.rs", self.node.filename),
            self.contents(),
        )
        .unwrap();
    }

    fn contents(&self) -> String {
        format!(
            "{imports}

{comment}
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct {struct_name} {{
{fields_declaration}
}}

impl InnerNode for {struct_name} {{
    fn expression(&self) -> &Loc {{
        &self.expression_l
    }}

    fn inspected_children(&self, indent: usize) -> Vec<String> {{
        let mut result = InspectVec::new(indent);
        {inspected_children}
        result.strings()
    }}

    fn str_type(&self) -> &'static str {{
        \"{str_type}\"
    }}

    fn print_with_locs(&self) {{
        println!(\"{{}}\", self.inspect(0));
{print_with_locs}
    }}
}}

",
            imports = self.imports().join("\n"),
            comment = self.node.render_comment("///", 0),
            struct_name = self.node.struct_name,
            fields_declaration = self.fields_declaration().join("\n"),
            inspected_children = self.inspected_children().join("\n        "),
            print_with_locs = self.print_with_locs().join("\n"),
            str_type = self.node.str_type,
        )
    }

    fn fields_declaration(&self) -> Vec<String> {
        self.node
            .fields
            .0
            .iter()
            .map(|f| FieldWrapper::new(f).declaration())
            .collect::<Vec<_>>()
    }

    fn inspected_children(&self) -> Vec<String> {
        self.node
            .fields
            .0
            .iter()
            .filter_map(|f| FieldWrapper::new(f).print_me_code())
            .collect()
    }

    fn print_with_locs(&self) -> Vec<String> {
        self.node
            .fields
            .0
            .iter()
            .filter_map(|f| FieldWrapper::new(f).print_with_locs_me())
            .collect()
    }

    fn imports(&self) -> Vec<&'static str> {
        let mut imports = vec![];
        imports.push("use crate::nodes::InnerNode;");
        imports.push("use crate::nodes::InspectVec;");
        imports.push("use crate::Loc;");
        if self.has_field_with_type(NodeFieldType::Node)
            || self.has_field_with_type(NodeFieldType::Nodes)
            || self.has_field_with_type(NodeFieldType::RegexOptions)
            || self.has_field_with_type(NodeFieldType::MaybeNode)
        {
            imports.push("use crate::Node;");
        }
        if self.has_field_with_type(NodeFieldType::StringValue) {
            imports.push("use crate::StringValue;");
        }

        if self.has_field_with_type(NodeFieldType::MaybeNode)
            || self.has_field_with_type(NodeFieldType::RegexOptions)
        {
            imports.push("");
            imports.push("#[cfg(feature = \"compile-with-external-structures\")]");
            imports.push("use crate::containers::ExternalMaybePtr;");
            imports.push("#[cfg(feature = \"compile-with-external-structures\")]");
            imports.push("type MaybePtr<T> = ExternalMaybePtr<T>;");
            imports.push("#[cfg(not(feature = \"compile-with-external-structures\"))]");
            imports.push("type MaybePtr<T> = Option<Box<T>>;");
            imports.push("");
        }

        if self.has_field_with_type(NodeFieldType::Node) {
            imports.push("");
            imports.push("#[cfg(feature = \"compile-with-external-structures\")]");
            imports.push("use crate::containers::ExternalPtr;");
            imports.push("#[cfg(feature = \"compile-with-external-structures\")]");
            imports.push("type Ptr<T> = ExternalPtr<T>;");
            imports.push("#[cfg(not(feature = \"compile-with-external-structures\"))]");
            imports.push("type Ptr<T> = Box<T>;");
            imports.push("");
        }

        if self.has_field_with_type(NodeFieldType::Nodes) {
            imports.push("");
            imports.push("#[cfg(feature = \"compile-with-external-structures\")]");
            imports.push("use crate::containers::ExternalList;");
            imports.push("#[cfg(feature = \"compile-with-external-structures\")]");
            imports.push("type List<T> = ExternalList<T>;");
            imports.push("#[cfg(not(feature = \"compile-with-external-structures\"))]");
            imports.push("type List<T> = Vec<T>;");
            imports.push("");
        }

        if self.has_field_with_type(NodeFieldType::MaybeLoc) {
            imports.push("");
            imports.push("#[cfg(feature = \"compile-with-external-structures\")]");
            imports.push("use crate::containers::ExternalMaybeLoc;");
            imports.push("#[cfg(feature = \"compile-with-external-structures\")]");
            imports.push("type MaybeLoc = ExternalMaybeLoc;");
            imports.push("#[cfg(not(feature = \"compile-with-external-structures\"))]");
            imports.push("type MaybeLoc = Option<Loc>;");
            imports.push("");
        }

        if self.has_field_with_type(NodeFieldType::Str)
            || self.has_field_with_type(NodeFieldType::RawString)
        {
            imports.push("");
            imports.push("#[cfg(feature = \"compile-with-external-structures\")]");
            imports.push("use crate::containers::ExternalStringPtr;");
            imports.push("#[cfg(feature = \"compile-with-external-structures\")]");
            imports.push("type StringPtr = ExternalStringPtr;");
            imports.push("#[cfg(not(feature = \"compile-with-external-structures\"))]");
            imports.push("type StringPtr = String;");
            imports.push("");
        }

        if self.has_field_with_type(NodeFieldType::MaybeStr)
            || self.has_field_with_type(NodeFieldType::Chars)
        {
            imports.push("");
            imports.push("#[cfg(feature = \"compile-with-external-structures\")]");
            imports.push("use crate::containers::ExternalMaybeStringPtr;");
            imports.push("#[cfg(feature = \"compile-with-external-structures\")]");
            imports.push("type MaybeStringPtr = ExternalMaybeStringPtr;");
            imports.push("#[cfg(not(feature = \"compile-with-external-structures\"))]");
            imports.push("type MaybeStringPtr = Option<String>;");
            imports.push("");
        }

        imports
    }

    fn has_field_with_type(&self, field_type: NodeFieldType) -> bool {
        self.node.fields.any_field_has_type(field_type)
    }
}

struct FieldWrapper<'a> {
    field: &'a NodeField,
}

impl<'a> FieldWrapper<'a> {
    pub(crate) fn new(field: &'a NodeField) -> Self {
        Self { field }
    }

    pub(crate) fn declaration(&self) -> String {
        format!(
            "{comment}
    pub {field_name}: {field_type},",
            comment = self.field.render_comment("///", 4),
            field_name = self.field.field_name,
            field_type = self.str_field_type()
        )
    }

    pub(crate) fn str_field_type(&self) -> &'static str {
        match self.field.field_type {
            NodeFieldType::Node => "Ptr<Node>",
            NodeFieldType::Nodes => "List<Node>",
            NodeFieldType::MaybeNode => "MaybePtr<Node>",
            NodeFieldType::Loc => "Loc",
            NodeFieldType::MaybeLoc => "MaybeLoc",
            NodeFieldType::Str => "StringPtr",
            NodeFieldType::MaybeStr => "MaybeStringPtr",
            NodeFieldType::Chars => "MaybeStringPtr",
            NodeFieldType::StringValue => "StringValue",
            NodeFieldType::U8 => "u8",
            NodeFieldType::Usize => "usize",
            NodeFieldType::RawString => "StringPtr",
            NodeFieldType::RegexOptions => "MaybePtr<Node>",
        }
    }

    fn print_me_code(&self) -> Option<String> {
        let method_name = match &self.field.field_type {
            NodeFieldType::Node => "push_node",
            NodeFieldType::Nodes => "push_nodes",
            NodeFieldType::MaybeNode => {
                if self.field.always_print {
                    "push_maybe_node_or_nil"
                } else {
                    "push_maybe_node"
                }
            }
            NodeFieldType::Loc => return None,
            NodeFieldType::MaybeLoc => return None,
            NodeFieldType::Str => "push_str",
            NodeFieldType::MaybeStr => "push_maybe_str",
            NodeFieldType::Chars => "push_chars",
            NodeFieldType::StringValue => "push_string_value",
            NodeFieldType::U8 => "push_u8",
            NodeFieldType::Usize => "push_usize",
            NodeFieldType::RawString => "push_raw_str",
            NodeFieldType::RegexOptions => "push_regex_options",
        };

        Some(format!(
            "result.{}(&self.{});",
            method_name, self.field.field_name
        ))
    }

    fn print_with_locs_me(&self) -> Option<String> {
        let offset = "        ";

        match &self.field.field_type {
            NodeFieldType::Node => Some(format!(
                "{offset}self.{field_name}.inner_ref().print_with_locs();",
                offset = offset,
                field_name = self.field.field_name
            )),
            NodeFieldType::Nodes => Some(format!(
                "{offset}for node in self.{field_name}.iter() {{
{offset}    node.inner_ref().print_with_locs();
{offset}}}",
                offset = offset,
                field_name = self.field.field_name
            )),
            NodeFieldType::MaybeNode | NodeFieldType::RegexOptions => Some(format!(
                "{offset}self.{field_name}.as_ref().map(|node| node.inner_ref().print_with_locs());",
                offset = offset,
                field_name = self.field.field_name
            )),
            NodeFieldType::Loc => Some(format!(
                "{offset}self.{field_name}.print(\"{printable_field_name}\");",
                offset = offset,
                field_name = self.field.field_name,
                printable_field_name = self
                    .field
                    .field_name
                    .strip_suffix("_l")
                    .expect("expected loc field to end with _l")
            )),
            NodeFieldType::MaybeLoc => Some(format!(
                "{offset}self.{field_name}.as_ref().map(|loc| loc.print(\"{printable_field_name}\"));",
                offset = offset,
                field_name = self.field.field_name,
                printable_field_name = self
                    .field
                    .field_name
                    .strip_suffix("_l")
                    .expect("expected loc field to end with _l"),
            )),
            NodeFieldType::Str => None,
            NodeFieldType::MaybeStr => None,
            NodeFieldType::Chars => None,
            NodeFieldType::StringValue => None,
            NodeFieldType::U8 => None,
            NodeFieldType::Usize => None,
            NodeFieldType::RawString => None,
        }
    }
}
