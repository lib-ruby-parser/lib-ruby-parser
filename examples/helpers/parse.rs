use ruby_parser::{Lexer, Node, Parser};

#[allow(dead_code)]
pub fn parse(source: &Vec<u8>, filename: &str, debug: bool) -> Result<Node, String> {
    print!("parsing {} ... ", filename);
    let lexer = Lexer::new(source, filename, None).map_err(|e| e.to_string())?;
    let mut parser = Parser::new_with_lexer(lexer);
    parser.set_debug(debug);

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
