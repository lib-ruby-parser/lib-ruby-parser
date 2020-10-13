use crate::nodes::InnerNode;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Kwoptarg {
    pub name: String,
    pub default: Box<Node>,

    pub name_l: Range,
    pub operator_l: Range,
    pub expression_l: Range,
}

impl<'a> InnerNode<'a> for Kwoptarg {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> String {
        todo!()
    }

    fn str_type(&self) -> &'static str {
        "kwoptarg"
    }
}
