use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct CSend {
    pub recv: Box<Node>,
    pub method_name: String,
    pub args: Vec<Node>,
    pub dot_l: Option<Range>,
    pub selector_l: Option<Range>,
    pub begin_l: Option<Range>,
    pub end_l: Option<Range>,
    pub operator_l: Option<Range>,
    pub expression_l: Range,
}

impl InnerNode for CSend {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.recv);
        result.push_str(&self.method_name);
        result.push_nodes(&self.args);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "csend"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        if let Some(range) = &self.operator_l {
            range.print("operator");
        }
        if let Some(range) = &self.end_l {
            range.print("end");
        }
        if let Some(range) = &self.begin_l {
            range.print("begin");
        }
        if let Some(range) = &self.selector_l {
            range.print("selector");
        }
        if let Some(range) = &self.dot_l {
            range.print("dot");
        }
        for node in self.args.iter() {
            node.inner().print_with_locs();
        }
        self.recv.inner().print_with_locs();
    }
}
