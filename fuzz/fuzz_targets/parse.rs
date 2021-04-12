#![no_main]
use libfuzzer_sys::fuzz_target;

extern crate lib_ruby_parser;
use lib_ruby_parser::{debug_level, Parser, ParserOptions};

fuzz_target!(|source: &[u8]| {
    let options = ParserOptions {
        buffer_name: String::from("(eval)"),
        debug: debug_level::NONE,
        record_tokens: false,
        ..Default::default()
    };
    Parser::new(source, options).do_parse();
});
