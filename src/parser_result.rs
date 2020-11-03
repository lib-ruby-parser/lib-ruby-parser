use crate::source::Comment;
use crate::Diagnostic;
use crate::Node;
use crate::Token;

#[derive(Debug)]
pub struct ParserResult {
    pub ast: Option<Node>,
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
    pub comments: Vec<Comment>,
}
