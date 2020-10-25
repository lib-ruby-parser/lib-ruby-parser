use crate::nodes::InnerNode;
use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Encoding {
    pub expression_l: Range,
}

impl InnerNode for Encoding {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, _indent: usize) -> Vec<String> {
        vec![]
    }

    fn str_type(&self) -> &'static str {
        "__ENCODING__"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
    }
}
