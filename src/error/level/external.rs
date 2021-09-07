use crate::blobs::Blob;

/// Byte sequence based on external implementation
#[repr(C)]
pub struct ErrorLevel {
    pub(crate) blob: Blob<ErrorLevel>,
}

extern "C" {
    fn lib_ruby_parser__external__error_level__new_warning() -> Blob<ErrorLevel>;
    fn lib_ruby_parser__external__error_level__new_error() -> Blob<ErrorLevel>;
    fn lib_ruby_parser__external__error_level__drop(blob: *mut Blob<ErrorLevel>);
    fn lib_ruby_parser__external__error_level__is_warning(blob: *const Blob<ErrorLevel>) -> bool;
    fn lib_ruby_parser__external__error_level__is_error(blob: *const Blob<ErrorLevel>) -> bool;
}

impl Drop for ErrorLevel {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__external__error_level__drop(&mut self.blob) }
    }
}

impl ErrorLevel {
    /// Constructs a warning
    pub fn warning() -> Self {
        let blob = unsafe { lib_ruby_parser__external__error_level__new_warning() };
        Self { blob }
    }

    /// Constructs an error
    pub fn error() -> Self {
        let blob = unsafe { lib_ruby_parser__external__error_level__new_error() };
        Self { blob }
    }

    /// Returns true if `self` is a warning
    pub fn is_warning(&self) -> bool {
        unsafe { lib_ruby_parser__external__error_level__is_warning(&self.blob) }
    }

    /// Returns true if `self` is an error
    pub fn is_error(&self) -> bool {
        unsafe { lib_ruby_parser__external__error_level__is_error(&self.blob) }
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
