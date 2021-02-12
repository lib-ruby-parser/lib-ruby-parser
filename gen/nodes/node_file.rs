use lib_ruby_parser_nodes::{Field, FieldType, Node};

pub struct NodeFile<'a> {
    node: &'a Node,
}

impl<'a> NodeFile<'a> {
    pub fn new(node: &'a Node) -> Self {
        Self { node }
    }

    pub fn write(&self) {
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
            comment = Comment::new(&self.node.comment).to_string(0),
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
            .iter()
            .map(|f| FieldWrapper::new(f).declaration())
            .collect::<Vec<_>>()
    }

    fn inspected_children(&self) -> Vec<String> {
        self.node
            .fields
            .iter()
            .filter_map(|f| FieldWrapper::new(f).print_me_code())
            .collect()
    }

    fn print_with_locs(&self) -> Vec<String> {
        self.node
            .fields
            .iter()
            .filter_map(|f| FieldWrapper::new(f).print_with_locs_me())
            .collect()
    }

    fn imports(&self) -> Vec<String> {
        let mut imports = vec![];
        imports.push("use crate::nodes::InnerNode;".to_owned());
        imports.push("use crate::nodes::InspectVec;".to_owned());
        imports.push("use crate::Loc;".to_owned());
        if self
            .node
            .fields
            .iter()
            .any(|f| f.field_type.has_reference_to_node())
        {
            imports.push("use crate::Node;".to_owned());
        }
        if self
            .node
            .fields
            .iter()
            .any(|f| f.field_type == FieldType::StringValue)
        {
            imports.push("use crate::StringValue;".to_owned());
        }
        imports
    }
}

struct FieldWrapper<'a> {
    field: &'a Field,
}

impl<'a> FieldWrapper<'a> {
    pub fn new(field: &'a Field) -> Self {
        Self { field }
    }

    pub fn declaration(&self) -> String {
        format!(
            "{comment}
    pub {field_name}: {field_type},",
            comment = Comment::new(&self.field.comment).to_string(4),
            field_name = self.field.field_name,
            field_type = self.str_field_type()
        )
    }

    pub fn str_field_type(&self) -> &'static str {
        match self.field.field_type {
            FieldType::Node => "Box<Node>",
            FieldType::Nodes => "Vec<Node>",
            FieldType::MaybeNode => "Option<Box<Node>>",
            FieldType::Loc => "Loc",
            FieldType::MaybeLoc => "Option<Loc>",
            FieldType::Str => "String",
            FieldType::MaybeStr => "Option<String>",
            FieldType::Chars => "Vec<char>",
            FieldType::StringValue => "StringValue",
            FieldType::U8 => "u8",
            FieldType::Usize => "usize",
            FieldType::RawString => "String",
            FieldType::RegexOptions => "Option<Box<Node>>",
        }
    }

    fn print_me_code(&self) -> Option<String> {
        let method_name = match &self.field.field_type {
            FieldType::Node => "push_node",
            FieldType::Nodes => "push_nodes",
            FieldType::MaybeNode => {
                if self.field.always_print {
                    "push_maybe_node_or_nil"
                } else {
                    "push_maybe_node"
                }
            }
            FieldType::Loc => return None,
            FieldType::MaybeLoc => return None,
            FieldType::Str => "push_str",
            FieldType::MaybeStr => "push_maybe_str",
            FieldType::Chars => "push_chars",
            FieldType::StringValue => "push_string_value",
            FieldType::U8 => "push_u8",
            FieldType::Usize => "push_usize",
            FieldType::RawString => "push_raw_str",
            FieldType::RegexOptions => "push_regex_options",
        };

        Some(format!(
            "result.{}(&self.{});",
            method_name, self.field.field_name
        ))
    }

    fn print_with_locs_me(&self) -> Option<String> {
        let offset = "        ";

        match &self.field.field_type {
            FieldType::Node => Some(format!(
                "{offset}self.{field_name}.inner_ref().print_with_locs();",
                offset = offset,
                field_name = self.field.field_name
            )),
            FieldType::Nodes => Some(format!(
                "{offset}for node in self.{field_name}.iter() {{
{offset}    node.inner_ref().print_with_locs();
{offset}}}",
                offset = offset,
                field_name = self.field.field_name
            )),
            FieldType::MaybeNode => Some(format!(
                "{offset}self.{field_name}.as_ref().map(|node| node.inner_ref().print_with_locs());",
                offset = offset,
                field_name = self.field.field_name
            )),
            FieldType::Loc => Some(format!(
                "{offset}self.{field_name}.print(\"{printable_field_name}\");",
                offset = offset,
                field_name = self.field.field_name,
                printable_field_name = self
                    .field
                    .field_name
                    .strip_suffix("_l")
                    .expect("expected loc field to end with _l")
            )),
            FieldType::MaybeLoc => Some(format!(
                "{offset}self.{field_name}.as_ref().map(|loc| loc.print(\"{printable_field_name}\"));",
                offset = offset,
                field_name = self.field.field_name,
                printable_field_name = self
                    .field
                    .field_name
                    .strip_suffix("_l")
                    .expect("expected loc field to end with _l"),
            )),
            FieldType::Str => None,
            FieldType::MaybeStr => None,
            FieldType::Chars => None,
            FieldType::StringValue => None,
            FieldType::U8 => None,
            FieldType::Usize => None,
            FieldType::RawString => None,
            FieldType::RegexOptions => Some(format!(
                "{offset}if let Some(node) = &self.{field_name} {{
{offset}    node.inner_ref().print_with_locs();
{offset}}}",
                offset = offset,
                field_name = self.field.field_name
            )),
        }
    }
}

struct Comment<'a> {
    comment: &'a Option<String>,
}

impl<'a> Comment<'a> {
    fn new(comment: &'a Option<String>) -> Self {
        Self { comment }
    }

    fn to_string(&self, offset: usize) -> String {
        self.comment
            .clone()
            .map(|c| {
                c.split("\n")
                    .map(|l| {
                        let mut l = l.to_owned();
                        if !l.is_empty() {
                            l = format!(" {}", l);
                        }
                        format!("{}///{}", " ".repeat(offset), l)
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .unwrap_or_default()
    }
}
