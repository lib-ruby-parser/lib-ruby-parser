use crate::lexer::*;
use crate::maybe_byte::*;

pub(crate) trait TokAdd<T> {
    fn tokadd(&mut self, c: T);
}

impl TokAdd<MaybeByte> for Lexer {
    fn tokadd(&mut self, c: MaybeByte) {
        match c {
            MaybeByte::Some(c) => self.tokadd(c),
            MaybeByte::EndOfInput => panic!("can't emit EOF"),
        }
    }
}

impl TokAdd<u8> for Lexer {
    fn tokadd(&mut self, c: u8) {
        self.tokenbuf.push(c)
    }
}
