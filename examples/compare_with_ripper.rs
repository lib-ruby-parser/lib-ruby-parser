use ruby_parser::Lexer;
use std::env;
use std::fs;
use std::path::Path;

mod helpers;
use helpers::*;

fn print_usage() -> ! {
    println!(
        "
USAGE:
    cargo run --example compare_with_ripper -- test.rb
"
    );
    std::process::exit(1)
}

fn lex_as_ripper(filepath: &str) -> Result<String, String> {
    let source = fs::read(filepath).map_err(|_| "failed to read a file".to_owned())?;
    let (parser, tokens) = lex(&source, filepath, false)?;

    let mut output = String::from("");
    for token in tokens {
        if token.token_type == Lexer::END_OF_INPUT {
            continue;
        }
        if token.token_type == Lexer::tNL {
            continue;
        }
        let token_name = Lexer::token_name(&token);
        let token_name = match &token_name[..] {
            "tLPAREN2" => "tLPAREN",
            "tLCURLY" => "tLBRACE",
            "tRCURLY" => "tRBRACE",
            "tLBRACK2" => "tLBRACK",
            "kDO_BLOCK" => "kDO",
            "kDO_COND" => "kDO",
            "kIF_MOD" => "kIF",
            "kUNLESS_MOD" => "kUNLESS",
            "kWHILE_MOD" => "kWHILE",
            "tUMINUS_NUM" => "tMINUS",
            "tXSTRING_BEG" => "tSTRING_BEG",
            other => other,
        }
        .to_owned();

        let bytes = token.to_bytes();
        let (line, col) = parser
            .yylexer
            .buffer
            .line_col_for_pos(token.loc.begin)
            .ok_or_else(|| format!("token {:#?} has invalid range", token))?;

        output.push_str(&format!("{} {:?} {}:{}\n", token_name, bytes, line, col))
    }
    Ok(output)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().skip(1).map(|e| &e[..]).collect();

    let path = match args[..] {
        [path] => path,
        _ => print_usage(),
    };

    each_ruby_file(
        Path::new(path),
        &|path| match (ripper_lex(path), lex_as_ripper(path)) {
            (Ok(ripper_out), Ok(out)) => {
                if ripper_out == out {
                    println!("OK")
                } else {
                    for (lineno, (ripper_line, line)) in
                        ripper_out.lines().zip(out.lines()).enumerate()
                    {
                        if ripper_line != line {
                            println!(
                                "file {}, line {}:\nripper: {}\nresult: {}",
                                path, lineno, ripper_line, line
                            );
                            std::process::exit(1)
                        }
                    }
                }
            }

            (Err(err), _) => println!("Given file can't be parsed by ripper: {}", err),

            (Ok(_), Err(err)) => println!("Given file is valid, but can't be parsed: {}", err),
        },
    )
    .unwrap();
}
