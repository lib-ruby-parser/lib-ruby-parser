use std::{
    fs::File,
    io::{Read, Write},
};

use lib_ruby_parser::{Node, Parser, ParserOptions, YYStackItem};
use lib_ruby_parser_ast::Blob;

const TMP_FILEPATH: &str = "/tmp/ten-plus-20.data";

fn main() {
    dump();

    let data = read();
    let node = load(&data);

    let mut buf = String::new();
    node.inspect(0, &mut buf).unwrap();
    println!("{}", buf);
}

fn dump() {
    let code = b"10 + 20";

    let mut mem = [0; 1000];
    let base_ptr = &mem as *const usize;
    let blob = Blob::from(&mut mem);

    let mut scratch = [0; 1000];
    let scratch = Blob::from(&mut scratch);

    let mut stack = [YYStackItem::none(); 20];

    let parser = Parser::new(code, ParserOptions::default(), &blob, &scratch);
    let result = parser.do_parse(&mut stack);

    let mut f = File::create(TMP_FILEPATH).unwrap();

    let offset = unsafe {
        let node_ptr = result.ast.unwrap() as *const Node;
        let offset = node_ptr.byte_offset_from(base_ptr);
        assert!(offset > 0);
        offset
    };
    f.write_all(&usize_to_bytes(offset as usize)).unwrap();
    f.write_all(blob.data()).unwrap();
}

fn read() -> Vec<u8> {
    let mut f = File::open(TMP_FILEPATH).unwrap();
    let mut data = vec![];
    f.read_to_end(&mut data).unwrap();
    data
}

fn load(data: &[u8]) -> &Node {
    let offset = bytes_to_usize(*data.first_chunk().unwrap());
    let data = &data[8..];

    unsafe { data.as_ptr().add(offset).cast::<Node>().as_ref().unwrap() }
}

fn usize_to_bytes(n: usize) -> [u8; 8] {
    n.to_ne_bytes()
}
fn bytes_to_usize(bytes: [u8; 8]) -> usize {
    unsafe { core::mem::transmute(bytes) }
}
