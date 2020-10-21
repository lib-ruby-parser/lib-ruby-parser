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
    let (parser, mut tokens) = lex(&source, filepath, false)?;
    tokens.sort_by(|a, b| a.loc.begin.cmp(&b.loc.begin));

    let mut output = String::from("");
    for token in tokens {
        if token.token_type == Lexer::END_OF_INPUT {
            continue;
        }
        let token_name = Lexer::token_name(&token);
        let token_name = match &token_name[..] {
            "tNL" | "tSPACE" | "tSP" => continue,
            "tLPAREN2" => "tLPAREN",
            "tLCURLY" => "tLBRACE",
            "tRCURLY" => "tRBRACE",
            "tLBRACK2" => "tLBRACK",
            "kDO_BLOCK" => "kDO",
            "kDO_COND" => "kDO",
            "kDO_LAMBDA" => "kDO",
            "kIF_MOD" => "kIF",
            "kUNLESS_MOD" => "kUNLESS",
            "kWHILE_MOD" => "kWHILE",
            "kRESCUE_MOD" => "kRESCUE",
            "kUNTIL_MOD" => "kUNTIL",
            "tUMINUS_NUM" => "tMINUS",
            "tFID" => "tIDENTIFIER",
            "tAMPER2" => "tAMPER",
            "tSTAR2" => "tSTAR",
            "tPOW" => "tDSTAR",
            "tUMINUS" => "tMINUS",
            "tCOLON3" => "tCOLON2",
            "tNTH_REF" => "tBACK_REF",
            "tLPAREN_ARG" => "tLPAREN",
            "tLBRACE_ARG" => "tLBRACE",
            "tUPLUS" => "tPLUS",
            other => other,
        }
        .to_owned();

        let bytes = token.to_bytes();
        let (line, col) = parser
            .yylexer
            .buffer
            .line_col_for_pos(token.loc.begin)
            .ok_or_else(|| format!("token {:#?} has invalid range", token))?;

        output.push_str(&format!("{} {:?} {}:{}\n", token_name, bytes, line, col));
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

    each_ruby_file(Path::new(path), &|path| {
        print!("{}  ", path);
        match (ripper_lex(path), lex_as_ripper(path)) {
            (Ok(ripper_out), Ok(out)) => {
                for (lineno, (ripper_line, line)) in ripper_out.lines().zip(out.lines()).enumerate()
                {
                    if ripper_line == "<<UNKNOWN>>" {
                        // Part of the regex with interpolation
                        // that can't be dumped
                    } else if ripper_line != line {
                        println!(
                            "file {}, line {}:\nripper: {}\nresult: {}",
                            path, lineno, ripper_line, line
                        );
                        // std::process::exit(1)
                    }
                }
                println!("OK")
            }

            (Err(err), _) => {
                println!("Given file can't be parsed by ripper: {}", err);
                // std::process::exit(1);
            }

            (Ok(_), Err(err)) => {
                println!("Given file is valid, but can't be parsed: {}", err);
                // std::process::exit(1);
            }
        };
    })
    .unwrap();
}
