use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub call: Box<Node>,
    pub args: Option<Box<Node>>,
    pub body: Option<Box<Node>>,
    pub begin_l: Range,
    pub end_l: Range,
    pub expression_l: Range,
}

impl InnerNode for Block {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.call);
        result.push_maybe_node_or_nil(&self.args);
        result.push_maybe_node_or_nil(&self.body);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "block"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        self.end_l.print("end");
        self.begin_l.print("begin");
        if let Some(node) = &self.body {
            node.inner().print_with_locs();
        }
        if let Some(node) = &self.args {
            node.inner().print_with_locs();
        }
        self.call.inner().print_with_locs();
    }
}
