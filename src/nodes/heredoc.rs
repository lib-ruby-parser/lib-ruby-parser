use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Heredoc {
    pub parts: Vec<Node>,
    pub heredoc_body_l: Range,
    pub heredoc_end_l: Range,
    pub expression_l: Range,
}

impl InnerNode for Heredoc {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_nodes(&self.parts);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "dstr"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        self.heredoc_end_l.print("heredoc_end");
        self.heredoc_body_l.print("heredoc_body");
        for node in self.parts.iter() {
            node.inner().print_with_locs();
        }
    }
}
