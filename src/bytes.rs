#[derive(Debug, Clone, PartialEq)]
pub struct Bytes {
    pub raw: Vec<u8>,
}

impl Default for Bytes {
    fn default() -> Self {
        Self { raw: vec![] }
    }
}

impl Bytes {
    pub fn new(raw: Vec<u8>) -> Self {
        Self { raw }
    }

    pub fn empty() -> Self {
        Self { raw: vec![] }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.raw
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.raw
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.raw
    }

    pub fn as_str_lossy(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.raw)
    }

    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.raw).into_owned()
    }

    pub fn to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.as_bytes().to_vec())
    }

    pub fn into_string(self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.raw)
    }

    pub fn is_valid_utf8(&self) -> bool {
        std::str::from_utf8(&self.raw).is_ok()
    }

    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }

    pub fn len(&self) -> usize {
        self.raw.len()
    }
}

impl std::ops::Index<usize> for Bytes {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        self.raw.index(index)
    }
}
