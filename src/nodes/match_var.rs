use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchVar {
    pub name: String,

    pub expression_l: Range,
}
