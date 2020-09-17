use std::env;
use std::fs;
use ruby_parser::State;
use ruby_parser::lexer::Token;

fn print_usage() -> ! {
    println!("
USAGE:
    cargo run --example tokenize -- test.rb
    cargo run --example tokenize -- -e \"2 + 2\"
");
    std::process::exit(1)
}

fn rpad<T: Sized + std::fmt::Debug>(value: &T, total_width: usize) -> String {
    format!("{:width$}", format!("{:?}, ", value), width = total_width)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().skip(1).map(|e| &e[..]).collect();

    let source =
        match args[..] {
            ["-e", code] => code.to_owned(),
            [filepath] => fs::read_to_string(filepath).expect("Failed to read file"),
            _ => print_usage()
        };

    let lexer = State::new(&source);
    let mut tokens = vec![];

    loop {
        let token = lexer.borrow_mut().yylex();
        match token {
            Token::END_OF_INPUT(..) => break,
            _ => tokens.push(token)
        }
    }

    let tok_name_length  = tokens.iter().map(|tok| format!("{:?}", tok.name()).len()).max().unwrap_or(0) + 2;
    let tok_value_length = tokens.iter().map(|tok| format!("{:?}", tok.value()).len()).max().unwrap_or(0) + 2;

    println!("[");
    for token in tokens {
        let name = rpad(&token.name(), tok_name_length);
        let value = rpad(&token.value(), tok_value_length);
        println!("    :{}{}[{}, {}]", name, value, token.begin(), token.end());
    }
    println!("]");
}
