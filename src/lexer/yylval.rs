use crate::bytes::BytesTrait;
use crate::lexer::*;
use crate::Bytes;
use crate::TokenBuf;

pub(crate) trait Yylval {
    fn set_yylval_id(&mut self, id: &str);
    fn set_yylval_literal(&mut self, value: &TokenBuf);
    fn set_yylval_num(&mut self, flags: String);
    fn set_yylval_str(&mut self, value: &TokenBuf);
    fn set_yylval_name(&mut self);
}

impl Yylval for Lexer {
    fn set_yylval_id(&mut self, id: &str) {
        if self.debug {
            println!("set_yylval_id({})", id);
        }
        self.lval = Some(Bytes::new(id.as_bytes().to_vec()));
    }

    fn set_yylval_literal(&mut self, value: &TokenBuf) {
        if self.debug {
            println!(
                "set_yylval_literal({:#?}) ptok = {}, pcur = {}",
                value, self.buffer.ptok, self.buffer.pcur
            );
        }
        self.lval = Some(value.bytes.clone());
    }

    fn set_yylval_num(&mut self, flags: String) {
        if self.debug {
            println!("set_yylval_num {:#?}", flags);
        }
        self.lval = Some(Bytes::new(flags.into_bytes()));
    }

    fn set_yylval_str(&mut self, value: &TokenBuf) {
        if self.debug {
            println!("set_yylval_str {:#?}", value);
        }
        self.lval = Some(value.bytes.clone());
    }

    fn set_yylval_name(&mut self) {
        if self.debug {
            println!("set_yyval_name({:#?})", self.tokenbuf);
        }
        self.lval = Some(self.tokenbuf.bytes.clone());
    }
}
