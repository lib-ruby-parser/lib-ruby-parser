use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Int {
    pub value: String,
    pub expression_l: Range,
}

impl InnerNode for Int {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_str(&self.value);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "int"
    }
}
