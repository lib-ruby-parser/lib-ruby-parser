use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Cvasgn {
    pub name: String,
    pub value: Option<Box<Node>>,
    pub name_l: Range,
    pub operator_l: Option<Range>,
    pub expression_l: Range,
}

impl InnerNode for Cvasgn {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_str(&self.name);
        result.push_maybe_node(&self.value);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "cvasgn"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        if let Some(range) = &self.operator_l {
            range.print("operator");
        }
        self.name_l.print("name");
        if let Some(node) = &self.value {
            node.inner().print_with_locs();
        }
    }
}
