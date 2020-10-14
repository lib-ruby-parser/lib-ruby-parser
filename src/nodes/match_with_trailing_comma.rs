use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

// TODO: remove
#[derive(Debug, Clone, PartialEq)]
pub struct MatchWithTrailingComma {
    pub match_: Box<Node>,

    pub expression_l: Range,
}

impl InnerNode for MatchWithTrailingComma {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.match_);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "match_with_trailing_comma"
    }
}
