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
        Self { raw }
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

    /// Constructs an empty instance of `Bytes`
    pub fn empty() -> Self {
        Self::new(vec![])
    }

    /// Converts byte sequence to a string slice, returns error if there are invalid UTF-8 chars
    pub fn as_str_lossy(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.as_raw())
    }

    /// Converts byte sequence to a string, all invalid UTF-8 chars are converted into "replacement char"
    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(self.as_raw()).into_owned()
    }

    /// Converts byte sequence to a String, returns error if there are invalid UTF-8 chars
    pub fn to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.as_raw().to_vec())
    }

    /// Consumes itself and convrters it into a string, returns error if there are invalid UTF-8 chars
    pub fn into_string(self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.into_raw())
    }

    /// Returns `true` if `self` represents a valid UTF-8 string
    pub fn is_valid_utf8(&self) -> bool {
        std::str::from_utf8(self.as_raw()).is_ok()
    }

    /// Returns `true` if byte sequence is empty
    pub fn is_empty(&self) -> bool {
        self.as_raw().is_empty()
    }

    /// Returns length of the byte sequence
    pub fn len(&self) -> usize {
        self.as_raw().len()
    }

    /// Clears inner data
    pub fn clear(&mut self) {
        self.set_raw(vec![])
    }
}

impl std::ops::Index<usize> for Bytes {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        self.raw.index(index)
    }
}

#[test]
fn test_new() {
    let bytes = Bytes::new(vec![1, 2, 3]);
    drop(bytes);
}

#[test]
fn test_as_raw() {
    let bytes = Bytes::new(vec![1, 2, 3]);

    assert_eq!(bytes.as_raw(), &vec![1, 2, 3])
}

#[test]
fn test_into_raw() {
    let bytes = Bytes::new(vec![1, 2, 3]);

    assert_eq!(bytes.into_raw(), vec![1, 2, 3])
}

#[test]
fn test_set_raw() {
    let mut bytes = Bytes::new(vec![1, 2, 3]);
    bytes.set_raw(vec![4, 5, 6]);

    assert_eq!(bytes.as_raw(), &vec![4, 5, 6])
}

#[test]
fn test_push() {
    let mut bytes = Bytes::default();
    for i in 0..10 {
        bytes.push(i);
    }
    assert_eq!(bytes.as_raw(), &vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
}
