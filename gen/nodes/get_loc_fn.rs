use lib_ruby_parser_nodes::{FieldType, Node};

include!("../../tests/loc_matcher/loc_name.rs");

impl LocName {
    fn to_str(&self) -> &'static str {
        match self {
            LocName::Begin => "begin_l",
            LocName::End => "end_l",
            LocName::Expression => "expression_l",
            LocName::Keyword => "keyword_l",
            LocName::Name => "name_l",
            LocName::Assignment => "assignment_l",
            LocName::Colon => "colon_l",
            LocName::DoubleColon => "double_colon_l",
            LocName::Else => "else_l",
            LocName::HeredocBody => "heredoc_body_l",
            LocName::Operator => "operator_l",
            LocName::Selector => "selector_l",
            LocName::Assoc => "assoc_l",
            LocName::Question => "question_l",
            LocName::HeredocEnd => "heredoc_end_l",
        }
    }
}

const LOC_NAMES: &[&'static LocName] = &[
    &LocName::Begin,
    &LocName::End,
    &LocName::Expression,
    &LocName::Keyword,
    &LocName::Name,
    &LocName::Assignment,
    &LocName::Colon,
    &LocName::DoubleColon,
    &LocName::Else,
    &LocName::HeredocBody,
    &LocName::Operator,
    &LocName::Selector,
    &LocName::Assoc,
    &LocName::Question,
    &LocName::HeredocEnd,
];

pub(crate) struct GetLocFn<'a> {
    nodes: &'a [Node],
}

impl<'a> GetLocFn<'a> {
    pub(crate) fn new(nodes: &'a [Node]) -> Self {
        Self { nodes }
    }

    pub(crate) fn write(&self) {
        std::fs::write("tests/loc_matcher/loc_name_gen.rs", self.contents()).unwrap();
    }

    fn contents(&self) -> String {
        format!(
            "use super::LocName;
use lib_ruby_parser::Node;

#[cfg(feature = \"compile-with-external-structures\")]
use lib_ruby_parser::containers::ExternalMaybeLoc;
#[cfg(feature = \"compile-with-external-structures\")]
type MaybeLoc = ExternalMaybeLoc;
#[cfg(not(feature = \"compile-with-external-structures\"))]
type MaybeLoc = Option<Loc>;

impl LocName {{
    pub(crate) fn get(&self, node: &Node) -> MaybeLoc {{
        match self {{
            {loc_branches}
        }}
    }}
}}
",
            loc_branches = self.loc_branches().join("\n            ")
        )
    }

    fn loc_branches(&self) -> Vec<String> {
        LOC_NAMES
            .iter()
            .map(|loc_name| {
                let fallback = if loc_name.to_str() == "expression_l" {
                    "".to_string()
                } else {
                    format!(
                        "other => panic!(\"node {{}} doesn't support {} loc\", other.str_type()),",
                        loc_name.to_str()
                    )
                };

                format!(
                    "LocName::{:?} => match node {{
                {}
                {}
            }},",
                    loc_name,
                    loc_name.match_nodes(self.nodes).join("\n                "),
                    fallback
                )
            })
            .collect()
    }
}

impl LocName {
    pub(crate) fn match_nodes(&self, nodes: &[Node]) -> Vec<String> {
        nodes
            .iter()
            .filter_map(|node| {
                let field = node.fields.iter().find(|f| f.field_name == self.to_str())?;
                match field.field_type {
                    FieldType::Loc => Some(format!(
                        "Node::{struct_name}(inner) => inner.{loc_name}.clone().into(),",
                        struct_name = node.struct_name,
                        loc_name = self.to_str()
                    )),
                    FieldType::MaybeLoc => Some(format!(
                        "Node::{struct_name}(inner) => inner.{loc_name}.clone(),",
                        struct_name = node.struct_name,
                        loc_name = self.to_str()
                    )),
                    _ => return None,
                }
            })
            .collect()
    }
}
