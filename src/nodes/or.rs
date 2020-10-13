use crate::nodes::InnerNode;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Or {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,

    pub expression_l: Range,
    pub operator_l: Range,
}

impl<'a> InnerNode<'a> for Or {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> String {
        todo!()
    }

    fn str_type(&self) -> &'static str {
        "or"
    }
}
