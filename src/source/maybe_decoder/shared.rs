use crate::source::Decoder;

/// Trait with common MaybeDecoder APIs
pub trait MaybeDecoderAPI {
    /// Constructs `Some` variant
    fn new_some(decoder: Decoder) -> Self
    where
        Self: Sized;

    /// Constructs `None` variant
    fn new_none() -> Self
    where
        Self: Sized;

    /// Returns true if `self` is `Some`
    fn is_some(&self) -> bool;

    /// Returns true if `self` is `None`
    fn is_none(&self) -> bool;

    /// Casts &self to Option<&Decoder>
    fn as_decoder(&self) -> Option<&Decoder>;

    /// Casts &mut self to Option<&mut Decoder>
    fn as_decoder_mut(&mut self) -> Option<&mut Decoder>;

    /// Casts self to Decoder. Panics if self is `None`
    fn into_decoder(self) -> Decoder;
}
