use crate::nodes::InnerNode;
use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchVar {
    pub name: String,

    pub expression_l: Range,
}

impl<'a> InnerNode<'a> for MatchVar {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspect(&self, level: usize) -> String {
        todo!()
    }
}
