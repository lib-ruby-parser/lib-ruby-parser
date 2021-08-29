use crate::containers::size::MAYBE_DECODER_SIZE;
use crate::containers::IntoBlob;
use crate::source::maybe_decoder::MaybeDecoderAPI;
use crate::source::{Decoder, DecoderBlob};

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct MaybeDecoderBlob {
    blob: [u8; MAYBE_DECODER_SIZE],
}

/// Custom decoder, a wrapper around a function
#[repr(C)]
pub struct MaybeDecoder {
    pub(crate) blob: MaybeDecoderBlob,
}

extern "C" {
    fn lib_ruby_parser__external__maybe_decoder__new_some(blob: DecoderBlob) -> MaybeDecoderBlob;
    fn lib_ruby_parser__external__maybe_decoder__new_none() -> MaybeDecoderBlob;
    fn lib_ruby_parser__external__maybe_decoder__drop(blob: *mut MaybeDecoderBlob);
    fn lib_ruby_parser__external__maybe_decoder__is_some(blob: *const MaybeDecoderBlob) -> bool;
    fn lib_ruby_parser__external__maybe_decoder__is_none(blob: *const MaybeDecoderBlob) -> bool;
    fn lib_ruby_parser__external__maybe_decoder__as_decoder(
        blob: *const MaybeDecoderBlob,
    ) -> *const DecoderBlob;
    fn lib_ruby_parser__external__maybe_decoder__into_decoder(
        blob: MaybeDecoderBlob,
    ) -> DecoderBlob;
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
