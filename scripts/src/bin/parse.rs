use scripts::helpers::*;

extern crate clap;
use clap::Parser;

#[cfg(not(windows))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[derive(Debug, Parser)]
struct Args {
    #[clap(help = "file/dir to parse")]
    pattern: Option<String>,

    #[clap(short = 'e', help = "code to evaluate")]
    code_to_eval: Option<String>,

    #[clap(long = "print", help = Printer::ABOUT)]
    printer: Option<Printer>,

    #[clap(long = "run-profiler", help = "Run profiling")]
    profiler: Option<Profiler>,

    #[clap(long, help = "Drop tokens info")]
    drop_tokens: bool,

    #[clap(long = "run-timer", help = "Measure time spent on benchmarking")]
    timer: Option<Timer>,

    #[clap(long, help = "Prints information about executable")]
    print_build_info: bool,

    #[clap(long, help = "Repeat parsing N times")]
    repeat: Option<usize>,
}

impl From<&Args> for Option<InputFiles> {
    fn from(args: &Args) -> Self {
        if let Some(code_to_eval) = &args.code_to_eval {
            Some(InputFiles::new_eval(code_to_eval.clone().into_bytes()))
        } else {
            args.pattern
                .as_ref()
                .map(|pattern| InputFiles::new_pattern(pattern))
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.print_build_info {
        BuildInfo::print()
    }

    let printer = args.printer.unwrap_or_default();

    let files = InputFiles::new(&args.code_to_eval, &args.pattern, &args.repeat);
    let files_count = files.len();

    let mut profiler = args.profiler.unwrap_or_default();
    let mut timer = args.timer.unwrap_or_default();

    profiler.start();
    timer.start();

    for file in files.into_iter() {
        let result = parse(file, args.drop_tokens);
        printer.print(&result);
    }

    timer.stop(files_count);
    profiler.stop()?;

    Ok(())
}

