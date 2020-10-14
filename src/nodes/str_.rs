use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub enum StringValue {
    String(String),
    Bytes(Vec<u8>),
}

impl StringValue {
    pub fn to_string_lossy(&self) -> String {
        match &self {
            StringValue::String(s) => s.clone(),
            StringValue::Bytes(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
        }
    }

    pub fn to_string(&self) -> Option<String> {
        match &self {
            StringValue::String(s) => Some(s.clone()),
            StringValue::Bytes(bytes) => match String::from_utf8(bytes.clone()) {
                Ok(s) => Some(s),
                Err(_) => None,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Str {
    pub value: StringValue,

    pub expression_l: Range,
    pub begin_l: Range,
    pub end_l: Range,
}

impl InnerNode for Str {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        let value = match &self.value {
            StringValue::String(s) => s.to_owned(),
            StringValue::Bytes(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
        };
        result.push_str(&value);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "str"
    }
}
