use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Shadowarg {
    pub name: String,

    pub expression_l: Range,
}
