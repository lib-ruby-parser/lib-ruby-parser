use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Casgn {
    pub scope: Option<Box<Node>>,
    pub name: String,
    pub value: Box<Node>,

    pub double_colon_l: Option<Range>,
    pub name_l: Range,
    pub expression_l: Range,
}
