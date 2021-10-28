use super::helpers::*;

extern crate clap;
use clap::Parser;

use std::process::Command;

#[derive(Debug, Parser)]
struct Args {
    #[clap(about = "file/dir to parse")]
    pattern: Option<String>,
}

fn compare(file: InputFile) -> Result<(), Box<dyn std::error::Error>> {
    println!("Checking {}", file.filepath);

    let expected = Command::new("./mri-tokenizer")
        .arg(&file.filepath)
        .output()?;
    let expected = String::from_utf8(expected.stdout)?;
    let expected = expected.lines().collect::<Vec<_>>();

    let tokens = parse(file, false).tokens;

    for (expected_name, tok) in expected.iter().zip(tokens.iter()) {
        assert_eq!(
            expected_name,
            &tok.token_name(),
            "token mismatch expected = {}, actual = {}, token = {:?}",
            expected_name,
            tok.token_name(),
            tok
        );
        // println!("{} == {:?}", expected_name, tok);
    }

    assert_eq!(expected.len(), tokens.len());

    Ok(())
}

#[allow(dead_code)]
pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let files = args
        .pattern
        .map(|pattern| InputFiles::new_pattern(&pattern))
        .unwrap_or_else(|| InputFiles::empty());

    for file in files.into_iter() {
        compare(file)?;
    }
    Ok(())
}
