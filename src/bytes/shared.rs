crate::use_native_or_external!(List);

use super::Bytes;

impl Bytes {
    /// Constructs an empty instance of `Bytes`
    pub fn empty() -> Self {
        Self::new(vec![])
    }

    /// Converts byte sequence to a string slice, returns error if there are invalid UTF-8 chars
    pub fn as_str_lossy(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.as_raw())
    }

    /// Converts byte sequnce to a string, all invalid UTF-8 chars are converted into "replacement char"
    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(self.as_raw()).into_owned()
    }

    /// Converts byte sequence to a String, returns error if there are invalid UTF-8 chars
    pub fn to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.as_raw().to_vec())
    }

    /// Consumes itself and convrters it into a string, returns error if there are invalid UTF-8 chars
    pub fn into_string(self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.into_raw().into())
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
        self.set_raw(list![])
    }
}
