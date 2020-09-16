use ruby_parser::Lexer;
use ruby_parser::lexer::{Token, TokenType};

pub fn tokenize(lexer: &mut Lexer, source: &str) -> Vec<Token> {
    let mut tokens = vec![];
    lexer.set_source(source);

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
macro_rules! setup_lexer {
    () => {
        {
            use ruby_parser::Lexer;
            Lexer::new("")
        }
    };
}

#[macro_export]
macro_rules! set_lex_state {
    ($lexer:ident, $state:ident) => {
        {
            use ruby_parser::lexer::lex_states::*;
            $lexer.set_lex_state($state);
        }
    };
}

#[macro_export]
macro_rules! assert_scanned {
    ($lexer:expr, $input:expr, $(:$token_type:tt, $value:expr, [$begin:expr, $end:expr]),*) => {
        {
            use ruby_parser::lexer::{Token, TokenType};
            let actual_tokens = tokenize($lexer, $input);

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
