extern crate clap;
use clap::Clap;

extern crate pprof;

use ruby_parser::Node;
use std::fs;
use std::path::Path;

mod helpers;
use helpers::*;

use std::fs::File;

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
}

fn print_quite(_src: &str, _node: &Node) {}

fn print_locations(src: &str, node: &Node) {
    print_all_locs(src, node);
}
fn print_ast(_src: &str, node: &Node) {
    println!("{}", node.inspect(0));
}

fn main() -> Result<(), ()> {
    let guard = pprof::ProfilerGuard::new(100).unwrap();

    let args: Args = Args::parse();
    let callback: &dyn Fn(&str, &Node) = if args.quiet {
        &print_quite
    } else if args.locations {
        &print_locations
    } else {
        &print_ast
    };
    let debug = args.debug;

    if let Some(code) = args.code {
        let node = parse(&code.to_owned().into_bytes(), "(eval)", debug)?;
        callback(&code, &node)
    } else if let Some(path) = args.path {
        for _ in 1..20 {
            let path = Path::new(&path);
            each_ruby_file(path, &|entry| {
                let code = fs::read(Path::new(entry)).unwrap();
                let node = parse(&code, entry, debug)
                    .unwrap_or_else(|_| panic!("failed to parse {}", entry));
                callback(&String::from_utf8_lossy(&code), &node)
            })
            .unwrap_or_else(|e| panic!("Error {:?}", e));
        }
    } else {
        println!("Nothing to parse");
    }

    if let Ok(report) = guard.report().build() {
        let file = File::create("flamegraph.svg").unwrap();
        report.flamegraph(file).unwrap();
    };

    println!("DOne");

    Ok(())
}
