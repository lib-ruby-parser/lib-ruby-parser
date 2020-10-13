use crate::nodes::InnerNode;
use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Blockarg {
    pub name: String,

    pub name_l: Range,
    pub expression_l: Range,
}

impl<'a> InnerNode<'a> for Blockarg {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> String {
        todo!()
    }

    fn str_type(&self) -> &'static str {
        "blockarg"
    }
}
