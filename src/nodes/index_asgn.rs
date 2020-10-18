use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct IndexAsgn {
    pub recv: Box<Node>,
    pub indexes: Vec<Node>,
    pub value: Option<Box<Node>>,

    pub begin_l: Range,
    pub end_l: Range,
    pub operator_l: Option<Range>,
    pub expression_l: Range,
}

impl InnerNode for IndexAsgn {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.recv);
        result.push_nodes(&self.indexes);
        if let Some(value) = &self.value {
            result.push_node(&value);
        }
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "indexasgn"
    }
}
