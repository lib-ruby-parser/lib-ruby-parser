extern crate clap;
use clap::Clap;

extern crate jemallocator;
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use lib_ruby_parser::ParserResult;
use lib_ruby_parser_helpers::*;

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

    #[clap(
        short,
        long,
        about = "comma-separated list of debug levels (parser, lexer, buffer)"
    )]
    debug: Option<String>,

    #[clap(short = 'L', long, about = "print locations")]
    locations: bool,

    #[clap(short = 'F', long, about = "print full AST using debug formatter")]
    print_full: bool,

    #[clap(short, long, about = "Run profiles")]
    profile: bool,

    #[clap(long, about = "Drop tokens info")]
    drop_tokens: bool,

    #[clap(long, about = "Measure time spent on benchmarking")]
    benchmark: bool,

    #[clap(long, about = "Prints information about executable")]
    print_info: bool,
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

fn print_quite(result: &ParserResult) {
    print_diagnostics(&result);
}

fn print_nothing(_: &ParserResult) {}

fn print_locations(result: &ParserResult) {
    let src = result.input.as_shared_bytes();
    let src = std::str::from_utf8(src.as_ref()).unwrap_or_else(|_| "invalid-source");
    println!("{}", src);
    print_diagnostics(&result);
    if let Some(ast) = result.ast.as_ref() {
        ast.print_with_locs()
    }
}
fn print_ast(result: &ParserResult) {
    print_diagnostics(&result);
    if let Some(ast) = result.ast.as_ref() {
        println!("{}", ast.inspect(0));
    }
}
fn print_full(result: &ParserResult) {
    println!("{:#?}", result)
}

struct InputFile {
    filepath: String,
    content: Vec<u8>,
}

impl From<&Args> for Option<Vec<InputFile>> {
    fn from(args: &Args) -> Self {
        let files = if let Some(code) = &args.code {
            vec![InputFile {
                filepath: "(eval)".to_string(),
                content: code.as_bytes().to_vec(),
            }]
        } else if let Some(path) = &args.path {
            glob::glob(&path)
                .expect("invalid glob pattern")
                .map(|f| f.unwrap().to_str().unwrap().to_string())
                .map(|filepath| InputFile {
                    content: std::fs::read(&filepath).unwrap(),
                    filepath,
                })
                .collect()
        } else {
            return None;
        };

        Some(files)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();

    if args.print_info {
        if cfg!(feature = "onig") {
            println!("Using 'onig' feature")
        }
        if cfg!(feature = "compile-with-external-structures") {
            println!("Using 'compile-with-external-structures' feature")
        } else {
            println!("Using Rust structures")
        }
        std::process::exit(0);
    }

    let print_result: &dyn Fn(&ParserResult) = if args.no_output {
        &print_nothing
    } else if args.quiet {
        &print_quite
    } else if args.locations {
        &print_locations
    } else if args.print_full {
        &print_full
    } else {
        &print_ast
    };

    let debug = debug_level_from_string(&args.debug);

    let files = Option::<Vec<InputFile>>::from(&args).unwrap_or_else(|| {
        println!("Nothing to parse");
        std::process::exit(1);
    });

    let profile = profiling::start(args.profile);
    let benchmark = benchmarking::start(args.benchmark);

    for file in files.iter() {
        let result = parse(&file.content, &file.filepath, debug, args.drop_tokens);
        print_result(&result);
    }

    benchmarking::stop(args.benchmark, benchmark, files.len());
    profiling::stop(args.profile, profile)?;

    Ok(())
}
