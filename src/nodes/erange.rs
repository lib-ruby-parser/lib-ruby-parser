use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Erange {
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub operator_l: Range,
    pub expression_l: Range,
}

impl InnerNode for Erange {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_maybe_node_or_nil(&self.left);
        result.push_maybe_node_or_nil(&self.right);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "erange"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        self.operator_l.print("operator");
        if let Some(node) = &self.right {
            node.inner().print_with_locs();
        }
        if let Some(node) = &self.left {
            node.inner().print_with_locs();
        }
    }
}
