use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct OpAsgn {
    pub recv: Box<Node>,
    pub operator: String,
    pub value: Box<Node>,

    pub expression_l: Range,
    pub operator_l: Range,
}

impl InnerNode for OpAsgn {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.recv);
        result.push_str(&self.operator);
        result.push_node(&self.value);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "op_asgn"
    }
}
