use crate::source::maybe_decoder::MaybeDecoderAPI;
use crate::source::Decoder;

/// Native MaybeDecoder type.
pub type MaybeDecoder = Option<Decoder>;

impl MaybeDecoderAPI for MaybeDecoder {
    fn new_some(decoder: Decoder) -> Self {
        Some(decoder)
    }

    fn new_none() -> Self {
        None
    }

    fn is_some(&self) -> bool {
        matches!(self, Some(_))
    }

    fn is_none(&self) -> bool {
        matches!(self, None)
    }

    fn as_decoder(&self) -> Option<&Decoder> {
        self.as_ref()
    }

    fn as_decoder_mut(&mut self) -> Option<&mut Decoder> {
        self.as_mut()
    }

    fn into_decoder(self) -> Decoder {
        self.unwrap()
    }
}
