use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Numblock {
    pub numargs: u8,
    pub call: Box<Node>,
    pub body: Box<Node>,

    pub begin_l: Range,
    pub end_l: Range,
    pub expression_l: Range,
}
