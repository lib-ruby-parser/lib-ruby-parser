use crate::containers::size::DECODER_SIZE;
use crate::containers::ExternalList as List;
use crate::containers::ExternalStringPtr as StringPtr;
use crate::containers::IntoBlob;
use crate::containers::ListBlob;
use crate::containers::StringPtrBlob;
use crate::source::DecoderResult;
use crate::source::DecoderResultBlob;
use crate::source::InputError;
use crate::source::MaybeDecoder;
use crate::source::MaybeDecoderAPI;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct DecoderBlob {
    blob: [u8; DECODER_SIZE],
}

#[cfg(test)]
impl Default for DecoderBlob {
    fn default() -> Self {
        let blob: [u8; DECODER_SIZE] = [0; DECODER_SIZE];
        Self { blob }
    }
}

/// Custom decoder, a wrapper around a function
#[repr(C)]
pub struct Decoder {
    pub(crate) blob: DecoderBlob,
}

extern "C" {
    fn lib_ruby_parser__internal__containers__decoder__call(
        blob: *const DecoderBlob,
        encoding: StringPtrBlob,
        input: ListBlob,
    ) -> DecoderResultBlob;
    fn lib_ruby_parser__internal__containers__decoder_drop(blob: *mut DecoderBlob);
}

impl Drop for Decoder {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__internal__containers__decoder_drop(&mut self.blob) }
    }
}

impl Decoder {
    pub(crate) fn call(&self, encoding: StringPtr, input: List<u8>) -> DecoderResult {
        DecoderResult::from_blob(unsafe {
            lib_ruby_parser__internal__containers__decoder__call(
                &self.blob,
                encoding.into_blob(),
                input.into_blob(),
            )
        })
    }

    #[allow(dead_code)]
    pub(crate) fn from_blob(blob: DecoderBlob) -> Self {
        Self { blob }
    }
}

impl std::fmt::Debug for Decoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decoder").finish()
    }
}

pub fn decode_input(input: List<u8>, enc: StringPtr, decoder: &MaybeDecoder) -> DecoderResult {
    match enc.to_uppercase().as_str() {
        "UTF-8" | "ASCII-8BIT" | "BINARY" => {
            return DecoderResult::new_ok(input.into());
        }
        _ => {
            if let Some(decoder) = decoder.as_decoder() {
                decoder.call(enc, input)
            } else {
                DecoderResult::new_err(InputError::new_unsupported_encoding(enc))
            }
        }
    }
}
