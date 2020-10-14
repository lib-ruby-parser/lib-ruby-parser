use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCurrentLine {
    pub re: Box<Node>,

    pub expression_l: Range,
}

impl InnerNode for MatchCurrentLine {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.re);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "match_current_line"
    }
}
