use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Defined {
    pub value: Box<Node>,

    pub keyword_l: Range,
    pub expression_l: Range,
}
