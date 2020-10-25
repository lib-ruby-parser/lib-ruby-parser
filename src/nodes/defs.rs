use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Defs {
    pub definee: Box<Node>,
    pub name: String,
    pub args: Option<Box<Node>>,
    pub body: Option<Box<Node>>,
    pub keyword_l: Range,
    pub operator_l: Range,
    pub name_l: Range,
    pub assignment_l: Option<Range>,
    pub end_l: Option<Range>,
    pub expression_l: Range,
}

impl InnerNode for Defs {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.definee);
        result.push_str(&self.name);
        result.push_maybe_node_or_nil(&self.args);
        result.push_maybe_node_or_nil(&self.body);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "defs"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        if let Some(range) = &self.end_l {
            range.print("end");
        }
        if let Some(range) = &self.assignment_l {
            range.print("assignment");
        }
        self.name_l.print("name");
        self.operator_l.print("operator");
        self.keyword_l.print("keyword");
        if let Some(node) = &self.body {
            node.inner().print_with_locs();
        }
        if let Some(node) = &self.args {
            node.inner().print_with_locs();
        }
        self.definee.inner().print_with_locs();
    }
}
