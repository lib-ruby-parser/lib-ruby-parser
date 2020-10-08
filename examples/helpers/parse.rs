use ruby_parser::{Parser, Lexer, Node};

#[allow(dead_code)]
pub fn parse(source: &Vec<u8>, filename: &str, debug: bool) -> Result<Node, ()> {
    print!("parsing {} ... ", filename);
    let mut lexer = Lexer::new(&source, None).unwrap();
    lexer.buffer.name = filename.to_owned();
    let mut parser = Parser::new(lexer);
    parser.set_debug(debug);

    match parser.do_parse() {
        Some(node) => {
            println!("OK");
            Ok(node)
        },
        None => {
            println!("Error");
            Err(())
        }
    }
}
