use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Def {
    pub name: String,
    pub args: Option<Box<Node>>,
    pub body: Option<Box<Node>>,
    pub keyword_l: Range,
    pub name_l: Range,
    pub end_l: Option<Range>,
    pub assignment_l: Option<Range>,
    pub expression_l: Range,
}

impl InnerNode for Def {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_str(&self.name);
        result.push_maybe_node_or_nil(&self.args);
        result.push_maybe_node_or_nil(&self.body);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "def"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        if let Some(range) = &self.assignment_l {
            range.print("assignment");
        }
        if let Some(range) = &self.end_l {
            range.print("end");
        }
        self.name_l.print("name");
        self.keyword_l.print("keyword");
        if let Some(node) = &self.body {
            node.inner().print_with_locs();
        }
        if let Some(node) = &self.args {
            node.inner().print_with_locs();
        }
    }
}
