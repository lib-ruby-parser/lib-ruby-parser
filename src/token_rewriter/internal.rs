#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalPtr;
#[cfg(feature = "compile-with-external-structures")]
type Ptr<T> = ExternalPtr<T>;
#[cfg(not(feature = "compile-with-external-structures"))]
type Ptr<T> = Box<T>;

use super::{LexStateAction, RewriteAction};
use crate::Token;

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct InternalTokenRewriterResult {
    pub(crate) rewritten_token: Ptr<Token>,
    pub(crate) token_action: RewriteAction,
    pub(crate) lex_state_action: LexStateAction,
}
