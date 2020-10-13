use crate::nodes::InnerNode;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct If {
    pub cond: Box<Node>,
    pub if_true: Option<Box<Node>>,
    pub if_false: Option<Box<Node>>,

    pub if_l: Range,
    pub else_l: Option<Range>,
    pub end_l: Range,
    pub expression_l: Range,
}

impl<'a> InnerNode<'a> for If {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> String {
        todo!()
    }

    fn str_type(&self) -> &'static str {
        "if"
    }
}
