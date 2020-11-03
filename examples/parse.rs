extern crate clap;
use clap::Clap;

use ruby_parser::{Diagnostic, ParserResult};
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

    #[clap(short = 'L', long, about = "print locations")]
    locations: bool,

    #[clap(long, about = "print full AST using debug formatter")]
    print_full: bool,
}

fn print_diagnostics(diagnostics: &[Diagnostic]) {
    for d in diagnostics {
        println!("{}", d.render().expect("Failed to render a diagnostic"))
    }
}

fn print_quite(_src: &str, result: &ParserResult) {
    print_diagnostics(&result.diagnostics);
}

fn print_locations(src: &str, result: &ParserResult) {
    println!("{}", src);
    print_diagnostics(&result.diagnostics);
    if let Some(ast) = &result.ast {
        ast.print_with_locs()
    }
}
fn print_ast(_src: &str, result: &ParserResult) {
    print_diagnostics(&result.diagnostics);
    if let Some(ast) = &result.ast {
        println!("{}", ast.inspect(0));
    }
}
fn print_full(_str: &str, result: &ParserResult) {
    println!("{:#?}", result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();
    let callback: &dyn Fn(&str, &ParserResult) = if args.quiet {
        &print_quite
    } else if args.locations {
        &print_locations
    } else if args.print_full {
        &print_full
    } else {
        &print_ast
    };
    let debug = args.debug;

    if let Some(code) = args.code {
        let result = parse(code.as_bytes(), "(eval)", debug)?;
        callback(&code, &result)
    } else if let Some(path) = args.path {
        let path = Path::new(&path);
        each_ruby_file(path, &|entry| {
            let code = fs::read(Path::new(entry))?;
            let result = parse(&code, entry, debug)?;
            callback(&String::from_utf8_lossy(&code), &result);
            Ok(())
        })?;
    } else {
        println!("Nothing to parse");
    }

    println!("Done");

    Ok(())
}
