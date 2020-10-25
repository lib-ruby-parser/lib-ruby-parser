use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    pub args: Vec<Node>,
    pub expression_l: Range,
    pub begin_l: Option<Range>,
    pub end_l: Option<Range>,
}

impl InnerNode for Args {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_nodes(&self.args);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "args"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        if let Some(range) = &self.end_l {
            range.print("end");
        }
        if let Some(range) = &self.begin_l {
            range.print("begin");
        }
        self.expression_l.print("expression");
        for node in self.args.iter() {
            node.inner().print_with_locs();
        }
    }
}
