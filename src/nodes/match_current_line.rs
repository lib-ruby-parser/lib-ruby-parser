use crate::nodes::InnerNode;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCurrentLine {
    pub re: Box<Node>,

    pub expression_l: Range,
}

impl<'a> InnerNode<'a> for MatchCurrentLine {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> String {
        todo!()
    }

    fn str_type(&self) -> &'static str {
        "match_current_line"
    }
}
