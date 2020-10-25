use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Ensure {
    pub body: Option<Box<Node>>,
    pub ensure: Option<Box<Node>>,
    pub keyword_l: Range,
    pub expression_l: Range,
}

impl InnerNode for Ensure {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_maybe_node_or_nil(&self.body);
        result.push_maybe_node_or_nil(&self.ensure);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "ensure"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        self.keyword_l.print("keyword");
        if let Some(node) = &self.ensure {
            node.inner().print_with_locs();
        }
        if let Some(node) = &self.body {
            node.inner().print_with_locs();
        }
    }
}
