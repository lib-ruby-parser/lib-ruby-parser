use lib_ruby_parser_ast::ByteArray;

use crate::lexer::*;
use crate::TokenBuf;

impl<'b> Lexer<'b> {
    pub(crate) fn set_yylval_id(&mut self, id: &'b str) {
        println_if_debug_lexer!("set_yylval_id({})", id);
        let bytes = ByteArray::new(self.blob);
        bytes.push_str(id, self.blob);
        self.lval = Some(bytes);
    }

    pub(crate) fn set_yylval_literal(&mut self, value: &mut TokenBuf<'b>) {
        println_if_debug_lexer!(
            "set_yylval_literal({:#?}) ptok = {}, pcur = {}",
            value,
            self.buffer.ptok,
            self.buffer.pcur
        );
        self.lval = Some(value.take().bytes);
    }

    pub(crate) fn set_yylval_num(&mut self, flags: &str) {
        println_if_debug_lexer!("set_yylval_num {:#?}", flags);
        let bytes = ByteArray::new(self.blob);
        bytes.push_str(flags, self.blob);
        self.lval = Some(bytes);
    }

    pub(crate) fn set_yylval_str(&mut self, value: &mut TokenBuf<'b>) {
        println_if_debug_lexer!("set_yylval_str {:#?}", value);
        self.lval = Some(value.take().bytes);
    }

    pub(crate) fn set_yylval_name(&mut self) {
        println_if_debug_lexer!("set_yylval_name({:#?})", self.tokenbuf.bytes.as_slice());
        // let lval = self.blob.alloc_mut::<Bytes>();
        // Bytes::shallow_copy(self.tokenbuf.bytes, lval);
        self.lval = Some(self.tokenbuf.bytes);
    }
}
