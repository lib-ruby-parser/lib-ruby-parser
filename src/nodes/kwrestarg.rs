use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Kwrestarg {
    pub name: Option<String>,

    pub dstar_l: Range,
    pub name_l: Option<Range>,
    pub expression_l: Range,
}

impl InnerNode for Kwrestarg {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        if let Some(name) = &self.name {
            result.push_str(name);
        }
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "kwrestarg"
    }
}
