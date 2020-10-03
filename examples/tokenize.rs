use std::{convert::TryInto, env};
use std::fs;
use ruby_parser::{Lexer, Token};

fn print_usage() -> ! {
    println!("
USAGE:
    cargo run --example tokenize -- test.rb
    cargo run --example tokenize -- -e \"2 + 2\"
");
    std::process::exit(1)
}

fn token_name(token: &Token) -> String {
    let (id, _, _) = token;
    let first_token: usize = Lexer::YYerror.try_into().unwrap();
    let id_usize: usize = (*id).try_into().unwrap(); // minus first token ID
    Lexer::TOKEN_NAMES[id_usize - first_token + 1].to_owned()
}

fn token_value(token: &Token) -> String {
    let (_, value, _) = token;
    value.to_string_lossy()
}

fn rpad1<T: Sized + std::fmt::Display>(value: &T, total_width: usize) -> String {
    format!("{:width$}", format!("{}, ", value), width = total_width)
}

fn rpad2<T: Sized + std::fmt::Debug>(value: &T, total_width: usize) -> String {
    format!("{:width$}", format!("{:?}, ", value), width = total_width)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().skip(1).map(|e| &e[..]).collect();

    let source =
        match args[..] {
            ["-e", code] => code.to_owned().into_bytes(),
            [filepath] => fs::read(filepath).expect("Failed to read file"),
            _ => print_usage()
        };

    let mut lexer = Lexer::new(&source, None).unwrap();
    lexer.set_debug(true);
    let tokens = lexer.tokenize_until_eof();

    let tok_name_length  = tokens.iter().map(|tok| format!("{:?}", token_name(tok)).len()).max().unwrap_or(0) + 2;
    let tok_value_length = tokens.iter().map(|tok| format!("{:?}", token_value(tok)).len()).max().unwrap_or(0) + 2;

    println!("[");
    for token in tokens {
        let (_, _, loc) = &token;
        let name = rpad1(&token_name(&token), tok_name_length);
        let value = rpad2(&token_value(&token), tok_value_length);
        println!("    :{}{}[{}, {}]", name, value, loc.begin, loc.end);
    }
    println!("]");
}
