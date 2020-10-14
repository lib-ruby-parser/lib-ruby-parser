use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Casgn {
    pub scope: Option<Box<Node>>,
    pub name: String,
    pub value: Box<Node>,

    pub double_colon_l: Option<Range>,
    pub name_l: Range,
    pub expression_l: Range,
}

impl InnerNode for Casgn {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        if let Some(scope) = &self.scope {
            result.push_node(scope)
        } else {
            result.push_nil()
        }
        result.push_str(&self.name);
        result.push_node(&self.value);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "casgn"
    }
}
