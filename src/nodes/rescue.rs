use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Rescue {
    pub body: Option<Box<Node>>,
    pub rescue_bodies: Vec<Node>,
    pub else_: Option<Box<Node>>,

    pub expression_l: Range,
    pub else_l: Range,
}

impl InnerNode for Rescue {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_maybe_node_or_nil(&self.body);
        result.push_nodes(&self.rescue_bodies);
        result.push_maybe_node_or_nil(&self.else_);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "rescue"
    }
}
