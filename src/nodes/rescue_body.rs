use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
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

impl InnerNode for RescueBody {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_maybe_node_or_nil(&self.exc_list);
        result.push_maybe_node_or_nil(&self.exc_var);
        result.push_maybe_node_or_nil(&self.body);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "resbody"
    }
}
