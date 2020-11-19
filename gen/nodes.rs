extern crate lib_ruby_parser_nodes;

use lib_ruby_parser_nodes::{Field, FieldType, Node};

fn map_field(field_type: &FieldType) -> String {
    match field_type {
        FieldType::Node => "Box<Node>",
        FieldType::Nodes => "Vec<Node>",
        FieldType::MaybeNode => "Option<Box<Node>>",
        FieldType::Range => "Range",
        FieldType::MaybeRange => "Option<Range>",
        FieldType::Str => "String",
        FieldType::MaybeStr => "Option<String>",
        FieldType::Chars => "Vec<char>",
        FieldType::StringValue => "StringValue",
        FieldType::U8 => "u8",
        FieldType::Usize => "usize",
        FieldType::RawString => "String",
        FieldType::RegexOptions => "Option<Box<Node>>",
    }
    .to_owned()
}

const FIELD_PREFIX: &str = "        ";

fn print_field_code(field: &Field) -> Option<String> {
    match &field.field_type {
        FieldType::Node => Some(format!(
            "{}result.push_node(&self.{});",
            FIELD_PREFIX, field.field_name
        )),
        FieldType::Nodes => Some(format!(
            "{}result.push_nodes(&self.{});",
            FIELD_PREFIX, field.field_name
        )),
        FieldType::MaybeNode => {
            if field.always_print {
                Some(format!(
                    "{}result.push_maybe_node_or_nil(&self.{});",
                    FIELD_PREFIX, field.field_name
                ))
            } else {
                Some(format!(
                    "{}result.push_maybe_node(&self.{});",
                    FIELD_PREFIX, field.field_name
                ))
            }
        }
        FieldType::Range => None,
        FieldType::MaybeRange => None,
        FieldType::Str => Some(format!(
            "{}result.push_str(&self.{});",
            FIELD_PREFIX, field.field_name
        )),
        FieldType::MaybeStr => Some(format!(
            "{}result.push_maybe_str(&self.{});",
            FIELD_PREFIX, field.field_name
        )),
        FieldType::Chars => Some(format!(
            "{}result.push_chars(&self.{});",
            FIELD_PREFIX, field.field_name
        )),
        FieldType::StringValue => Some(format!(
            "{}result.push_string_value(&self.{});",
            FIELD_PREFIX, field.field_name
        )),
        FieldType::U8 => Some(format!(
            "{}result.push_u8(&self.{});",
            FIELD_PREFIX, field.field_name
        )),
        FieldType::Usize => Some(format!(
            "{}result.push_usize(&self.{});",
            FIELD_PREFIX, field.field_name
        )),
        FieldType::RawString => Some(format!(
            "{}result.push_raw_str(&self.{});",
            FIELD_PREFIX, field.field_name
        )),
        FieldType::RegexOptions => Some(format!(
            "{}result.push_regex_options(&self.{});",
            FIELD_PREFIX, field.field_name
        )),
    }
}

fn print_field_with_locs(field: &Field) -> Option<String> {
    let offset = "        ";

    match &field.field_type {
        FieldType::Node => Some(format!(
            "{offset}self.{field_name}.inner().print_with_locs();",
            offset = offset,
            field_name = field.field_name
        )),
        FieldType::Nodes => Some(format!(
            "{offset}for node in self.{field_name}.iter() {{
{offset}    node.inner().print_with_locs();
{offset}}}",
            offset = offset,
            field_name = field.field_name
        )),
        FieldType::MaybeNode => Some(format!(
            "{offset}if let Some(node) = &self.{field_name} {{
{offset}    node.inner().print_with_locs();
{offset}}}",
            offset = offset,
            field_name = field.field_name
        )),
        FieldType::Range => Some(format!(
            "{offset}self.{field_name}.print(\"{printable_field_name}\");",
            offset = offset,
            field_name = field.field_name,
            printable_field_name = field
                .field_name
                .strip_suffix("_l")
                .expect("expected loc field to end with _l")
        )),
        FieldType::MaybeRange => Some(format!(
            "{offset}if let Some(range) = &self.{field_name} {{
{offset}    range.print(\"{printable_field_name}\");
{offset}}}",
            offset = offset,
            field_name = field.field_name,
            printable_field_name = field
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
{offset}    node.inner().print_with_locs();
{offset}}}",
            offset = offset,
            field_name = field.field_name
        )),
    }
}

fn uses(node: &Node) -> Vec<String> {
    let mut uses = vec![];
    uses.push("use crate::nodes::InnerNode;".to_owned());
    if node
        .fields
        .iter()
        .any(|f| !f.field_type.has_reference_to_range())
    {
        uses.push("use crate::nodes::InspectVec;".to_owned());
    }
    uses.push("use crate::source::Range;".to_owned());
    if node
        .fields
        .iter()
        .any(|f| f.field_type.has_reference_to_node())
    {
        uses.push("use crate::Node;".to_owned());
    }
    if node
        .fields
        .iter()
        .any(|f| f.field_type == FieldType::StringValue)
    {
        uses.push("use crate::StringValue;".to_owned());
    }
    uses
}

fn inspected_children_fn_declaration(node: &Node) -> String {
    let mut result = vec![];
    for field in node.fields.iter() {
        if let Some(code) = print_field_code(field) {
            result.push(code)
        }
    }
    if result.is_empty() {
        "fn inspected_children(&self, _indent: usize) -> Vec<String> {
        vec![]
    }"
        .to_owned()
    } else {
        format!(
            "fn inspected_children(&self, indent: usize) -> Vec<String> {{
        let mut result = InspectVec::new(indent);
{}
        result.strings()
    }}",
            result.join("\n")
        )
    }
}

fn print_with_locs_fn_declaration(node: &Node) -> String {
    let mut stmts = vec![];
    for field in node.fields.iter().rev() {
        if let Some(code) = print_field_with_locs(field) {
            stmts.push(code)
        }
    }
    let stmts = stmts.join("\n");

    format!(
        "fn print_with_locs(&self) {{
        println!(\"{{}}\", self.inspect(0));
{}
    }}",
        stmts
    )
}

fn prologue(_: &Node) -> String {
    "".to_owned()
}

fn epilogue(node: &Node) -> String {
    format!(
        "
impl InnerNode for {struct_name} {{
    fn expression(&self) -> &Range {{
        &self.expression_l
    }}

    {inspected_children}

    fn str_type(&self) -> &'static str {{
        \"{str_type}\"
    }}

    {print_with_locs}
}}
",
        struct_name = node.struct_name,
        inspected_children = inspected_children_fn_declaration(node),
        str_type = node.str_type,
        print_with_locs = print_with_locs_fn_declaration(node)
    )
}

pub fn generate_nodes() {
    let options = lib_ruby_parser_nodes::Options {
        target_dir: "src/nodes/types".to_owned(),
        map_field: Box::new(map_field),
        uses: Box::new(uses),
        prologue: Box::new(prologue),
        epilogue: Box::new(epilogue),
    };

    lib_ruby_parser_nodes::generate_nodes(&options).unwrap();
    lib_ruby_parser_nodes::generate_mod(&options).unwrap();
}
