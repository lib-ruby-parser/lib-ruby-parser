use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct IfGuard {
    pub cond: Box<Node>,

    pub keyword_l: Range,
    pub expression_l: Range,
}

impl InnerNode for IfGuard {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.cond);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "if_guard"
    }
}
