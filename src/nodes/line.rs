use crate::nodes::InnerNode;
use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub expression_l: Range,
}

impl<'a> InnerNode<'a> for Line {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspect(&self, level: usize) -> String {
        todo!()
    }
}
