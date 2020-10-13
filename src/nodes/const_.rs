use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Const {
    pub scope: Option<Box<Node>>,
    pub name: String,

    pub double_colon_l: Option<Range>,
    pub name_l: Range,
    pub expression_l: Range,
}
