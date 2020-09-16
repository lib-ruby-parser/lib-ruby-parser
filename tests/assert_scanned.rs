use ruby_parser::Lexer;
use ruby_parser::lexer::{Token, TokenType};

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut lexer = Lexer::new(source);

    loop {
        let token = lexer.yylex();
        match token {
            Token { token_type: TokenType::END_OF_INPUT, .. } => break,
            _ => tokens.push(token)
        }
    }

    tokens
}

#[macro_export]
macro_rules! assert_scanned {
    ($input:expr, $(:$token_type:tt, $value:expr, [$begin:expr, $end:expr]),*) => {
        {
            use ruby_parser::lexer::{Token, TokenType};
            let actual_tokens = assert_scanned::tokenize($input);

            let token_types : Vec<TokenType>    = vec![$(TokenType::$token_type),*];
            let token_values: Vec<Option<&'static str>> = vec![$($value),*];
            let begins      : Vec<usize>        = vec![$($begin),*];
            let ends        : Vec<usize>        = vec![$($end),*];

            let mut expected_tokens: Vec<Token> = vec![];

            for (idx, token_type) in token_types.iter().enumerate() {
                let token_type = token_type.clone();
                let token_value = token_values[idx].map(|v| v.to_owned());
                let begin = begins[idx];
                let end = ends[idx];

                let token = Token { token_type, token_value, begin, end };
                expected_tokens.push(token);
            }

            assert_eq!(actual_tokens, expected_tokens);
        }
    };
}
