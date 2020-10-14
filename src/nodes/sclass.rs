use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct SClass {
    pub expr: Box<Node>,
    pub body: Option<Box<Node>>,

    pub keyword_l: Range,
    pub operator_l: Range,
    pub end_l: Range,
    pub expression_l: Range,
}

impl InnerNode for SClass {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.expr);
        result.push_maybe_node_or_nil(&self.body);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "sclass"
    }
}
