use crate::lexer::TokenType;

#[derive(Clone, Eq, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub token_value: Option<String>,
    pub begin: usize,
    pub end: usize,
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entry(&self.token_type)
            .entry(&self.token_value)
            .entry(&[self.begin, self.end])
            .finish()
    }
}
