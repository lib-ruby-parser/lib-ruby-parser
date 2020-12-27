use crate::lex_states;
use crate::token_rewriter::{LexStateAction, RewriteAction, TokenRewriter};
use crate::{reserved_word, Lexer, Token};

#[cfg(feature = "sorbet")]
#[derive(Default, Debug)]
pub(crate) struct SorbetTokenRewriter {
    pub(crate) got_nl_in_expr_dot: bool,
}

impl SorbetTokenRewriter {
    pub(crate) fn new_boxed() -> Box<dyn TokenRewriter> {
        Box::new(Self::default())
    }
}

fn map_to_keyword(bytes: &[u8]) -> Option<i32> {
    let token_value = std::str::from_utf8(bytes).ok()?;
    let keyword = reserved_word(token_value)?;
    Some(keyword.id)
}

impl TokenRewriter for SorbetTokenRewriter {
    fn rewrite_token(
        &mut self,
        mut token: Token,
        input: &[u8],
    ) -> (Token, RewriteAction, LexStateAction) {
        // println!(
        //     "{:?} {:?} char after = {:?}",
        //     self,
        //     token,
        //     input.get(token.loc.end).map(|c| *c as char)
        // );
        if token.lex_state_after.is_some(lex_states::EXPR_DOT) && input[token.loc.end] == b'\n' {
            // We just consumed '.' or '&.' and the next char is '\n':
            //   foo.
            //     bar
            self.got_nl_in_expr_dot = true;
            // println!("Recording got_nl_in_expr_dot = true {:?}", token);
            return (token, RewriteAction::Keep, LexStateAction::Keep);
        }

        if token.lex_state_before.is_some(lex_states::EXPR_DOT)
            && self.got_nl_in_expr_dot
            && token.token_type == Lexer::tIDENTIFIER
        {
            self.got_nl_in_expr_dot = false;
            if let Some(keyword) = map_to_keyword(token.token_value.as_bytes()) {
                // rewrite
                // println!(
                //     "rewriting {} into keyword",
                //     token.token_value.to_string_lossy()
                // );
                token.token_type = keyword;
                return (
                    token,
                    RewriteAction::Keep,
                    LexStateAction::Set(lex_states::EXPR_END),
                );
            }
        }

        // instantly reset it otherwise
        self.got_nl_in_expr_dot = false;

        (token, RewriteAction::Keep, LexStateAction::Keep)
    }
}
