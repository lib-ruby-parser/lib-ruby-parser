use ruby_parser::{Parser, Lexer};
use std::env;
use std::fs;
use std::process::Command;

fn print_usage() -> ! {
    println!("
USAGE:
    cargo run --example compare_with_ripper -- test.rb
");
    std::process::exit(1)
}

fn lex_as_ripper(filepath: &str) -> Result<String, String> {
    let out = Command::new("ruby")
            .args(&["examples/lex_ripper.rb", filepath])
            .output()
            .map_err(|_| "failed to execute process".to_owned() )?;

    if out.status.success() {
        String::from_utf8(out.stdout).map_err(|_| "non-utf8 output".to_owned())
    } else {
        println!("{}", String::from_utf8_lossy(&out.stderr).to_owned());
        Err("non-zero exit code".to_owned())
    }
}

fn lex(filepath: &str) -> Result<String, String> {
    let source = fs::read(filepath).map_err(|_| "failed to read a file".to_owned())?;
    let lexer = Lexer::new(&source, None).map_err(|e| e.to_string())?;
    let mut parser = Parser::new(lexer);
    parser.set_debug(false);

    match parser.lex() {
        Some(tokens) => {
            let mut output = String::from("");
            for token in tokens {
                if token.0 == Lexer::END_OF_INPUT { continue }
                let token_name = Lexer::token_name(&token);
                let token_name =
                    match &token_name[..] {
                        "tLPAREN2" => "tLPAREN",
                        "tLCURLY"  => "tLBRACE",
                        "tRCURLY"  => "tRBRACE",
                        other => other
                    }.to_owned();

                let bytes = token.1.to_bytes();
                let start = parser.yylexer.buffer.line_col_for_pos(token.2.begin);
                match start {
                    Some((line, col)) => {
                        output.push_str(
                            &format!("{} {:?} {}:{}\n", token_name, bytes, line, col)
                        )
                    },
                    None => {
                        return Err(format!("token {:#?} has invalid range", token))
                    }
                }
            }
            Ok(output)
        },
        None => Err("empty tokens".to_owned()),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().skip(1).map(|e| &e[..]).collect();

    let filepath =
        match args[..] {
            [filepath] => filepath,
            _ => print_usage()
        };

    let ripper_result = lex_as_ripper(filepath);
    let result = lex(filepath);

    match ripper_result {
        Ok(ripper_result) => {
            match result {
                Ok(result) => {
                    if ripper_result == result {
                        println!("OK")
                    } else {
                        for (lineno, (ripper_line, line)) in ripper_result.lines().zip(result.lines()).enumerate() {
                            if ripper_line != line {
                                println!("line {}:\nripper: {}\nresult: {}", lineno, ripper_line, line)
                            }
                        }
                        std::process::exit(1)
                    }
                },
                Err(err) => {
                    println!("Failed to parse: {}", err)
                }
            }
        },
        Err(err) => {
            println!("Given file can't be parsed by ripper: {}", err)
        }
    }
}
