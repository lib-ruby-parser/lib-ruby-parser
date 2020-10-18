use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct When {
    pub patterns: Vec<Node>,
    pub body: Option<Box<Node>>,

    pub keyword_l: Range,
    pub begin_l: Range,
    pub expression_l: Range,
}

impl InnerNode for When {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_nodes(&self.patterns);
        result.push_maybe_node_or_nil(&self.body);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "when"
    }
}
