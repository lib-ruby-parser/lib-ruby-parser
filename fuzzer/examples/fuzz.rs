#![no_main]
use libfuzzer_sys::fuzz_target;

use lib_ruby_parser::{Parser, ParserOptions};

fuzz_target!(|source: &[u8]| {
    let options = ParserOptions {
        buffer_name: String::from("(eval)"),
        record_tokens: false,
        ..Default::default()
    };
    Parser::new(source, options).do_parse();
});
