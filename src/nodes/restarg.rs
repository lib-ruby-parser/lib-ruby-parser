use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Restarg {
    pub name: Option<String>,
    pub operator_l: Range,
    pub name_l: Option<Range>,
    pub expression_l: Range,
}

impl InnerNode for Restarg {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_maybe_str(&self.name);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "restarg"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        if let Some(range) = &self.name_l {
            range.print("name");
        }
        self.operator_l.print("operator");
    }
}
