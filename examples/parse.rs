use ruby_parser::{Parser, Lexer, Node};
use std::env;
use std::fs;

fn print_usage() -> ! {
    println!("
USAGE:
    cargo run --example parse -- test.rb
    cargo run --example parse -- -e \"2 + 2\"
");
    std::process::exit(1)
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

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    parser.yydebug = 1;

    match parser.do_parse() {
        Some(node) => println!("{}", node.inspect(0)),
        None => println!("None")
    }
}
