#[cfg(not(feature = "compile-with-external-structures"))]
mod native;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use native::Token;

#[cfg(feature = "compile-with-external-structures")]
mod external;
#[cfg(feature = "compile-with-external-structures")]
pub use external::Token;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use external::TokenBlob;

mod shared;

#[cfg(test)]
mod tests;
