use crate::lexer::*;
use crate::Bytes;
use crate::TokenBuf;

impl Lexer {
    pub(crate) fn set_yylval_id(&mut self, id: &str) {
        if cfg!(feature = "debug-lexer") {
            println!("set_yylval_id({})", id);
        }
        self.lval = Some(Bytes::new(Vec::from(id)));
    }

    pub(crate) fn set_yylval_literal(&mut self, value: &TokenBuf) {
        if cfg!(feature = "debug-lexer") {
            println!(
                "set_yylval_literal({:#?}) ptok = {}, pcur = {}",
                value, self.buffer.ptok, self.buffer.pcur
            );
        }
        self.lval = Some(value.bytes.clone());
    }

    pub(crate) fn set_yylval_num(&mut self, flags: String) {
        if cfg!(feature = "debug-lexer") {
            println!("set_yylval_num {:#?}", flags);
        }
        self.lval = Some(Bytes::new(Vec::from(flags)));
    }

    pub(crate) fn set_yylval_str(&mut self, value: &TokenBuf) {
        if cfg!(feature = "debug-lexer") {
            println!("set_yylval_str {:#?}", value);
        }
        self.lval = Some(value.bytes.clone());
    }

    pub(crate) fn set_yylval_name(&mut self) {
        if cfg!(feature = "debug-lexer") {
            println!("set_yyval_name({:#?})", self.tokenbuf);
        }
        self.lval = Some(self.tokenbuf.bytes.clone());
    }
}
