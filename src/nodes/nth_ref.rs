use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct NthRef {
    pub name: usize,

    pub expression_l: Range,
}

impl InnerNode for NthRef {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_usize(self.name);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "nth_ref"
    }
}
