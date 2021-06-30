#[cfg(not(feature = "compile-with-external-structures"))]
mod error_level {
    /// Error level of the diagnostic message
    #[repr(C)]
    pub enum ErrorLevel {
        /// Warning level
        Warning,
        /// Error level
        Error,
    }

    impl ErrorLevel {
        /// Constructs a warning
        pub fn warning() -> Self {
            Self::Warning
        }

        /// Constructs an error
        pub fn error() -> Self {
            Self::Error
        }

        /// Returns true if `self` is a warning
        pub fn is_warning(&self) -> bool {
            matches!(self, Self::Warning)
        }

        /// Returns true if `self` is an error
        pub fn is_error(&self) -> bool {
            matches!(self, Self::Error)
        }
    }
}

#[cfg(feature = "compile-with-external-structures")]
mod error_level {
    use crate::containers::size::ERROR_LEVEL_SIZE;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct ErrorLevelBlob {
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
        fn lib_ruby_parser__internal__containers__error_level__is_warning(
            blob: ErrorLevelBlob,
        ) -> bool;
        fn lib_ruby_parser__internal__containers__error_level__is_error(
            blob: ErrorLevelBlob,
        ) -> bool;
    }

    impl ErrorLevel {
        /// Constructs a warning
        pub fn warning() -> Self {
            let blob =
                unsafe { lib_ruby_parser__internal__containers__error_level__make_warning() };
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

    #[cfg(test)]
    mod tests {
        use super::ErrorLevel;

        #[test]
        fn test_error() {
            let error = ErrorLevel::error();
            drop(error);
        }

        #[test]
        fn test_warning() {
            let warning = ErrorLevel::warning();
            drop(warning);
        }

        #[test]
        fn test_is_error() {
            assert!(ErrorLevel::error().is_error());
            assert!(!ErrorLevel::warning().is_error());
        }

        #[test]
        fn test_is_warning() {
            assert!(ErrorLevel::warning().is_warning());
            assert!(!ErrorLevel::error().is_warning());
        }
    }
}

pub use error_level::ErrorLevel;

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

impl ToString for ErrorLevel {
    fn to_string(&self) -> String {
        if self.is_warning() {
            "warning"
        } else if self.is_error() {
            "error"
        } else {
            unreachable!("only error/warning supported")
        }
        .to_string()
    }
}

impl std::fmt::Debug for ErrorLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}
