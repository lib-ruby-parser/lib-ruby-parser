mod helpers;

use helpers::{parse, print_build_info, InputToParse, Printer, Profiler, Repeater, Timer};
use lib_ruby_parser_ast::Blob;

#[cfg(not(windows))]
#[cfg(feature = "jemallocator")]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

const HELP: &str = "
Parse Ruby code using lib-ruby-parser

USAGE:
    parse [OPTIONS]

OPTIONS:

    -e  <code>                    Code to parse
    --glob <glob>                 File/dir to parse, supports glob patterns
    --print <printer>             Print information about the parsed code
                                  N = Nothing
                                  F = Full AST
                                  L = Compact AST with locations
                                  D = Only Diagnostics
                                  default = Compact AST
    --run-profiler                Run profiling
    --drop-tokens                 Drop tokens info
    --run-timer                   Measure time spent on benchmarking
    --print-build-info            Prints information about executable
    --repeat <n>                  Repeat parsing N times
";

fn print_help_and_exit() -> ! {
    eprintln!("{}", HELP);
    std::process::exit(1);
}

#[derive(Debug)]
struct Args {
    input_to_parse: InputToParse,
    printer: Printer,
    profiler: Profiler,
    drop_tokens: bool,
    timer: Timer,
    repeater: Repeater,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut input_to_parse = None;
    let mut printer = Printer::default();
    let mut profiler = Profiler::disabled();
    let mut drop_tokens = false;
    let mut timer = Timer::default();
    let mut repeater = Repeater::default();

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Long("glob") => input_to_parse = Some(InputToParse::Glob(parser.value()?)),

            Short('e') => input_to_parse = Some(InputToParse::Eval(parser.value()?)),

            Long("print") => printer = Printer::from(parser.value()?),

            Long("run-profiler") => profiler = Profiler::enabled(),

            Long("drop-tokens") => drop_tokens = true,

            Long("run-timer") => timer = Timer::enabled(),

            Long("print-build-info") => print_build_info(),

            Long("repeat") => repeater = Repeater::from(parser.value()?),

            _ => return Err(arg.unexpected()),
        }
    }

    let input_to_parse = input_to_parse.ok_or_else(|| {
        eprintln!("You must provide either --glob or -e");
        lexopt::Error::MissingValue {
            option: Some(String::from("--glob or -e is required")),
        }
    })?;

    Ok(Args {
        input_to_parse,
        printer,
        profiler,
        drop_tokens,
        timer,
        repeater,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args {
        input_to_parse,
        printer,
        mut profiler,
        drop_tokens,
        mut timer,
        repeater,
    } = parse_args().unwrap_or_else(|err| {
        eprintln!("{}", err);
        print_help_and_exit();
    });

    let mut files = input_to_parse.into_files();
    repeater.repeat(&mut files);
    let files_count = files.len();

    profiler.start();
    timer.start();

    let mut mem = vec![0; 20_000_000];

    for file in files.iter() {
        let blob = Blob::from(mem.as_mut_slice());
        let result = parse(&file, &blob, drop_tokens);
        printer.print(&result);
    }

    timer.stop(files_count);
    profiler.stop()?;

    Ok(())
}
