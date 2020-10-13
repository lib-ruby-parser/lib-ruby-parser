use crate::nodes::InnerNode;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: Box<Node>,
    pub superclass: Option<Box<Node>>,
    pub body: Option<Box<Node>>,

    pub keyword_l: Range,
    pub operator_l: Option<Range>,
    pub end_l: Range,
    pub expression_l: Range,
}

impl<'a> InnerNode<'a> for Class {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> String {
        todo!()
    }

    fn str_type(&self) -> &'static str {
        "class"
    }
}
