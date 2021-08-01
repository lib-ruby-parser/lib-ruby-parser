#[cfg(not(feature = "compile-with-external-structures"))]
mod native;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use native::MagicCommentKind;

#[cfg(feature = "compile-with-external-structures")]
mod external;
#[cfg(feature = "compile-with-external-structures")]
pub use external::MagicCommentKind;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use external::MagicCommentKindBlob;

mod shared;

#[cfg(test)]
mod tests;
