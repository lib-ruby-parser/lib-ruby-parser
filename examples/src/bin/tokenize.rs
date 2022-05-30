extern crate clap;
use clap::Parser;

use examples::helpers::*;

#[derive(Debug, Parser)]
struct Args {
    #[clap(help = "file/dir to parse")]
    pattern: Option<String>,

    #[clap(short = 'e', help = "code to evaluate")]
    code_to_eval: Option<String>,

    #[clap(short, long, help = "don't print anything except OK/Error per file")]
    quiet: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();

    fn print_nothing(_: TokenList) {}

    fn print_all(tokens: TokenList) {
        println!("{:?}", tokens)
    }
    let callback = if args.quiet { print_nothing } else { print_all };

    let files = InputFiles::new(&args.code_to_eval, &args.pattern, &None);

    for file in files.into_iter() {
        let tokens = tokenize(file)?;
        println!("OK");
        callback(TokenList { tokens });
    }

    Ok(())
}
