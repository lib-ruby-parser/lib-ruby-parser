use std::env;
use std::path::Path;

mod helpers;
use helpers::*;

fn print_usage() -> ! {
    println!(
        "
USAGE:
    cargo run --example compare_with_ripper -- test.rb
    cargo run --example compare_with_ripper -- /path/to/dir/
"
    );
    std::process::exit(1)
}

fn compare(path: &str) -> Result<(), ()> {
    match (ripper_lex(path), lex_as_ripper(path)) {
        (Ok(ripper_out), Ok(out)) => {
            for (lineno, (ripper_line, line)) in ripper_out.lines().zip(out.lines()).enumerate() {
                if ripper_line == "<<UNKNOWN>>" {
                    // Part of the regex with interpolation
                    // that can't be dumped
                } else if ripper_line != line {
                    println!(
                        "file {}, line {}:\nripper: {}\nresult: {}",
                        path, lineno, ripper_line, line
                    );
                    return Err(());
                }
            }
            println!("{}  OK", path);
            Ok(())
        }

        (Err(err), _) => {
            println!("{}  Given file can't be parsed by ripper: {}", path, err);
            Ok(())
        }

        (Ok(_), Err(err)) => {
            println!(
                "{}  Given file is valid, but can't be parsed by us: {}",
                path, err
            );
            Err(())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().skip(1).map(|e| &e[..]).collect();

    let path = match args[..] {
        [path] => path,
        _ => print_usage(),
    };

    each_async_ruby_file(Path::new(path), &|path| match compare(&path) {
        Ok(_) => {}
        Err(_) => {}
    })?;

    Ok(())
}
