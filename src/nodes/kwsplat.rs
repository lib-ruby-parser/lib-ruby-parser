use crate::nodes::InnerNode;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Kwsplat {
    pub value: Option<Box<Node>>,

    pub operator_l: Range,
    pub expression_l: Range,
}

impl<'a> InnerNode<'a> for Kwsplat {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspect(&self, level: usize) -> String {
        todo!()
    }
}
