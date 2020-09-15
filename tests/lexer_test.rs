use ruby_parser::Lexer;
use ruby_parser::lexer::{Token, TokenType};

fn tokenize(source: &str) -> Vec<Token> {
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

macro_rules! assert_scanned {
    ($input:expr, $(:$token_type:tt, $value:expr, [$begin:expr, $end:expr]),+) => {
        {
            let actual_tokens = tokenize($input);

            let token_types : Vec<TokenType>    = vec![$(TokenType::$token_type),+];
            let token_values: Vec<&'static str> = vec![$($value),+];
            let begins      : Vec<usize>        = vec![$($begin),+];
            let ends        : Vec<usize>        = vec![$($end),+];

            let mut expected_tokens: Vec<Token> = vec![];

            for (idx, token_type) in token_types.iter().enumerate() {
                let token_type = token_type.clone();
                let token_value = token_values[idx].into();
                let begin = begins[idx];
                let end = ends[idx];

                let token = Token { token_type, token_value, begin, end };
                expected_tokens.push(token);
            }

            assert_eq!(actual_tokens, expected_tokens);
        }
    };
}

#[test]
fn test_lexer_basic() {
    assert_scanned!("10 + 20",
                    :tINTEGER, "10", [0, 2],
                    :tPLUS,    "+",  [3, 4],
                    :tINTEGER, "20", [5, 7]);
}
