use crate::nodes::InnerNode;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Casgn {
    pub scope: Option<Box<Node>>,
    pub name: String,
    pub value: Box<Node>,

    pub double_colon_l: Option<Range>,
    pub name_l: Range,
    pub expression_l: Range,
}

impl<'a> InnerNode<'a> for Casgn {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> String {
        todo!()
    }

    fn str_type(&self) -> &'static str {
        "casgn"
    }
}
