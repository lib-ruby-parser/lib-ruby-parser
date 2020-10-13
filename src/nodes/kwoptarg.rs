use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Kwoptarg {
    pub name: String,
    pub default: Box<Node>,

    pub name_l: Range,
    pub operator_l: Range,
    pub expression_l: Range,
}
