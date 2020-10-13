use crate::nodes::InnerNode;
use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Str {
    pub name: String,

    pub expression_l: Range,
    pub begin_l: Range,
    pub end_l: Range,
}
impl<'a> InnerNode<'a> for Str {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> String {
        todo!()
    }

    fn str_type(&self) -> &'static str {
        "str"
    }
}
