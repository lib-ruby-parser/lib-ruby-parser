use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchAs {
    pub value: Box<Node>,
    pub as_: Box<Node>,

    pub operator_l: Range,
    pub expression_l: Range,
}

impl InnerNode for MatchAs {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.value);
        result.push_node(&self.as_);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "match_as"
    }
}
