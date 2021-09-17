/// Representation of a byte sequence
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Bytes {
    /// Raw vector of bytes
    pub raw: Vec<u8>,
}

impl Default for Bytes {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl Bytes {
    /// Constructs Bytes based on a given vector
    pub fn new(raw: Vec<u8>) -> Self {
        Self { raw: raw.into() }
    }

    /// Returns a reference to inner data
    pub fn as_raw(&self) -> &Vec<u8> {
        &self.raw
    }

    /// "Unwraps" self and returns inner data
    pub fn into_raw(self) -> Vec<u8> {
        self.raw
    }

    /// Replaces inner data with given Vec
    pub fn set_raw(&mut self, raw: Vec<u8>) {
        self.raw = raw
    }

    /// Appends a byte
    pub fn push(&mut self, item: u8) {
        self.raw.push(item);
    }
}

impl std::ops::Index<usize> for Bytes {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        self.raw.index(index)
    }
}
