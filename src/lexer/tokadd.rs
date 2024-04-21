use crate::lexer::*;
use crate::maybe_byte::*;

pub(crate) trait TokAdd<T> {
    fn tokadd(&mut self, c: T);
}

impl TokAdd<MaybeByte> for Lexer<'_> {
    fn tokadd(&mut self, c: MaybeByte) {
        match c {
            MaybeByte::Some(c) => self.tokadd(c),
            MaybeByte::EndOfInput => panic!("can't emit EOF"),
        }
    }
}

impl TokAdd<u8> for Lexer<'_> {
    fn tokadd(&mut self, c: u8) {
        self.tokenbuf.append_invalid_escaped(c)
    }
}
