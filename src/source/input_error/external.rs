use crate::blobs::{Blob, HasBlob};
use crate::containers::ExternalStringPtr as StringPtr;

/// An enum with all possible kinds of errors that can be returned
/// from a decoder
#[repr(C)]
pub struct InputError {
    pub(crate) blob: Blob<InputError>,
}

extern "C" {
    fn lib_ruby_parser__external__input_error__new_unsupported_encoding(
        err: Blob<StringPtr>,
    ) -> Blob<InputError>;
    fn lib_ruby_parser__external__input_error__new_decoding_error(
        err: Blob<StringPtr>,
    ) -> Blob<InputError>;
    fn lib_ruby_parser__external__input_error__drop(blob: *mut Blob<InputError>);
    fn lib_ruby_parser__external__input_error__is_unsupported_encoding(
        blob: *const Blob<InputError>,
    ) -> bool;
    fn lib_ruby_parser__external__input_error__is_decoding_error(
        blob: *const Blob<InputError>,
    ) -> bool;
    fn lib_ruby_parser__external__input_error__get_unsupported_encoding(
        blob: *const Blob<InputError>,
    ) -> *const Blob<StringPtr>;
    fn lib_ruby_parser__external__input_error__get_decoding_error(
        blob: *const Blob<InputError>,
    ) -> *const Blob<StringPtr>;
}

impl Drop for InputError {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__external__input_error__drop(&mut self.blob) }
    }
}

impl std::fmt::Debug for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_unsupported_encoding() {
            write!(
                f,
                "UnsupportedEncoding({:?})",
                self.get_unsupported_encoding_message()
            )
        } else if self.is_decoding_error() {
            write!(f, "DecodingError({:?})", self.get_decoding_error_message())
        } else {
            panic!("Unsupported InputError variant")
        }
    }
}

impl InputError {
    /// Constructs UnupportedEncoding variant
    pub fn new_unsupported_encoding(err: StringPtr) -> Self {
        let blob = unsafe {
            lib_ruby_parser__external__input_error__new_unsupported_encoding(err.into_blob())
        };
        Self { blob }
    }

    /// Constructs DecodingError variant
    pub fn new_decoding_error(err: StringPtr) -> Self {
        let blob =
            unsafe { lib_ruby_parser__external__input_error__new_decoding_error(err.into_blob()) };
        Self { blob }
    }

    /// Returns `true` if `self` is a `UnupportedEncoding` variant
    pub(crate) fn is_unsupported_encoding(&self) -> bool {
        unsafe { lib_ruby_parser__external__input_error__is_unsupported_encoding(&self.blob) }
    }

    /// Returns `true` if `self` is a `DecodingError` variant
    pub(crate) fn is_decoding_error(&self) -> bool {
        unsafe { lib_ruby_parser__external__input_error__is_decoding_error(&self.blob) }
    }

    /// Returns StringPtr of the `UnupportedEncoding` variant. Panics if variant doesn't match
    pub(crate) fn get_unsupported_encoding_message(&self) -> &StringPtr {
        unsafe {
            (lib_ruby_parser__external__input_error__get_unsupported_encoding(&self.blob)
                as *const StringPtr)
                .as_ref()
                .unwrap()
        }
    }

    /// Returns StringPtr of the `DecodingError` variant. Panics if variant doesn't match
    pub(crate) fn get_decoding_error_message(&self) -> &StringPtr {
        unsafe {
            (lib_ruby_parser__external__input_error__get_decoding_error(&self.blob)
                as *const StringPtr)
                .as_ref()
                .unwrap()
        }
    }
}

impl PartialEq for InputError {
    fn eq(&self, other: &Self) -> bool {
        if self.is_unsupported_encoding() {
            if other.is_unsupported_encoding() {
                self.get_unsupported_encoding_message() == other.get_unsupported_encoding_message()
            } else {
                false
            }
        } else if self.is_decoding_error() {
            if other.is_decoding_error() {
                self.get_decoding_error_message() == other.get_decoding_error_message()
            } else {
                false
            }
        } else {
            panic!("Unknown InputError variant")
        }
    }
}

impl Eq for InputError {}

impl Clone for InputError {
    fn clone(&self) -> Self {
        if self.is_unsupported_encoding() {
            Self::new_unsupported_encoding(self.get_unsupported_encoding_message().clone())
        } else if self.is_decoding_error() {
            Self::new_decoding_error(self.get_decoding_error_message().clone())
        } else {
            panic!("Unknown InputError variant")
        }
    }
}
