use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: Box<Node>,
    pub superclass: Option<Box<Node>>,
    pub body: Option<Box<Node>>,
    pub keyword_l: Range,
    pub operator_l: Option<Range>,
    pub end_l: Range,
    pub expression_l: Range,
}

impl InnerNode for Class {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.name);
        result.push_maybe_node_or_nil(&self.superclass);
        result.push_maybe_node_or_nil(&self.body);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "class"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        self.end_l.print("end");
        if let Some(range) = &self.operator_l {
            range.print("operator");
        }
        self.keyword_l.print("keyword");
        if let Some(node) = &self.body {
            node.inner().print_with_locs();
        }
        if let Some(node) = &self.superclass {
            node.inner().print_with_locs();
        }
        self.name.inner().print_with_locs();
    }
}
