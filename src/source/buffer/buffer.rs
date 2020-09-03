use crate::source::buffer::{reencode_string, BufferError};

#[derive(Debug, PartialEq)]
pub struct Buffer {
    name: String,
    chars: Vec<char>,
    lines: Vec<String>,
    encoding: String
}

impl Buffer {
    fn new(name: &str, source: String, encoding: String) -> Buffer {
        let lines: Vec<String> = source.lines().map(|e| e.to_owned()).collect();
        let chars: Vec<char> = source.chars().collect();
        Buffer { name: name.into(), lines, chars, encoding }
    }

    pub fn new_from_source(name: &str, source: &str) -> Result<Buffer, BufferError> {
        let source: Vec<u8> = source.bytes().collect();
        let (source, encoding) = reencode_string(&source)?;
        Ok(Buffer::new(name, source, encoding))
    }

    pub fn new_from_file(name: &str, filepath: &str) -> Result<Buffer, BufferError> {
        let source = std::fs::read(filepath)?;
        let (source, encoding) = reencode_string(&source)?;

        Ok(Buffer::new(name, source, encoding))
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn encoding(&self) -> &str {
        &self.encoding
    }

    pub fn slice(&self, range: std::ops::Range<usize>) -> &[char] {
        if range.start < self.chars.len() && range.end < self.chars.len() {
            &self.chars.as_slice()[range]
        } else {
            &[]
        }
    }
}
