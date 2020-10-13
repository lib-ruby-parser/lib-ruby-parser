use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Blockarg {
    pub name: String,

    pub name_l: Range,
    pub expression_l: Range,
}
