use crate::nodes::InnerNode;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct OpAsgn {
    pub recv: Box<Node>,
    pub value: Box<Node>,

    pub expression_l: Range,
    pub operator_l: Range,
}

impl<'a> InnerNode<'a> for OpAsgn {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> String {
        todo!()
    }

    fn str_type(&self) -> &'static str {
        "op_asgn"
    }
}
