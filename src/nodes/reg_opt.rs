use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct RegOpt {
    pub options: Vec<char>,

    pub expression_l: Range,
}

impl InnerNode for RegOpt {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        for option in &self.options {
            result.push_str(&format!("{}", option));
        }
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "regopt"
    }
}
