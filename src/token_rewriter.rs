use crate::Token;

pub enum RewriteAction {
    Drop,
    Keep,
}

pub enum LexStateAction {
    Set(i32),
    Keep,
}

pub trait TokenRewriter: std::fmt::Debug {
    fn rewrite_token(
        &mut self,
        token: Token,
        input: &[u8],
    ) -> (Token, RewriteAction, LexStateAction);
}
