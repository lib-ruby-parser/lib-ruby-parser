use crate::lexer::*;
use crate::TokenBuf;
use crate::lex_char::*;

pub trait TokAdd<T> {
    fn tokadd(&mut self, c: T);
}

impl TokAdd<char> for Lexer {
    fn tokadd(&mut self, c: char) {
        match &mut self.tokenbuf {
            TokenBuf::String(s) => s.push(c),
            TokenBuf::Bytes(bytes) => bytes.append(&mut c.to_string().into_bytes())
        }
    }
}

impl TokAdd<&LexChar> for Lexer {
    fn tokadd(&mut self, c: &LexChar) {
        self.tokadd(c.unwrap())
    }
}

impl TokAdd<&mut LexChar> for Lexer {
    fn tokadd(&mut self, c: &mut LexChar) {
        self.tokadd(c.unwrap())
    }
}

impl TokAdd<u8> for Lexer {
    fn tokadd(&mut self, c: u8) {
        match &mut self.tokenbuf {
            TokenBuf::String(s) => {
                let mut bytes = s.as_bytes().to_vec();
                bytes.push(c);
                self.tokenbuf = TokenBuf::Bytes(bytes);
            }
            TokenBuf::Bytes(bytes) => bytes.push(c)
        }
    }
}
