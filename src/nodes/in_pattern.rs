use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct InPattern {
    pub pattern: Box<Node>,
    pub guard: Option<Box<Node>>,
    pub body: Option<Box<Node>>,
    pub keyword_l: Range,
    pub begin_l: Range,
    pub expression_l: Range,
}

impl InnerNode for InPattern {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.pattern);
        result.push_maybe_node_or_nil(&self.guard);
        result.push_maybe_node_or_nil(&self.body);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "in_pattern"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        self.begin_l.print("begin");
        self.keyword_l.print("keyword");
        if let Some(node) = &self.body {
            node.inner().print_with_locs();
        }
        if let Some(node) = &self.guard {
            node.inner().print_with_locs();
        }
        self.pattern.inner().print_with_locs();
    }
}
