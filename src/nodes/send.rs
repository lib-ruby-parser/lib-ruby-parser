use crate::nodes::InnerNode;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Send {
    pub receiver: Option<Box<Node>>,
    pub method_name: String,
    pub args: Vec<Node>,

    pub dot_l: Range,
    pub selector_l: Range,
    pub expression_l: Range,
}

impl<'a> InnerNode<'a> for Send {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> String {
        todo!()
    }

    fn str_type(&self) -> &'static str {
        "send"
    }
}
