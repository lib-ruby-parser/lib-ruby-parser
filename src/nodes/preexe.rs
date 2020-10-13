use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Preexe {
    pub body: Option<Box<Node>>,

    pub expression_l: Range,
    pub keyword_l: Range,
}
