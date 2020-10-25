use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Numblock {
    pub call: Box<Node>,
    pub numargs: u8,
    pub body: Box<Node>,
    pub begin_l: Range,
    pub end_l: Range,
    pub expression_l: Range,
}

impl InnerNode for Numblock {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.call);
        result.push_u8(&self.numargs);
        result.push_node(&self.body);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "numblock"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        self.end_l.print("end");
        self.begin_l.print("begin");
        self.body.inner().print_with_locs();
        self.call.inner().print_with_locs();
    }
}
