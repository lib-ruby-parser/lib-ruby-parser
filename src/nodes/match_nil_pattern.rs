use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchNilPattern {
    pub expression_l: Range,
}
