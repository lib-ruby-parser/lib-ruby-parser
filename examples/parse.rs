#[allow(dead_code)]
mod helpers;
#[cfg(feature = "run-examples")]
use helpers::*;

#[cfg(feature = "run-examples")]
extern crate clap;
#[cfg(feature = "run-examples")]
use clap::Clap;

#[cfg(feature = "run-examples")]
extern crate jemallocator;
#[cfg(feature = "run-examples")]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(feature = "run-examples")]
#[derive(Debug, Clap)]
pub struct Args {
    #[clap(about = "file/dir to parse")]
    pub pattern: Option<String>,

    #[clap(short = 'e', about = "code to evaluate")]
    pub code_to_eval: Option<String>,

    #[clap(long = "print", about = Printer::ABOUT)]
    pub printer: Option<Printer>,

    #[clap(long, about = DebugLevel::ABOUT)]
    pub debug_level: Option<DebugLevel>,

    #[clap(long = "run-profiler", about = "Run profiling")]
    pub profiler: Option<Profiler>,

    #[clap(long, about = "Drop tokens info")]
    pub drop_tokens: bool,

    #[clap(long = "run-timer", about = "Measure time spent on benchmarking")]
    pub timer: Option<Timer>,

    #[clap(long, about = "Prints information about executable")]
    pub print_build_info: bool,

    #[clap(long, about = "Repeat parsing N times")]
    pub repeat: Option<usize>,
}

#[cfg(feature = "run-examples")]
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

#[cfg(feature = "run-examples")]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
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

#[cfg(not(feature = "run-examples"))]
fn main() {
    println!("'parse' example must be executed with 'run-examples' feature")
}
