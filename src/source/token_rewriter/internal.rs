crate::use_native_or_external!(Ptr);

use super::{LexStateAction, RewriteAction};
use crate::Token;

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct InternalTokenRewriterResult {
    pub(crate) rewritten_token: Ptr<Token>,
    pub(crate) token_action: RewriteAction,
    pub(crate) lex_state_action: LexStateAction,
}
