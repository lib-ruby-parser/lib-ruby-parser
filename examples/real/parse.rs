use super::helpers::*;

extern crate clap;
use clap::Clap;

extern crate jemallocator;
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[derive(Debug, Clap)]
struct Args {
    #[clap(about = "file/dir to parse")]
    pattern: Option<String>,

    #[clap(short = 'e', about = "code to evaluate")]
    code_to_eval: Option<String>,

    #[clap(long = "print", about = Printer::ABOUT)]
    printer: Option<Printer>,

    #[clap(long, about = DebugLevel::ABOUT)]
    debug_level: Option<DebugLevel>,

    #[clap(long = "run-profiler", about = "Run profiling")]
    profiler: Option<Profiler>,

    #[clap(long, about = "Drop tokens info")]
    drop_tokens: bool,

    #[clap(long = "run-timer", about = "Measure time spent on benchmarking")]
    timer: Option<Timer>,

    #[clap(long, about = "Prints information about executable")]
    print_build_info: bool,

    #[clap(long, about = "Repeat parsing N times")]
    repeat: Option<usize>,
}

impl From<&Args> for Option<InputFiles> {
    fn from(args: &Args) -> Self {
        if let Some(code_to_eval) = &args.code_to_eval {
            Some(InputFiles::new_eval(code_to_eval.clone().into_bytes()))
        } else if let Some(pattern) = &args.pattern {
            Some(InputFiles::new_pattern(&pattern))
        } else {
            None
        }
    }
}

#[allow(dead_code)]
pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.print_build_info {
        BuildInfo::print()
    }

    let printer = args.printer.unwrap_or_default();

    let debug_level = args.debug_level.unwrap_or_default();

    let files = InputFiles::new(&args.code_to_eval, &args.pattern, &args.repeat);
    let files_count = files.len();

    let mut profiler = args.profiler.unwrap_or_default();
    let mut timer = args.timer.unwrap_or_default();

    profiler.start();
    timer.start();

    for file in files.into_iter() {
        let result = parse(file, debug_level, args.drop_tokens);
        printer.print(&result);
    }

    timer.stop(files_count);
    profiler.stop()?;

    Ok(())
}
