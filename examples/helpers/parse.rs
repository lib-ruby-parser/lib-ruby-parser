use ruby_parser::{Parser, ParserOptions, ParserResult};

#[allow(dead_code)]
pub fn parse(source: &Vec<u8>, filename: &str, debug: bool) -> Result<ParserResult, String> {
    println!("parsing {} ... ", filename);
    let options = ParserOptions {
        buffer_name: filename,
        debug,
        ..Default::default()
    };
    let mut parser = Parser::new(source, options).map_err(|e| e.to_string())?;

    Ok(parser.do_parse())
}