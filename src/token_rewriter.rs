use crate::Token;

#[derive(Debug)]
pub enum RewriteAction {
    Drop,
    Keep,
}

#[derive(Debug)]
pub enum LexStateAction {
    Set(i32),
    Keep,
}

pub trait TokenRewriter: std::fmt::Debug {
    fn rewrite_token(
        &mut self,
        token: Box<Token>,
        input: &[u8],
    ) -> (Box<Token>, RewriteAction, LexStateAction);
}
