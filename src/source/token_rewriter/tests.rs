crate::use_native_or_external!(Ptr);
crate::use_native_or_external!(List);

use crate::source::token_rewriter::InternalTokenRewriterResult;

use super::{TokenRewriter, TokenRewriterResult};
use crate::Bytes;
use crate::LexState;
use crate::Loc;
use crate::Token;

const INITIAL_TOKEN_ID: i32 = 310;
const REWRITTEN_TOKEN_ID: i32 = 300;

fn rewritten_token() -> Ptr<Token> {
    Ptr::new(Token::new(
        REWRITTEN_TOKEN_ID,
        Bytes::new(List::from("rewritten")),
        Loc::new(1, 2),
        LexState { value: 1 },
        LexState { value: 2 },
    ))
}

#[cfg(feature = "compile-with-external-structures")]
mod dummy_rewriter {
    use super::rewritten_token;
    use super::{Ptr, Token};
    use crate::blobs::{Blob, HasBlob};
    use crate::source::token_rewriter::TokenRewriter;

    extern "C" {
        fn lib_ruby_parser__testing__token_rewriter__new_keep(
            token_f: extern "C" fn() -> Ptr<Token>,
        ) -> Blob<TokenRewriter>;
        fn lib_ruby_parser__testing__token_rewriter__new_drop(
            token_f: extern "C" fn() -> Ptr<Token>,
        ) -> Blob<TokenRewriter>;
        fn lib_ruby_parser__testing__token_rewriter__new_rewrite(
            token_f: extern "C" fn() -> Ptr<Token>,
        ) -> Blob<TokenRewriter>;
    }

    extern "C" fn token_f() -> Ptr<Token> {
        rewritten_token()
    }

    pub(crate) fn dummy_decoder_keep() -> TokenRewriter {
        TokenRewriter::from_blob(unsafe {
            lib_ruby_parser__testing__token_rewriter__new_keep(token_f)
        })
    }

    pub(crate) fn dummy_decoder_drop() -> TokenRewriter {
        TokenRewriter::from_blob(unsafe {
            lib_ruby_parser__testing__token_rewriter__new_drop(token_f)
        })
    }

    pub(crate) fn dummy_decoder_rewrite() -> TokenRewriter {
        TokenRewriter::from_blob(unsafe {
            lib_ruby_parser__testing__token_rewriter__new_rewrite(token_f)
        })
    }
}

#[cfg(not(feature = "compile-with-external-structures"))]
mod dummy_rewriter {
    use super::rewritten_token;
    use crate::source::token_rewriter::{
        LexStateAction, RewriteAction, TokenRewriter, TokenRewriterResult,
    };

    pub(crate) fn dummy_decoder_keep() -> TokenRewriter {
        TokenRewriter::new(Box::new(|token, _input| TokenRewriterResult {
            rewritten_token: token,
            token_action: RewriteAction::Keep,
            lex_state_action: LexStateAction::Keep,
        }))
    }
    pub(crate) fn dummy_decoder_drop() -> TokenRewriter {
        TokenRewriter::new(Box::new(|token, _input| TokenRewriterResult {
            rewritten_token: token,
            token_action: RewriteAction::Drop,
            lex_state_action: LexStateAction::Keep,
        }))
    }
    pub(crate) fn dummy_decoder_rewrite() -> TokenRewriter {
        TokenRewriter::new(Box::new(|_token, _input| TokenRewriterResult {
            rewritten_token: rewritten_token(),
            token_action: RewriteAction::Keep,
            lex_state_action: LexStateAction::Keep,
        }))
    }
}

fn call_dummy_rewriter(rewriter: TokenRewriter) -> TokenRewriterResult {
    // it's dummy, so encoding/input doesn't matter
    let token = Ptr::new(Token::new(
        INITIAL_TOKEN_ID,
        Bytes::new(List::from("initial")),
        Loc::new(1, 2),
        LexState { value: 1 },
        LexState { value: 2 },
    ));
    let input = list![b'2', b'+', b'2'];

    rewriter.call(token, input.as_slice())
}

#[test]
fn test_keep() {
    let InternalTokenRewriterResult {
        rewritten_token,
        token_action,
        ..
    } = call_dummy_rewriter(dummy_rewriter::dummy_decoder_keep()).into_internal();

    assert_eq!(rewritten_token.token_type(), INITIAL_TOKEN_ID);
    assert_eq!(
        rewritten_token.token_value(),
        &Bytes::new(List::from("initial"))
    );
    assert!(token_action.is_keep());
}

#[test]
fn test_drop() {
    let InternalTokenRewriterResult { token_action, .. } =
        call_dummy_rewriter(dummy_rewriter::dummy_decoder_drop()).into_internal();

    assert!(token_action.is_drop());
}

#[test]
fn test_rewrite() {
    let InternalTokenRewriterResult {
        token_action,
        rewritten_token,
        ..
    } = call_dummy_rewriter(dummy_rewriter::dummy_decoder_rewrite()).into_internal();

    assert_eq!(rewritten_token.token_type(), REWRITTEN_TOKEN_ID);
    assert_eq!(
        rewritten_token.token_value(),
        &Bytes::new(List::from("rewritten"))
    );
    assert!(token_action.is_keep());
}
