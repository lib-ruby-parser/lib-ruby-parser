use crate::containers::size::DECODER_RESULT_SIZE;
use crate::containers::ExternalList as List;
use crate::containers::IntoBlob;
use crate::containers::ListBlob;
use crate::source::InputError;
use crate::source::InputErrorBlob;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct DecoderResultBlob {
    blob: [u8; DECODER_RESULT_SIZE],
}

/// Result that is returned from decoding function
#[repr(C)]
pub struct DecoderResult {
    pub(crate) blob: DecoderResultBlob,
}

extern "C" {
    fn lib_ruby_parser__external__decoder_result__new_ok(byte_list: ListBlob) -> DecoderResultBlob;
    fn lib_ruby_parser__external__decoder_result__new_err(
        input_error: InputErrorBlob,
    ) -> DecoderResultBlob;
    fn lib_ruby_parser__external__decoder_result__drop(blob: *mut DecoderResultBlob);
    fn lib_ruby_parser__external__decoder_result_is_ok(blob: *const DecoderResultBlob) -> bool;
    fn lib_ruby_parser__external__decoder_result_is_err(blob: *const DecoderResultBlob) -> bool;
    fn lib_ruby_parser__external__decoder_result_into_ok(blob: DecoderResultBlob) -> ListBlob;
    fn lib_ruby_parser__external__decoder_result_into_err(
        blob: DecoderResultBlob,
    ) -> InputErrorBlob;
    fn lib_ruby_parser__external__decoder_result_as_ok(
        blob: *const DecoderResultBlob,
    ) -> *const ListBlob;
    fn lib_ruby_parser__external__decoder_result_as_err(
        blob: *const DecoderResultBlob,
    ) -> *const InputErrorBlob;
}

impl Drop for DecoderResult {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__external__decoder_result__drop(&mut self.blob) }
    }
}

impl DecoderResult {
    pub(crate) fn new_ok(output: List<u8>) -> Self {
        let blob = unsafe { lib_ruby_parser__external__decoder_result__new_ok(output.into_blob()) };
        Self { blob }
    }

    pub(crate) fn new_err(err: InputError) -> Self {
        let blob = unsafe { lib_ruby_parser__external__decoder_result__new_err(err.into_blob()) };
        Self { blob }
    }

    pub(crate) fn is_ok(&self) -> bool {
        unsafe { lib_ruby_parser__external__decoder_result_is_ok(&self.blob) }
    }

    pub(crate) fn is_err(&self) -> bool {
        unsafe { lib_ruby_parser__external__decoder_result_is_err(&self.blob) }
    }

    pub(crate) fn as_ok(&self) -> &List<u8> {
        unsafe {
            (lib_ruby_parser__external__decoder_result_as_ok(&self.blob) as *const List<u8>)
                .as_ref()
                .unwrap()
        }
    }

    pub(crate) fn as_err(&self) -> &InputError {
        unsafe {
            (lib_ruby_parser__external__decoder_result_as_err(&self.blob) as *const InputError)
                .as_ref()
                .unwrap()
        }
    }

    pub(crate) fn unwrap_ok(self) -> List<u8> {
        let list_blob = unsafe { lib_ruby_parser__external__decoder_result_into_ok(self.blob) };
        std::mem::forget(self);
        List::<u8>::from_blob(list_blob)
    }

    pub(crate) fn unwrap_err(self) -> InputError {
        let input_error_blob =
            unsafe { lib_ruby_parser__external__decoder_result_into_err(self.blob) };
        std::mem::forget(self);
        InputError::from_blob(input_error_blob)
    }

    pub(crate) fn as_result(&self) -> Result<&List<u8>, &InputError> {
        if self.is_ok() {
            Ok(self.as_ok())
        } else if self.is_err() {
            Err(self.as_err())
        } else {
            panic!("Unknown DecoderResult variant")
        }
    }

    pub(crate) fn into_result(self) -> Result<List<u8>, InputError> {
        if self.is_ok() {
            Ok(self.unwrap_ok())
        } else if self.is_err() {
            Err(self.unwrap_err())
        } else {
            panic!("Unknown DecoderResult variant")
        }
    }

    pub(crate) fn from_blob(blob: DecoderResultBlob) -> Self {
        Self { blob }
    }
}

impl std::fmt::Debug for DecoderResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_result())
    }
}

impl PartialEq for DecoderResult {
    fn eq(&self, other: &Self) -> bool {
        self.as_result() == other.as_result()
    }
}

impl Eq for DecoderResult {}

impl Clone for DecoderResult {
    fn clone(&self) -> Self {
        if self.is_ok() {
            Self::new_ok(self.as_ok().clone())
        } else if self.is_err() {
            Self::new_err(self.as_err().clone())
        } else {
            panic!("Unknown DecoderResult variant")
        }
    }
}
