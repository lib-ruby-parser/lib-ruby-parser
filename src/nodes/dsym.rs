use crate::nodes::InnerNode;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Dsym {
    pub parts: Vec<Node>,

    pub begin_l: Range,
    pub end_l: Range,
    pub expression_l: Range,
}

impl<'a> InnerNode<'a> for Dsym {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspect(&self, level: usize) -> String {
        todo!()
    }
}
