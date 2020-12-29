extern crate clap;
use clap::Clap;

extern crate jemallocator;
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use lib_ruby_parser::ParserResult;
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

    #[clap(short, long, about = "don't print anything except diagnostics")]
    quiet: bool,

    #[clap(long, about = "don't print anything")]
    no_output: bool,

    #[clap(short, long, about = "print debug information")]
    debug: bool,

    #[clap(short = 'L', long, about = "print locations")]
    locations: bool,

    #[clap(short = 'F', long, about = "print full AST using debug formatter")]
    print_full: bool,

    #[clap(short, long, about = "Run profiles")]
    profile: bool,

    #[clap(long, about = "Drop tokens info")]
    drop_tokens: bool,
}

fn print_diagnostics(result: &ParserResult) {
    for d in result.diagnostics.iter() {
        println!(
            "{}",
            d.render(&result.input)
                .expect("Failed to render a diagnostic")
        )
    }
}

fn print_quite(_src: &str, result: &ParserResult) {
    print_diagnostics(&result);
}

fn no_output(_: &str, _: &ParserResult) {}

fn print_locations(src: &str, result: &ParserResult) {
    println!("{}", src);
    print_diagnostics(&result);
    if let Some(ast) = &result.ast {
        ast.print_with_locs()
    }
}
fn print_ast(_src: &str, result: &ParserResult) {
    print_diagnostics(&result);
    if let Some(ast) = &result.ast {
        println!("{}", ast.inspect(0));
    }
}
fn print_full(_str: &str, result: &ParserResult) {
    println!("{:#?}", result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();

    let callback: &dyn Fn(&str, &ParserResult) = if args.no_output {
        &no_output
    } else if args.quiet {
        &print_quite
    } else if args.locations {
        &print_locations
    } else if args.print_full {
        &print_full
    } else {
        &print_ast
    };

    let profile = start_profiling(args.profile);
    let debug = args.debug;
    let drop_tokens = args.drop_tokens;

    if let Some(code) = args.code {
        let result = parse(code.as_bytes(), "(eval)", debug, drop_tokens);
        callback(&code, &result);
    } else if let Some(path) = args.path {
        each_ruby_file(&path, &|entry| {
            let code = fs::read(Path::new(entry))?;
            let result = parse(&code, entry, debug, drop_tokens);
            callback(&String::from_utf8_lossy(&code), &result);
            Ok(())
        })?;
    } else {
        println!("Nothing to parse");
    }

    stop_profiling(args.profile, profile)?;

    println!("Done");

    Ok(())
}
