use crate::nodes::InnerNode;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct RescueBody {
    pub exc_list: Option<Box<Node>>,
    pub exc_var: Option<Box<Node>>,
    pub body: Option<Box<Node>>,

    pub expression_l: Range,
    pub keyword_l: Range,
    pub assoc_l: Option<Range>,
    pub begin_l: Option<Range>,
}

impl<'a> InnerNode<'a> for RescueBody {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> String {
        todo!()
    }

    fn str_type(&self) -> &'static str {
        "resbody"
    }
}
