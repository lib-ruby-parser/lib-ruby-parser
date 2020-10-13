use crate::nodes::InnerNode;
use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Restarg {
    pub name: Option<String>,

    pub name_l: Option<Range>,
    pub expression_l: Range,
}

impl<'a> InnerNode<'a> for Restarg {
    fn expression(&'a self) -> &'a Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> String {
        todo!()
    }

    fn str_type(&self) -> &'static str {
        "restarg"
    }
}
