use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Or {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,

    pub expression_l: Range,
    pub operator_l: Range,
}

impl InnerNode for Or {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.lhs);
        result.push_node(&self.rhs);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "or"
    }
}
