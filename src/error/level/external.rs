use crate::containers::size::ERROR_LEVEL_SIZE;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct ErrorLevelBlob {
    blob: [u8; ERROR_LEVEL_SIZE],
}

/// Byte sequence based on external implementation
#[repr(C)]
pub struct ErrorLevel {
    blob: ErrorLevelBlob,
}

extern "C" {
    fn lib_ruby_parser__internal__containers__error_level__make_warning() -> ErrorLevelBlob;
    fn lib_ruby_parser__internal__containers__error_level__make_error() -> ErrorLevelBlob;
    fn lib_ruby_parser__internal__containers__error_level__is_warning(blob: ErrorLevelBlob)
        -> bool;
    fn lib_ruby_parser__internal__containers__error_level__is_error(blob: ErrorLevelBlob) -> bool;
}

impl ErrorLevel {
    /// Constructs a warning
    pub fn warning() -> Self {
        let blob = unsafe { lib_ruby_parser__internal__containers__error_level__make_warning() };
        Self { blob }
    }

    /// Constructs an error
    pub fn error() -> Self {
        let blob = unsafe { lib_ruby_parser__internal__containers__error_level__make_error() };
        Self { blob }
    }

    /// Returns true if `self` is a warning
    pub fn is_warning(&self) -> bool {
        unsafe { lib_ruby_parser__internal__containers__error_level__is_warning(self.blob) }
    }

    /// Returns true if `self` is an error
    pub fn is_error(&self) -> bool {
        unsafe { lib_ruby_parser__internal__containers__error_level__is_error(self.blob) }
    }
}

impl Clone for ErrorLevel {
    fn clone(&self) -> Self {
        if self.is_warning() {
            Self::warning()
        } else if self.is_error() {
            Self::error()
        } else {
            unreachable!("only error/warning supported")
        }
    }
}

impl PartialEq for ErrorLevel {
    fn eq(&self, other: &Self) -> bool {
        if self.is_warning() && other.is_warning() {
            true
        } else if self.is_error() && other.is_error() {
            true
        } else {
            false
        }
    }
}

impl Eq for ErrorLevel {}
