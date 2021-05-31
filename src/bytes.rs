#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalList;
#[cfg(feature = "compile-with-external-structures")]
type List<T> = ExternalList<T>;
#[cfg(not(feature = "compile-with-external-structures"))]
type List<T> = Vec<T>;

/// Representation of a byte sequence
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Bytes {
    /// Raw vector of bytes
    pub(crate) raw: List<u8>,
}

impl Default for Bytes {
    fn default() -> Self {
        Self {
            raw: List::<u8>::new(),
        }
    }
}

impl Bytes {
    /// Constructs an instance of `Bytes` with a given byte vector
    pub fn new(raw: Vec<u8>) -> Self {
        Self { raw: raw.into() }
    }

    /// Constructs an empty instance of `Bytes`
    pub fn empty() -> Self {
        Self {
            raw: List::<u8>::new(),
        }
    }

    /// Returns a slice of the byte sequence
    pub fn as_bytes(&self) -> &[u8] {
        &self.raw
    }

    /// Returns a mutable slice of the byte sequence
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.raw
    }

    /// Consumes itself and converts it into a vector of bytes
    pub fn into_bytes(self) -> Vec<u8> {
        self.raw.into()
    }

    /// Converts byte sequence to a string slice, returns error if there are invalid UTF-8 chars
    pub fn as_str_lossy(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.raw)
    }

    /// Converts byte sequnce to a string, all invalid UTF-8 chars are converted into "replacement char"
    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.raw).into_owned()
    }

    /// Converts byte sequence to a String, returns error if there are invalid UTF-8 chars
    pub fn to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.as_bytes().to_vec())
    }

    /// Consumes itself and convrters it into a string, returns error if there are invalid UTF-8 chars
    pub fn into_string(self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.raw.into())
    }

    /// Returns `true` if `self` represents a valid UTF-8 string
    pub fn is_valid_utf8(&self) -> bool {
        std::str::from_utf8(&self.raw).is_ok()
    }

    /// Returns `true` if byte sequence is empty
    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }

    /// Returns length of the byte sequence
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    pub(crate) fn clear(&mut self) {
        self.raw = List::<u8>::new()
    }
}

impl std::ops::Index<usize> for Bytes {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        self.raw.index(index)
    }
}
