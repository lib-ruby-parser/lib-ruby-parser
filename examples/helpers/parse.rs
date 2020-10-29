use ruby_parser::{Node, Parser, ParserOptions};

#[allow(dead_code)]
pub fn parse(source: &Vec<u8>, filename: &str, debug: bool) -> Result<Node, String> {
    print!("parsing {} ... ", filename);
    let options = ParserOptions {
        buffer_name: filename,
        debug,
        ..Default::default()
    };
    let mut parser = Parser::new(source, options).map_err(|e| e.to_string())?;

    match parser.do_parse() {
        Some(node) => {
            println!("OK");
            Ok(node)
        }
        None => {
            println!("Error");
            Err("Got no tokens".to_owned())
        }
    }
}
