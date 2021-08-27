use crate::source::SourceLine;

/// Decoded input
#[derive(Debug, Default)]
#[repr(C)]
pub struct DecodedInput {
    /// Name of the input
    pub name: String,

    /// Lines list
    pub lines: Vec<SourceLine>,

    /// Decoded bytes
    pub bytes: Vec<u8>,
}

impl DecodedInput {
    /// Constructs empty DecodedInput with given name
    pub fn named<Name>(name: Name) -> Self
    where
        Name: Into<String>,
    {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub(crate) fn name(&self) -> &String {
        &self.name
    }
    pub(crate) fn lines(&self) -> &Vec<SourceLine> {
        &self.lines
    }
    pub(crate) fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    #[allow(dead_code)]
    pub(crate) fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub(crate) fn set_lines(&mut self, lines: Vec<SourceLine>) {
        self.lines = lines;
    }
    pub(crate) fn set_bytes(&mut self, bytes: Vec<u8>) {
        self.bytes = bytes;
    }

    /// Converts itself into owned vector of bytes
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub(crate) fn take_bytes(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.bytes)
    }
}
