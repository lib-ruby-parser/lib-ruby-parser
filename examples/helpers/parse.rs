use ruby_parser::{Node, Parser};

#[allow(dead_code)]
pub fn parse(source: &Vec<u8>, filename: &str, debug: bool) -> Result<Node, ()> {
    print!("parsing {} ... ", filename);
    let mut parser = Parser::new(&source).unwrap();
    parser.yylexer.buffer.name = filename.to_owned();
    parser.set_debug(debug);

    match parser.do_parse() {
        Some(node) => {
            println!("OK");
            Ok(node)
        }
        None => {
            println!("Error");
            Err(())
        }
    }
}
