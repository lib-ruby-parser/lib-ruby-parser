use crate::containers::size::INPUT_ERROR_SIZE;
use crate::containers::ExternalStringPtr as StringPtr;
use crate::containers::IntoBlob;
use crate::containers::StringPtrBlob;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct InputErrorBlob {
    blob: [u8; INPUT_ERROR_SIZE],
}

/// An enum with all possible kinds of errors that can be returned
/// from a decoder
#[repr(C)]
pub struct InputError {
    pub(crate) blob: InputErrorBlob,
}

extern "C" {
    fn lib_ruby_parser__internal__containers__input_error__new_unsupported_encoding(
        err: StringPtrBlob,
    ) -> InputErrorBlob;
    fn lib_ruby_parser__internal__containers__input_error__new_decoding_error(
        err: StringPtrBlob,
    ) -> InputErrorBlob;
    fn lib_ruby_parser__internal__containers__input_error__is_unsupported_encoding(
        blob: *const InputErrorBlob,
    ) -> bool;
    fn lib_ruby_parser__internal__containers__input_error__is_decoding_error(
        blob: *const InputErrorBlob,
    ) -> bool;
    fn lib_ruby_parser__internal__containers__input_error__get_unsupported_encoding(
        blob: *const InputErrorBlob,
    ) -> *const StringPtrBlob;
    fn lib_ruby_parser__internal__containers__input_error__get_decoding_error(
        blob: *const InputErrorBlob,
    ) -> *const StringPtrBlob;
    fn lib_ruby_parser__internal__containers__input_error__drop(blob: *mut InputErrorBlob);
}

impl Drop for InputError {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__internal__containers__input_error__drop(&mut self.blob) }
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
            lib_ruby_parser__internal__containers__input_error__new_unsupported_encoding(
                err.into_blob(),
            )
        };
        Self { blob }
    }

    /// Constructs DecodingError variant
    pub fn new_decoding_error(err: StringPtr) -> Self {
        let blob = unsafe {
            lib_ruby_parser__internal__containers__input_error__new_decoding_error(err.into_blob())
        };
        Self { blob }
    }

    /// Returns `true` if `self` is a `UnupportedEncoding` variant
    pub(crate) fn is_unsupported_encoding(&self) -> bool {
        unsafe {
            lib_ruby_parser__internal__containers__input_error__is_unsupported_encoding(&self.blob)
        }
    }

    /// Returns `true` if `self` is a `DecodingError` variant
    pub(crate) fn is_decoding_error(&self) -> bool {
        unsafe { lib_ruby_parser__internal__containers__input_error__is_decoding_error(&self.blob) }
    }

    /// Returns StringPtr of the `UnupportedEncoding` variant. Panics if variant doesn't match
    pub(crate) fn get_unsupported_encoding_message(&self) -> &StringPtr {
        unsafe {
            (lib_ruby_parser__internal__containers__input_error__get_unsupported_encoding(
                &self.blob,
            ) as *const StringPtr)
                .as_ref()
                .unwrap()
        }
    }

    /// Returns StringPtr of the `DecodingError` variant. Panics if variant doesn't match
    pub(crate) fn get_decoding_error_message(&self) -> &StringPtr {
        unsafe {
            (lib_ruby_parser__internal__containers__input_error__get_decoding_error(&self.blob)
                as *const StringPtr)
                .as_ref()
                .unwrap()
        }
    }
}
