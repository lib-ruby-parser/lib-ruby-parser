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
