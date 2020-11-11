# lib-ruby-parser

[![test](https://github.com/lib-ruby-parser/lib-ruby-parser/workflows/test/badge.svg?branch=master)](https://github.com/lib-ruby-parser/lib-ruby-parser/actions?query=workflow%3Atest)
[![codecov](https://codecov.io/gh/lib-ruby-parser/lib-ruby-parser/branch/master/graph/badge.svg)](https://codecov.io/gh/lib-ruby-parser/lib-ruby-parser)
[![MIT Licence](https://badges.frapsoft.com/os/mit/mit.svg?v=103)](https://opensource.org/licenses/mit-license.php)


`lib-ruby-parser` is a Ruby parser written in Rust.

Basic usage:

```rust
use lib_ruby_parser::{Parser, ParserOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = ParserOptions {
        buffer_name: "(eval)".to_owned(),
        debug: true,
        ..Default::default()
    };
    let mut parser = Parser::new(b"2 + 2", options)?;

    println!("{:#?}", parser.do_parse());

    Ok(())
}
```

## Features

TLDR; it's fast, it's precise, and it has a beautiful interface.

Comparison with `Ripper`/`RubyVM::AST`:
1. It's based on MRI's `parse.y`, and so it returns **exactly** the same sequence of tokens.
2. It's been tested on top 300 gems (by total downlads, that's about 3M LOC), `rubyspec` and `ruby/ruby` repos and there's no difference with `Ripper.lex`.
3. It's twice slower than `Ripper`, it takes ~30s to parse 300M LOC, MRI does it in 15s. Still, that's 100K LOC/s.
4. It has a much, much better interface. AST is strongly types and well documented.
5. It doesn't throw away information about tokens. All nodes have information about their source locations.

Comparison with [whitequark/parser](https://github.com/whitequark/parser):
1. It's much faster (the same corpus of 3M LOC can be parsed in 180s on the same machine)
1. It has a very similar interface (both in terms of AST structure and errors reporting)
3. However, AST is strongly typed, and so if something is nullable it's explicitly defined and documented.
4. What's important, it doesn't depend on Ruby

## Versioning

`lib-ruby-parser` follows MRI/master. There are no plans to support multiple versions like it's done in `whitequark/parser`.

## Encodings

By default `lib-ruby-parser` can only parse source files encoded in `UTF-8` or `ASCII-8BIT/BINARY`.

It's possible to pass a `decoder` function in `ParserOptions` that takes a recognized (by the library) encoding and a byte array. It must return a UTF-8 encoded byte array or an error:

```rust
use lib_ruby_parser::source::{InputError, RecognizedEncoding};

fn decoder(encoding: RecognizedEncoding, input: Vec<u8>) -> Result<Vec<u8>, InputError> {
    if let RecognizedEncoding::US_ASCII = encoding {
        // reencode and return Ok(result)
        return Ok(b"2 + 2".to_vec());
    }
    Err(InputError::DecodingError(
        "only us-ascii is supported".to_owned(),
    ))
}

fn parse(input: Vec<u8>) -> Result<(), Box<dyn std::error::Error>>{
    let options = ParserOptions {
        decoder: Some(Box::new(decoder)),
        ..Default::default()
    };

    let result = Parser::new(b"3 + 3", options)?.do_parse();
    println!("{:#?}", result);
    // prints AST for "2 + 2"
}
```

## Invalid string values

Ruby doesn't require string literals to be valid in their encodings. This is why the following code is valid:

```ruby
# encoding: utf-8

"\xFF"
```

Byte sequence `255` is invalid in UTF-8, but MRI ignores it.

But not all languages support it, and this is why string and symbol nodes encapsulate a custom `StringValue` instead of a plain `String`.

If your langauge supports invalid strings you can use raw `.bytes` of this `StringValue`. For example, a Ruby wrapper for this library could do that.

If your language doesn't support it, better call `.to_string_lossy()` that replaces all unsupported chars with a special `U+FFFD REPLACEMENT CHARACTER (�)`.

## Regexes

Ruby constructs regexes from literals during parsing to:
1. validate them
2. declare local variables if regex is used for matching AND it contains named captures

To mirror this behavior `lib-ruby-parser` uses Onigurama to compile, validate and parse regex literals.

This feature is disabled by default, but you can enable it by enabling `"onig"` feature.

## Bison

The grammar of `lib-ruby-parser` is built using a [custom bison skeleton](https://github.com/iliabylich/rust-bison-skeleton) that was written for this project.

For development you need the latest version Bison installed locally. Of course, it's not necessary for release builds from crates.io (because compiled `parser.rs` is included into release build).

If you use it from GitHub directly you also need Bison (because `parser.rs` is under gitignore)

## Bindings for other languages

WIP
