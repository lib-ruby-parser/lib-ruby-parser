use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct StringValue {
    pub bytes: Vec<u8>,
}

impl StringValue {
    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.bytes).into_owned()
    }

    pub fn to_string(&self) -> Option<String> {
        String::from_utf8(self.bytes.clone()).ok()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

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
        result.push_str(&self.value.to_string_lossy());
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "str"
    }
}
