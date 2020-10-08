extern crate clap;
use clap::Clap;

use ruby_parser::Node;
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

fn main() -> Result<(), ()> {
    let args: Args = Args::parse();
    let callback: &dyn Fn(&Node) = if args.quiet {
        &|_node: &Node| {}
    } else {
        &|node: &Node| println!("{}", node.inspect(0))
    };
    let debug = args.debug;

    if let Some(code) = args.code {
        let node = parse(
            &code.to_owned().into_bytes(),
            "(eval)",
            debug
        )?;
        callback(&node)
    } else if let Some(path) = args.path {
        let path = Path::new(&path);
        each_ruby_file(path, &|entry| {
            let code = fs::read(Path::new(entry)).unwrap();
            let node = parse(
                &code,
                entry,
                debug
            ).unwrap_or_else(|_| panic!("failed to parse {}", entry));
            callback(&node)
        }).unwrap_or_else(|e| panic!("Error {:?}", e));
    } else {
        println!("Nothing to parse");
        return Err(())
    }

    Ok(())
}
