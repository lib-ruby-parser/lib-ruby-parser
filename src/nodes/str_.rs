use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Str {
    pub name: String,

    pub expression_l: Range,
    pub begin_l: Range,
    pub end_l: Range,
}
