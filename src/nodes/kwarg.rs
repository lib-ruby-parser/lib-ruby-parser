use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Kwarg {
    pub name: String,

    pub expression_l: Range,
}
