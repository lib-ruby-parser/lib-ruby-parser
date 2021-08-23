#[cfg(not(feature = "compile-with-external-structures"))]
mod native;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use native::{decode_input, Decoder, DecoderResult};

#[cfg(feature = "compile-with-external-structures")]
mod external;
#[cfg(feature = "compile-with-external-structures")]
pub use external::{decode_input, Decoder, DecoderResult};
// #[cfg(feature = "compile-with-external-structures")]
// pub(crate) use external::DecoderBlob;

mod shared;

#[cfg(test)]
mod tests;
