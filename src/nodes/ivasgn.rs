use crate::nodes::InnerNode;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Ivasgn {
    pub name: String,
    pub value: Box<Node>,

    pub expression_l: Range,
    pub name_l: Range,
    pub operator_l: Range,
}

impl<'a> InnerNode<'a> for Ivasgn {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> String {
        todo!()
    }

    fn str_type(&self) -> &'static str {
        "ivasgn"
    }
}
