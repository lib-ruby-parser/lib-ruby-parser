use super::helpers::*;

extern crate clap;
use clap::Parser;
use lib_ruby_parser::{DiagnosticMessage, ParserResult};

use std::process::Command;

#[derive(Debug, Parser)]
struct Args {
    #[clap(help = "file/dir to parse")]
    pattern: Option<String>,
}

const RED: &str = "\x1b[0;31m";
const GREY: &str = "\x1b[0;37m";
const RESET: &str = "\x1b[0m";

enum Output {
    Ok,
    Skip { reason: String },
    Err(String),
}

fn compare(file: InputFile) -> Output {
    let filepath = file.filepath.clone();

    match std::fs::read_to_string(&filepath) {
        Ok(contents) => {
            if contents.chars().all(|c| c.is_ascii()) {
                // ok, otherwise there's a risk that MRI > 3.0.0.alpha
                // emits multibyte UTF chars as separate tSTRING_CONTENT chunks
            } else {
                return Output::Skip {
                    reason: "has multibyte UTF-8 chars".to_string(),
                };
            }
        }
        Err(_) => {
            return Output::Skip {
                reason: "has invalid utf-8 source".to_string(),
            };
        }
    }

    let expected = match Command::new("./mri-tokenizer").arg(&filepath).output() {
        Ok(output) => output,
        Err(err) => return Output::Err(format!("Failed to run ./mri-tokenizer: {}", err)),
    };
    if expected.status.code() != Some(0) {
        // invalid file, even popular gems have them
        return Output::Skip {
            reason: "contains invalid Ruby code".to_string(),
        };
    }

    let expected = match String::from_utf8(expected.stdout) {
        Ok(s) => s,
        Err(err) => {
            return Output::Err(format!(
                "failed to convert mri-tokenizer output to UTF-8: {}",
                err
            ));
        }
    };
    let expected = expected.lines().collect::<Vec<_>>();

    let ParserResult {
        tokens,
        diagnostics,
        ..
    } = parse(file, false);

    for diagnostic in diagnostics {
        if let DiagnosticMessage::EncodingError { error } = diagnostic.message {
            // non-utf-8 encoding comment
            return Output::Skip {
                reason: format!("has non-utf-8 magic comment ({})", error),
            };
        }
    }

    println!("Checking {}", filepath);

    for (expected_name, tok) in expected.iter().zip(tokens.iter()) {
        if *expected_name != tok.token_name() {
            return Output::Err(format!(
                "token mismatch expected = {}, actual = {}, token = {:?}",
                expected_name,
                tok.token_name(),
                tok,
            ));
        }
    }

    if expected.len() != tokens.len() {
        return Output::Err(format!(
            "tokens length mispatch: expected = {}, actual = {}",
            expected.len(),
            tokens.len(),
        ));
    }

    Output::Ok
}

#[allow(dead_code)]
pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let files = args
        .pattern
        .map(|pattern| InputFiles::new_pattern(&pattern))
        .unwrap_or_else(InputFiles::empty);

    let mut warnings = vec![];
    let mut errors = vec![];

    for file in files.into_iter() {
        let filepath = file.filepath.clone();

        match compare(file) {
            Output::Ok => {}
            Output::Skip { reason } => {
                let warning = format!("{}File {}: {}, skipping.{}", GREY, filepath, reason, RESET);
                println!("{}", warning);
                warnings.push(warning);
            }
            Output::Err(error) => {
                let error = format!("{}File {}: {}{}", RED, filepath, error, RESET);
                println!("{}", error);
                errors.push(error);
            }
        }
    }

    if !warnings.is_empty() {
        println!(
            "\n\n{}WARNINGS:\n\n{}\n\n{}",
            GREY,
            warnings.join("\n"),
            RESET
        );
    }

    if errors.is_empty() {
        Ok(())
    } else {
        println!("\n\n{}ERRORS:\n\n{}\n\n{}", RED, errors.join("\n"), RESET);
        std::process::exit(1);
    }
}
