#[cfg(not(feature = "compile-with-external-structures"))]
mod native;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use native::Bytes;

#[cfg(feature = "compile-with-external-structures")]
mod external;
#[cfg(feature = "compile-with-external-structures")]
pub use external::Bytes;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use external::BytesBlob;

mod shared;

#[cfg(test)]
mod tests;
