#[cfg(not(feature = "compile-with-external-structures"))]
mod native;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use native::DecoderResult;

#[cfg(feature = "compile-with-external-structures")]
mod external;
#[cfg(feature = "compile-with-external-structures")]
pub use external::DecoderResult;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use external::DecoderResultBlob;

#[cfg(test)]
mod tests;
