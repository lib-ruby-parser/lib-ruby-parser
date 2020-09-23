use ruby_parser::{Parser, Loc, DummyLexer};
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

    let _source =
        match args[..] {
            ["-e", code] => code.to_owned(),
            [filepath] => fs::read_to_string(filepath).expect("Failed to read file"),
            _ => print_usage()
        };

    let tokens = vec![
        (
            DummyLexer::NUM,
            String::from("42"),
            Loc { begin: 0, end: 2 },
        ),
        (
            DummyLexer::PLUS,
            String::from("+"),
            Loc { begin: 2, end: 3 },
        ),
        (
            DummyLexer::NUM,
            String::from("17"),
            Loc { begin: 3, end: 5 },
        ),
        (
            DummyLexer::EOL,
            String::from("\n"),
            Loc { begin: 5, end: 6 },
        ),
        (
            DummyLexer::YYEOF,
            String::from(""),
            Loc { begin: 6, end: 6 },
        ),
    ];

    let lexer = DummyLexer::new(tokens);
    let parser = Parser::new(Box::new(lexer));

    println!("{:#?}", parser.do_parse())
}
