extern crate rust_bison_skeleton;
extern crate serde;
extern crate serde_yaml;
use serde::Deserialize;
use std::path::Path;

fn generate_parse_y() {
    match rust_bison_skeleton::process_bison_file(&Path::new("src/parser.y")) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Failed to generate grammar.\n{:#?}", err);
            std::process::exit(1);
        }
    }
}

#[derive(PartialEq, Clone, Deserialize)]
enum FieldType {
    Node,
    Nodes,
    MaybeNode,
    Range,
    MaybeRange,
    Str,
    MaybeStr,
    Chars,
    StringValue,
    U8,
    Usize,
    RawString,
    RegexOptions,
}
impl FieldType {
    pub fn some_node_ref(&self) -> bool {
        match self {
            Node => true,
            Nodes => true,
            MaybeNode => true,
            Range => false,
            MaybeRange => false,
            Str => false,
            MaybeStr => false,
            Chars => false,
            StringValue => false,
            U8 => false,
            Usize => false,
            RawString => false,
            RegexOptions => true,
        }
    }
}
use FieldType::*;

#[derive(Clone, Deserialize)]
struct Field {
    field_name: String,
    field_type: FieldType,
    always_print: bool,
}

impl Field {
    pub fn declaration(&self) -> String {
        let field_type = match self.field_type {
            Node => "Box<Node>",
            Nodes => "Vec<Node>",
            MaybeNode => "Option<Box<Node>>",
            Range => "Range",
            MaybeRange => "Option<Range>",
            Str => "String",
            MaybeStr => "Option<String>",
            Chars => "Vec<char>",
            StringValue => "StringValue",
            U8 => "u8",
            Usize => "usize",
            RawString => "String",
            RegexOptions => "Option<Box<Node>>",
        };
        format!("    pub {}: {},", self.field_name, field_type)
    }

    pub fn print(&self) -> Option<String> {
        match self.field_type {
            Node => Some(format!(
                "        result.push_node(&self.{});",
                self.field_name
            )),
            Nodes => Some(format!(
                "        result.push_nodes(&self.{});",
                self.field_name
            )),
            MaybeNode => {
                if self.always_print {
                    Some(format!(
                        "        result.push_maybe_node_or_nil(&self.{});",
                        self.field_name
                    ))
                } else {
                    Some(format!(
                        "        result.push_maybe_node(&self.{});",
                        self.field_name
                    ))
                }
            }
            Range => None,
            MaybeRange => None,
            Str => Some(format!(
                "        result.push_str(&self.{});",
                self.field_name
            )),
            MaybeStr => Some(format!(
                "        result.push_maybe_str(&self.{});",
                self.field_name
            )),
            Chars => Some(format!(
                "        result.push_chars(&self.{});",
                self.field_name
            )),
            StringValue => Some(format!(
                "        result.push_string_value(&self.{});",
                self.field_name
            )),
            U8 => Some(format!(
                "        result.push_u8(&self.{});",
                self.field_name
            )),
            Usize => Some(format!(
                "        result.push_usize(&self.{});",
                self.field_name
            )),
            RawString => Some(format!(
                "        result.push_raw_str(&self.{});",
                self.field_name
            )),
            RegexOptions => Some(format!(
                "        result.push_regex_options(&self.{});",
                self.field_name
            )),
        }
    }

    pub fn print_with_locs(&self) -> Option<String> {
        let offset = "        ";

        match self.field_type {
            Node => Some(format!(
                "{offset}self.{field_name}.inner().print_with_locs();",
                offset = offset,
                field_name = self.field_name
            )),
            Nodes => Some(format!(
                "{offset}for node in self.{field_name}.iter() {{
{offset}    node.inner().print_with_locs();
{offset}}}",
                offset = offset,
                field_name = self.field_name
            )),
            MaybeNode => Some(format!(
                "{offset}if let Some(node) = &self.{field_name} {{
{offset}    node.inner().print_with_locs();
{offset}}}",
                offset = offset,
                field_name = self.field_name
            )),
            Range => Some(format!(
                "{offset}self.{field_name}.print(\"{printable_field_name}\");",
                offset = offset,
                field_name = self.field_name,
                printable_field_name = self
                    .field_name
                    .strip_suffix("_l")
                    .expect("expected loc field to end with _l")
            )),
            MaybeRange => Some(format!(
                "{offset}if let Some(range) = &self.{field_name} {{
{offset}    range.print(\"{printable_field_name}\");
{offset}}}",
                offset = offset,
                field_name = self.field_name,
                printable_field_name = self
                    .field_name
                    .strip_suffix("_l")
                    .expect("expected loc field to end with _l"),
            )),
            Str => None,
            MaybeStr => None,
            Chars => None,
            StringValue => None,
            U8 => None,
            Usize => None,
            RawString => None,
            RegexOptions => Some(format!(
                "{offset}if let Some(node) = &self.{field_name} {{
{offset}    node.inner().print_with_locs();
{offset}}}",
                offset = offset,
                field_name = self.field_name
            )),
        }
    }
}

#[derive(Deserialize)]
struct Struct {
    struct_name: String,
    str_type: String,
    filename: String,
    fields: Vec<Field>,
}

impl Struct {
    fn uses(&self) -> String {
        let mut uses = vec![];
        uses.push("use crate::nodes::InnerNode;");
        if self
            .fields
            .iter()
            .any(|f| f.field_type != Range && f.field_type != MaybeRange)
        {
            uses.push("use crate::nodes::InspectVec;");
        }
        uses.push("use crate::source::Range;");
        if self.fields.iter().any(|f| f.field_type.some_node_ref()) {
            uses.push("use crate::Node;");
        }
        if self
            .fields
            .iter()
            .any(|f| f.field_type == FieldType::StringValue)
        {
            uses.push("use crate::nodes::StringValue;");
        }
        uses.join("\n")
    }

    fn fields_declaration(&self) -> String {
        self.fields
            .iter()
            .map(|f| f.declaration())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn print_children(&self) -> String {
        let mut result = vec![];
        for field in self.fields.iter() {
            if let Some(code) = field.print() {
                result.push(code)
            }
        }
        if result.is_empty() {
            "
    fn inspected_children(&self, _indent: usize) -> Vec<String> {
        vec![]
    }"
            .to_owned()
        } else {
            format!(
                "
    fn inspected_children(&self, indent: usize) -> Vec<String> {{
        let mut result = InspectVec::new(indent);
{}
        result.strings()
    }}",
                result.join("\n")
            )
        }
    }

    pub fn print_with_locs(&self) -> String {
        let mut result = vec![];
        for field in self.fields.iter().rev() {
            if let Some(code) = field.print_with_locs() {
                result.push(code)
            }
        }
        result.join("\n")
    }

    pub fn code(&self) -> String {
        format!(
            "{uses}

#[derive(Debug, Clone, PartialEq)]
pub struct {struct_name} {{
{declare_fields}
}}

impl InnerNode for {struct_name} {{
    fn expression(&self) -> &Range {{
        &self.expression_l
    }}

{print_children}

    fn str_type(&self) -> &'static str {{
        \"{str_type}\"
    }}

    fn print_with_locs(&self) {{
        println!(\"{{}}\", self.inspect(0));
{print_with_locs}
    }}
}}
",
            uses = self.uses(),
            struct_name = self.struct_name,
            declare_fields = self.fields_declaration(),
            print_children = self.print_children(),
            str_type = self.str_type,
            print_with_locs = self.print_with_locs(),
        )
    }
}

fn generate_nodes() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=nodes.yaml");

    let f = std::fs::File::open("nodes.yaml")?;
    let nodes: Vec<Struct> = serde_yaml::from_reader(f)?;

    std::fs::create_dir_all("src/nodes/types")?;

    for node in nodes {
        std::fs::write(
            &format!("src/nodes/types/{}.rs", node.filename),
            node.code(),
        )
        .unwrap_or_else(|e| panic!("Failed to write into {}: {}", node.filename, e));
    }

    Ok(())
}

fn main() {
    generate_parse_y();
    generate_nodes().expect("Failed to generate nodes");
}
