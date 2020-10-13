use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct IfMod {
    pub cond: Box<Node>,
    pub if_true: Option<Box<Node>>,
    pub if_false: Option<Box<Node>>,

    pub keyword_l: Range,
    pub expression_l: Range,
}
