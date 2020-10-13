use crate::nodes::InnerNode;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct UntilPost {
    pub cond: Box<Node>,
    pub body: Option<Box<Node>>,

    pub keyword_l: Range,
    pub expression_l: Range,
}

impl<'a> InnerNode<'a> for UntilPost {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspect(&self, level: usize) -> String {
        todo!()
    }
}
