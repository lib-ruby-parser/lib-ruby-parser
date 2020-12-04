use crate::lexer::*;
use crate::TokenBuf;
use crate::TokenValue;

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
        self.lval = Some(TokenValue::String(id.to_owned()));
    }

    fn set_yylval_literal(&mut self, value: &TokenBuf) {
        if self.debug {
            println!(
                "set_yylval_literal({:#?}) ptok = {}, pcur = {}",
                value, self.buffer.ptok, self.buffer.pcur
            );
        }
        self.lval = Some(value.to_token_value());
    }

    fn set_yylval_num(&mut self, flags: String) {
        if self.debug {
            println!("set_yylval_num {:#?}", flags);
        }
        self.lval = Some(TokenValue::String(flags));
    }

    fn set_yylval_str(&mut self, value: &TokenBuf) {
        if self.debug {
            println!("set_yylval_str {:#?}", value);
        }
        self.lval = Some(value.to_token_value());
    }

    fn set_yylval_name(&mut self) {
        if self.debug {
            println!("set_yyval_name({:#?})", self.tokenbuf);
        }
        self.lval = Some(self.tokenbuf.to_token_value());
    }
}
