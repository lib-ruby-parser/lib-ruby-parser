use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct IfMod {
    pub cond: Box<Node>,
    pub if_true: Option<Box<Node>>,
    pub if_false: Option<Box<Node>>,
    pub keyword_l: Range,
    pub expression_l: Range,
}

impl InnerNode for IfMod {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.cond);
        result.push_maybe_node_or_nil(&self.if_true);
        result.push_maybe_node_or_nil(&self.if_false);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "if"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        self.keyword_l.print("keyword");
        if let Some(node) = &self.if_false {
            node.inner().print_with_locs();
        }
        if let Some(node) = &self.if_true {
            node.inner().print_with_locs();
        }
        self.cond.inner().print_with_locs();
    }
}
