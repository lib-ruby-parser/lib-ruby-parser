crate::use_native_or_external!(Maybe);

use crate::blobs::{Blob, HasBlob};
use crate::containers::ExternalList as List;
use crate::containers::ExternalStringPtr as StringPtr;
use crate::source::DecoderResult;
use crate::source::InputError;

/// Custom decoder, a wrapper around a function
#[repr(C)]
pub struct Decoder {
    pub(crate) blob: Blob<Decoder>,
}

extern "C" {
    fn lib_ruby_parser__external__decoder__call(
        blob: *const Blob<Decoder>,
        encoding: Blob<StringPtr>,
        input: Blob<List<u8>>,
    ) -> Blob<DecoderResult>;
    fn lib_ruby_parser__external__decoder_drop(blob: *mut Blob<Decoder>);
}

impl Drop for Decoder {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__external__decoder_drop(&mut self.blob) }
    }
}

impl Decoder {
    pub(crate) fn call(&self, encoding: StringPtr, input: List<u8>) -> DecoderResult {
        DecoderResult::from_blob(unsafe {
            lib_ruby_parser__external__decoder__call(
                &self.blob,
                encoding.into_blob(),
                input.into_blob(),
            )
        })
    }
}

impl std::fmt::Debug for Decoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decoder").finish()
    }
}

pub fn decode_input(
    input: List<u8>,
    enc: StringPtr,
    decoder: &mut Maybe<Decoder>,
) -> DecoderResult {
    match enc.to_uppercase().as_str() {
        "UTF-8" | "ASCII-8BIT" | "BINARY" => DecoderResult::new_ok(input),
        _ => {
            if let Some(decoder) = decoder.as_ref() {
                decoder.call(enc, input)
            } else {
                DecoderResult::new_err(InputError::new_unsupported_encoding(enc))
            }
        }
    }
}
