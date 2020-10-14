use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchRest {
    pub name: Option<Box<Node>>,

    pub operator_l: Range,
    pub expression_l: Range,
}

impl InnerNode for MatchRest {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_maybe_node(&self.name);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "match_rest"
    }
}
