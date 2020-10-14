use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct IfTernary {
    pub cond: Box<Node>,
    pub if_true: Box<Node>,
    pub if_false: Box<Node>,

    pub question_l: Range,
    pub colon_l: Range,
    pub expression_l: Range,
}

impl InnerNode for IfTernary {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.cond);
        result.push_node(&self.if_true);
        result.push_node(&self.if_false);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "if"
    }
}
