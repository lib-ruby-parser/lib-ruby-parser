use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct UntilPost {
    pub cond: Box<Node>,
    pub body: Option<Box<Node>>,

    pub keyword_l: Range,
    pub expression_l: Range,
}
