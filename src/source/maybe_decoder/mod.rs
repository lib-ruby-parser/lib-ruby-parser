#[cfg(not(feature = "compile-with-external-structures"))]
mod native;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use native::MaybeDecoder;

#[cfg(feature = "compile-with-external-structures")]
mod external;
#[cfg(feature = "compile-with-external-structures")]
pub use external::MaybeDecoder;

mod shared;
pub use shared::MaybeDecoderAPI;

#[cfg(test)]
mod tests;
