extern crate clap;
use clap::Clap;

use ruby_parser::{Lexer, Token};
use std::fs;
use std::path::Path;

mod helpers;
use helpers::*;

#[derive(Debug, Clap)]
struct Args {
    #[clap(about = "file/dir to parse")]
    path: Option<String>,

    #[clap(short = 'e', about = "code to evaluate")]
    code: Option<String>,

    #[clap(short, long, about = "don't print anything except OK/Error per file")]
    quiet: bool,

    #[clap(short, long, about = "print debug information")]
    debug: bool,
}

fn token_name(token: &Token) -> String {
    Lexer::token_name(token.token_type)
}

fn token_value(token: &Token) -> String {
    token.to_string_lossy()
}

fn rpad<T: Sized + std::fmt::Debug>(value: &T, total_width: usize) -> String {
    format!("{:width$}", format!("{:?}, ", value), width = total_width)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();
    let callback: &dyn Fn(&Vec<Token>) = if args.quiet {
        &|_tokens: &Vec<Token>| {}
    } else {
        &|tokens: &Vec<Token>| {
            let tok_name_length = tokens
                .iter()
                .map(|tok| format!("{:?}", token_name(tok)).len())
                .max()
                .unwrap_or(0)
                + 2;
            let tok_value_length = tokens
                .iter()
                .map(|tok| format!("{:?}", token_value(tok)).len())
                .max()
                .unwrap_or(0)
                + 2;

            println!("[");
            for token in tokens {
                let name = rpad(&token_name(&token), tok_name_length);
                let value = rpad(&token_value(&token), tok_value_length);
                println!(
                    "    :{}{}[{}, {}]",
                    name, value, token.loc.begin, token.loc.end
                );
            }
            println!("]");
        }
    };
    let debug = args.debug;

    if let Some(code) = args.code {
        let tokens = tokenize(code.as_bytes(), "(eval)", debug)?;
        callback(&tokens)
    } else if let Some(path) = args.path {
        let path = Path::new(&path);
        each_ruby_file(path, &|entry| {
            let code = fs::read(Path::new(entry))?;
            let node = tokenize(&code, entry, debug)?;
            callback(&node);
            Ok(())
        })?;
    }

    return Ok(());
}
