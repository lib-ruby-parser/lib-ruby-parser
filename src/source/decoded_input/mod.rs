#[cfg(not(feature = "compile-with-external-structures"))]
mod native;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use native::DecodedInput;

#[cfg(feature = "compile-with-external-structures")]
mod external;
#[cfg(feature = "compile-with-external-structures")]
pub use external::DecodedInput;

mod shared;

#[cfg(test)]
mod tests;

#[cfg(feature = "compile-with-external-structures")]
#[allow(unused_imports)]
pub(crate) use external::DecodedInputBlob;
