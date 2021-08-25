#[cfg(not(feature = "compile-with-external-structures"))]
mod native;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use native::MaybeTokenRewriter;

#[cfg(feature = "compile-with-external-structures")]
mod external;
#[cfg(feature = "compile-with-external-structures")]
pub use external::MaybeTokenRewriter;

#[cfg(feature = "compile-with-external-structures")]
pub(crate) use external::MaybeTokenRewriterBlob;

mod shared;
pub use shared::MaybeTokenRewriterAPI;

#[cfg(test)]
mod tests;
