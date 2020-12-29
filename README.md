# lib-ruby-parser

[![test](https://github.com/lib-ruby-parser/lib-ruby-parser/workflows/test/badge.svg?branch=master)](https://github.com/lib-ruby-parser/lib-ruby-parser/actions?query=workflow%3Atest)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
![Crates.io](https://img.shields.io/crates/v/lib-ruby-parser)
[![codecov](https://codecov.io/gh/lib-ruby-parser/lib-ruby-parser/branch/master/graph/badge.svg)](https://codecov.io/gh/lib-ruby-parser/lib-ruby-parser)
[![MIT Licence](https://badges.frapsoft.com/os/mit/mit.svg?v=103)](https://opensource.org/licenses/mit-license.php)
[![dependency status](https://deps.rs/repo/github/lib-ruby-parser/lib-ruby-parser/status.svg)](https://deps.rs/repo/github/lib-ruby-parser/lib-ruby-parser)


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

[Full documentation](https://docs.rs/lib-ruby-parser)

## Features

TLDR; it's fast, it's precise, and it has a beautiful interface.

Comparison with `Ripper`/`RubyVM::AST`:
1. It's based on MRI's `parse.y`, and so it returns **exactly** the same sequence of tokens.
2. It's been tested on top 300 gems (by total downlads, that's about 3M LOC), `rubyspec` and `ruby/ruby` repos and there's no difference with `Ripper.lex`.
3. It's ~2-3 times faster than `Ripper` (with `jemalloc`), Ripper parses 3.9M LOC in 16-17s, `lib-ruby-parser` does it 8-9s. That's ~450K LOC/s. And these benchmarks include IO that is roughly the same in Ruby and Rust. Without IO (i.e. for example with mmaped file from tmpfs) the difference is even more noticeable, however it's not what you are going to do anyway. I think it's valid to include IO into benchmarks.
4. It has a much, much better interface. AST is strongly typed and well documented.
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
use lib_ruby_parser::source::{InputError, RecognizedEncoding, CustomDecoder};
use lib_ruby_parser::{Parser, ParserOptions, ParserResult};

fn decode(encoding: RecognizedEncoding, input: &[u8]) -> Result<Vec<u8>, InputError> {
    if let RecognizedEncoding::US_ASCII = encoding {
        // reencode and return Ok(result)
        return Ok(b"# encoding: us-ascii\n2 + 2".to_vec());
    }
    Err(InputError::DecodingError(
        "only us-ascii is supported".to_owned(),
    ))
}

/// // Or
let decode_closure = |encoding: RecognizedEncoding, input: &[u8]| -> Result<Vec<u8>, InputError> {
    if let RecognizedEncoding::US_ASCII = encoding {
        // reencode and return Ok(result)
        return Ok(b"# encoding: us-ascii\ndecoded".to_vec());
    }
    Err(InputError::DecodingError(
        "only us-ascii is supported".to_owned(),
    ))
};

let decoder = CustomDecoder::new(Box::new(decode_closure));
let options = ParserOptions { decoder, debug: true, ..Default::default() };
let mut parser = Parser::new(b"# encoding: us-ascii\n3 + 3", options);
let result = parser.do_parse();
println!("{:#?}", result);
// prints AST for "2 + 2"
```

## Invalid string values

Ruby doesn't require string literals to be valid in their encodings. This is why the following code is valid:

```ruby
# encoding: utf-8

"\xFF"
```

Byte sequence `255` is invalid in UTF-8, but MRI ignores it.

But not all languages support it, and this is why string and symbol nodes encapsulate a custom `StringValue` instead of a plain `String`.

If your language supports invalid strings you can use raw `.bytes` of this `StringValue`. For example, a Ruby wrapper for this library could do that.

If your language doesn't support it, better call `.to_string_lossy()` that replaces all unsupported chars with a special `U+FFFD REPLACEMENT CHARACTER (�)`.

## Regexes

Ruby constructs regexes from literals during parsing to:
1. validate them
2. declare local variables if regex is used for matching AND it contains named captures

To mirror this behavior `lib-ruby-parser` uses Onigurama to compile, validate and parse regex literals.

This feature is disabled by default, but you can add it by enabling `"onig"` feature.

## Bison

The grammar of `lib-ruby-parser` is built using a [custom bison skeleton](https://github.com/iliabylich/rust-bison-skeleton) that was written for this project.

For development you need the latest version of Bison installed locally. Of course, it's not necessary for release builds from crates.io (because compiled `parser.rs` is included into release build AND `build.rs` that converts it is excluded).

If you use it from GitHub directly you also need Bison (because `parser.rs` is under gitignore)

## Bindings for other languages

+ [C](https://github.com/lib-ruby-parser/c-bindings)
+ [C++](https://github.com/lib-ruby-parser/cpp-bindings)
+ [Node.js](https://github.com/lib-ruby-parser/node-bindings)
+ [WASM](https://github.com/lib-ruby-parser/wasm-bindings) (with live demo)

## Profiling

You can use `parse` example:

```sh
$ cargo run --all-features --example parse -- --no-output --profile "<pattern>"
```

## Benchmarking

A pretty big codebase could be generated using a `download.rb` script:

```sh
$ ruby gems/download.rb
$ cargo build --release --all-features --example parse
$ target/release/examples/parse --no-output --drop-tokens "gems/repos/**/*.rb"
```

## Profile-guided optimization

```sh
# Build recording executable
RUSTFLAGS="-Cprofile-generate=$(PWD)/target/pgo/pgo.profraw" cargo build --release --all-features --example parse

# Record raw profiling data
target/release/examples/parse --no-output "gems/repos/**/*.rb"

# Merge profiled data
llvm-profdata merge -o target/pgo/pgo.profraw/merged.profdata target/pgo/pgo.profraw

# Build optimized executable
RUSTFLAGS="-Cprofile-use=$(PWD)/target/pgo/pgo.profraw/merged.profdata" cargo build --release --all-features --example parse
```

PGO, No LTO:

```
$ repeat 5 time target/release/examples/parse --no-output "gems/repos/**/*.rb"
9.46s user 1.27s system 80% cpu 13.371 total
8.51s user 0.66s system 99% cpu 9.171 total
8.52s user 0.68s system 99% cpu 9.208 total
9.63s user 0.74s system 99% cpu 10.381 total
9.70s user 0.73s system 99% cpu 10.443 total
```

No PGO, LTO=fat:

```
$ repeat 5 time target/release/examples/parse --no-output "gems/repos/**/*.rb"
9.90s user 1.29s system 80% cpu 13.917 total
9.42s user 0.71s system 99% cpu 10.138 total
10.24s user 0.76s system 99% cpu 11.004 total
10.21s user 0.75s system 99% cpu 10.962 total
10.22s user 0.74s system 99% cpu 10.966 total
```

The diff seems to be too small to use this feature.

When both PGO and LTO are enabled building a `parse` example gives a bunch of LLVM errors about wrong types of functions (like `expected a Function or null`).

If you know how to fix them, please open an issue.
