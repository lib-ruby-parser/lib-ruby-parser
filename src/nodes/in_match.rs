use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct InMatch {
    pub value: Box<Node>,
    pub pattern: Box<Node>,

    pub keyword_l: Range,
    pub expression_l: Range,
}
