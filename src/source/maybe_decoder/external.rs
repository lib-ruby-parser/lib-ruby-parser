use crate::blobs::{Blob, HasBlob};
use crate::source::maybe_decoder::MaybeDecoderAPI;
use crate::source::Decoder;

/// Custom decoder, a wrapper around a function
#[repr(C)]
pub struct MaybeDecoder {
    pub(crate) blob: Blob<MaybeDecoder>,
}

extern "C" {
    fn lib_ruby_parser__external__maybe_decoder__new_some(
        blob: Blob<Decoder>,
    ) -> Blob<MaybeDecoder>;
    fn lib_ruby_parser__external__maybe_decoder__new_none() -> Blob<MaybeDecoder>;
    fn lib_ruby_parser__external__maybe_decoder__drop(blob: *mut Blob<MaybeDecoder>);
    fn lib_ruby_parser__external__maybe_decoder__is_some(blob: *const Blob<MaybeDecoder>) -> bool;
    fn lib_ruby_parser__external__maybe_decoder__is_none(blob: *const Blob<MaybeDecoder>) -> bool;
    fn lib_ruby_parser__external__maybe_decoder__as_decoder(
        blob: *const Blob<MaybeDecoder>,
    ) -> *const Blob<Decoder>;
    fn lib_ruby_parser__external__maybe_decoder__into_decoder(
        blob: Blob<MaybeDecoder>,
    ) -> Blob<Decoder>;
}

impl Drop for MaybeDecoder {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__external__maybe_decoder__drop(&mut self.blob) }
    }
}

impl MaybeDecoder {}

impl MaybeDecoderAPI for MaybeDecoder {
    fn new_some(decoder: Decoder) -> Self {
        let blob =
            unsafe { lib_ruby_parser__external__maybe_decoder__new_some(decoder.into_blob()) };
        Self { blob }
    }

    fn new_none() -> Self {
        let blob = unsafe { lib_ruby_parser__external__maybe_decoder__new_none() };
        Self { blob }
    }

    fn is_some(&self) -> bool {
        unsafe { lib_ruby_parser__external__maybe_decoder__is_some(&self.blob) }
    }

    fn is_none(&self) -> bool {
        unsafe { lib_ruby_parser__external__maybe_decoder__is_none(&self.blob) }
    }

    fn as_decoder(&self) -> Option<&Decoder> {
        unsafe {
            (lib_ruby_parser__external__maybe_decoder__as_decoder(&self.blob) as *const Decoder)
                .as_ref()
        }
    }

    fn as_decoder_mut(&mut self) -> Option<&mut Decoder> {
        unsafe {
            (lib_ruby_parser__external__maybe_decoder__as_decoder(&self.blob) as *mut Decoder)
                .as_mut()
        }
    }

    fn into_decoder(self) -> Decoder {
        let decoder = Decoder::from_blob(unsafe {
            lib_ruby_parser__external__maybe_decoder__into_decoder(self.blob)
        });
        std::mem::forget(self);
        decoder
    }
}

impl std::fmt::Debug for MaybeDecoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_decoder())
    }
}

impl Default for MaybeDecoder {
    fn default() -> Self {
        Self::new_none()
    }
}
