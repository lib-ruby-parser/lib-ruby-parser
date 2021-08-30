#[cfg(not(feature = "compile-with-external-structures"))]
mod native;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use native::{LexStateAction, RewriteAction, TokenRewriter, TokenRewriterResult};

#[cfg(feature = "compile-with-external-structures")]
mod external;
#[cfg(feature = "compile-with-external-structures")]
pub use external::{LexStateAction, RewriteAction, TokenRewriter, TokenRewriterResult};

mod shared;

mod internal;
pub(crate) use internal::InternalTokenRewriterResult;

#[cfg(test)]
mod tests;
