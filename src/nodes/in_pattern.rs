use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct InPattern {
    pub pattern: Box<Node>,
    pub guard: Option<Box<Node>>,
    pub body: Option<Box<Node>>,

    pub keyword_l: Range,
    pub expression_l: Range,
}
