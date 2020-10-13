use crate::nodes::InnerNode;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct When {
    pub patterns: Vec<Node>,
    pub body: Option<Box<Node>>,

    pub keyword_l: Range,
    pub expression_l: Range,
}

impl<'a> InnerNode<'a> for When {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> String {
        todo!()
    }

    fn str_type(&self) -> &'static str {
        "when"
    }
}
