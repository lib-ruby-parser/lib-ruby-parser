use crate::nodes::InnerNode;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Lvasgn {
    pub name: String,
    pub value: Box<Node>,

    pub expression_l: Range,
    pub name_l: Range,
    pub operator_l: Range,
}

impl<'a> InnerNode<'a> for Lvasgn {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspect(&self, level: usize) -> String {
        todo!()
    }
}
