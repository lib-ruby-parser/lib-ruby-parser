use lib_ruby_parser_ast_arena::Bytes;

use crate::lexer::*;
use crate::TokenBuf;

impl<'b> Lexer<'b> {
    pub(crate) fn set_yylval_id(&mut self, id: &'b str) {
        println_if_debug_lexer!("set_yylval_id({})", id);
        let bytes = self.blob.alloc_ref::<Bytes>();
        bytes.append_borrowed(id, self.blob);
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

    pub(crate) fn set_yylval_num(&mut self, flags: String) {
        println_if_debug_lexer!("set_yylval_num {:#?}", flags);
        let bytes = self.blob.alloc_ref::<Bytes>();
        let flags = self.blob.push_str(&flags);
        bytes.append_borrowed(flags, self.blob);
        self.lval = Some(bytes);
    }

    pub(crate) fn set_yylval_str(&mut self, value: &mut TokenBuf<'b>) {
        println_if_debug_lexer!("set_yylval_str {:#?}", value);
        self.lval = Some(value.take().bytes);
    }

    pub(crate) fn set_yylval_name(&mut self) {
        println_if_debug_lexer!(
            "set_yylval_name({:#?})",
            self.tokenbuf.bytes.iter().collect::<Vec<_>>()
        );
        let lval = self.blob.alloc_ref::<Bytes>();
        *lval = *self.tokenbuf.bytes;
        self.lval = Some(lval);
    }
}
