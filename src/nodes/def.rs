use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Def {
    pub name: String,
    pub args: Option<Box<Node>>,
    pub body: Option<Box<Node>>,

    pub keyword_l: Range,
    pub name_l: Range,
    pub end_l: Range,
    pub expression_l: Range,
}
