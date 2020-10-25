use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::nodes::StringValue;

#[derive(Debug, Clone, PartialEq)]
pub struct Str {
    pub value: StringValue,
    pub expression_l: Range,
    pub begin_l: Option<Range>,
    pub end_l: Option<Range>,
}

impl InnerNode for Str {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_string_value(&self.value);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "str"
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
    }
}
