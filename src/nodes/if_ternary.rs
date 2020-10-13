use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct IfTernary {
    pub cond: Box<Node>,
    pub if_true: Box<Node>,
    pub if_false: Box<Node>,

    pub question_l: Range,
    pub colon_l: Range,
    pub expression_l: Range,
}
