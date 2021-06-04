use lib_ruby_parser::{token_name, Token, TokenTrait};

pub struct TokenList {
    pub tokens: Vec<Token>,
}

fn token_value(token: &Token) -> String {
    token.to_string_lossy()
}

impl TokenList {
    fn tok_name_length(&self) -> usize {
        self.tokens
            .iter()
            .map(|tok| format!("{:?}", token_name(tok.token_type())).len())
            .max()
            .unwrap_or(0)
            + 2
    }

    fn tok_value_length(&self) -> usize {
        self.tokens
            .iter()
            .map(|tok| format!("{:?}", token_value(tok)).len())
            .max()
            .unwrap_or(0)
            + 2
    }
}

fn rpad<T: Sized + std::fmt::Debug>(value: &T, total_width: usize) -> String {
    format!("{:width$}", format!("{:?}, ", value), width = total_width)
}

impl std::fmt::Debug for TokenList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tok_name_length = self.tok_name_length();
        let tok_value_length = self.tok_value_length();

        let tokens = self
            .tokens
            .iter()
            .map(|token| {
                let name = rpad(&token_name(token.token_type()), tok_name_length);
                let value = rpad(&token_value(&token), tok_value_length);

                format!(
                    "    :{}{}[{}, {}]",
                    name,
                    value,
                    token.loc().begin,
                    token.loc().end,
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "[\n{}\n]", tokens)
    }
}
