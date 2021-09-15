use crate::lexer::*;
use crate::Bytes;
use crate::TokenBuf;

impl Lexer {
    pub(crate) fn set_yylval_id(&mut self, id: &str) {
        if self.debug {
            println!("set_yylval_id({})", id);
        }
        self.lval = Some(Bytes::new(id.as_bytes().to_vec()));
    }

    pub(crate) fn set_yylval_literal(&mut self, value: &TokenBuf) {
        if self.debug {
            println!(
                "set_yylval_literal({:#?}) ptok = {}, pcur = {}",
                value, self.buffer.ptok, self.buffer.pcur
            );
        }
        self.lval = Some(value.bytes.clone());
    }

    pub(crate) fn set_yylval_num(&mut self, flags: String) {
        if self.debug {
            println!("set_yylval_num {:#?}", flags);
        }
        self.lval = Some(Bytes::new(flags.into_bytes()));
    }

    pub(crate) fn set_yylval_str(&mut self, value: &TokenBuf) {
        if self.debug {
            println!("set_yylval_str {:#?}", value);
        }
        self.lval = Some(value.bytes.clone());
    }

    pub(crate) fn set_yylval_name(&mut self) {
        if self.debug {
            println!("set_yyval_name({:#?})", self.tokenbuf);
        }
        self.lval = Some(self.tokenbuf.bytes.clone());
    }
}
