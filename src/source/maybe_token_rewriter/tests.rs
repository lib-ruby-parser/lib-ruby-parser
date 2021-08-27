use super::{MaybeTokenRewriter, MaybeTokenRewriterAPI};
use crate::source::token_rewriter::TokenRewriter;

#[cfg(feature = "compile-with-external-structures")]
fn decoder() -> TokenRewriter {
    use crate::source::token_rewriter::TokenRewriterBlob;
    TokenRewriter::from_blob(TokenRewriterBlob::default())
}

#[cfg(not(feature = "compile-with-external-structures"))]
fn decoder() -> TokenRewriter {
    use crate::source::token_rewriter::{LexStateAction, RewriteAction, TokenRewriterResult};
    use crate::Token;
    TokenRewriter::new(Box::new(|token: Box<Token>, _input: &[u8]| {
        TokenRewriterResult {
            rewritten_token: token,
            token_action: RewriteAction::Keep,
            lex_state_action: LexStateAction::Keep,
        }
    }))
}

#[test]
fn test_some() {
    let decoder = decoder();
    let maybe_decoder = MaybeTokenRewriter::new_some(decoder);

    assert!(maybe_decoder.is_some());
    assert!(!maybe_decoder.is_none());
}

#[test]
fn test_none() {
    let maybe_decoder = MaybeTokenRewriter::new_none();

    assert!(maybe_decoder.is_none());
    assert!(!maybe_decoder.is_some());
}
