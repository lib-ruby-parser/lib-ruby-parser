use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct For {
    pub iterator: Box<Node>,
    pub iteratee: Box<Node>,
    pub body: Option<Box<Node>>,
    pub keyword_l: Range,
    pub operator_l: Range,
    pub begin_l: Range,
    pub end_l: Range,
    pub expression_l: Range,
}

impl InnerNode for For {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.iterator);
        result.push_node(&self.iteratee);
        result.push_maybe_node_or_nil(&self.body);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "for"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        self.end_l.print("end");
        self.begin_l.print("begin");
        self.operator_l.print("operator");
        self.keyword_l.print("keyword");
        if let Some(node) = &self.body {
            node.inner().print_with_locs();
        }
        self.iteratee.inner().print_with_locs();
        self.iterator.inner().print_with_locs();
    }
}
