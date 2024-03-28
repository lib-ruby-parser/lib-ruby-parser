# lib-ruby-parser

[![test](https://github.com/lib-ruby-parser/lib-ruby-parser/actions/workflows/test.yml/badge.svg)](https://github.com/lib-ruby-parser/lib-ruby-parser/actions/workflows/test.yml)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Crates.io](https://img.shields.io/crates/v/lib-ruby-parser?color=orange)](https://crates.io/crates/lib-ruby-parser)
[![codecov](https://codecov.io/gh/lib-ruby-parser/lib-ruby-parser/branch/master/graph/badge.svg)](https://codecov.io/gh/lib-ruby-parser/lib-ruby-parser)
[![MIT Licence](https://badges.frapsoft.com/os/mit/mit.svg?v=103)](https://opensource.org/licenses/mit-license.php)
[![dependency status](https://deps.rs/repo/github/lib-ruby-parser/lib-ruby-parser/status.svg)](https://deps.rs/repo/github/lib-ruby-parser/lib-ruby-parser)
[![Docs](https://img.shields.io/docsrs/lib-ruby-parser)](https://docs.rs/lib-ruby-parser)


`lib-ruby-parser` is a Ruby parser written in Rust.

Basic usage:

```rust
use lib_ruby_parser::{Parser, ParserOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = ParserOptions {
        buffer_name: "(eval)".to_string(),
        ..Default::default()
    };
    let mut parser = Parser::new(b"2 + 2".to_vec(), options);

    println!("{:#?}", parser.do_parse());

    Ok(())
}
```

[Full documentation](https://docs.rs/lib-ruby-parser)

## Features

TLDR; it's fast, it's precise, and it has a beautiful interface.

Comparison with `Ripper`/`RubyVM::AST`:
1. It's based on MRI's `parse.y`, and so it returns **exactly** the same sequence of tokens.
2. It's been tested on top 300 gems (by total downloads, that's about 3M LOC), `rubyspec` and `ruby/ruby` repos and there's no difference with `Ripper.lex`.
3. It's ~3 times faster than `Ripper` (with `jemalloc`), Ripper parses 3.9M LOC in ~16s, `lib-ruby-parser` does it in ~6.5s. That's ~600K LOC/s. You can find some benchmarks in the `bench/` directory, they don't include IO and GC.
4. It has a much, much better interface. AST is strongly typed and well documented.
5. It doesn't throw away information about tokens. All nodes have information about their source locations.

Comparison with [whitequark/parser](https://github.com/whitequark/parser):
1. It's much faster (the same corpus of 3M LOC can be parsed in 180s on the same machine)
1. It has a very similar interface (both in terms of AST structure and errors reporting)
3. However, AST is strongly typed, and so if something is nullable it's explicitly defined and documented.
4. What's important, it doesn't depend on Ruby

## Grammar versioning

`lib-ruby-parser` follows MRI/master. There are no plans to support multiple versions like it's done in `whitequark/parser`.

## Library versioning

| Ruby version | lib-ruby-parser version |
|--------------|-------------------------|
| 3.0.0        | 3.0.0+                  |
| 3.1.0        | 4.0.0+ruby-3.1.0        |

Starting from `4.0.0` lib-ruby-parser follows SemVer. Base version increments according to API changes,
while metadata matches current Ruby version, i.e. `X.Y.Z+ruby-A.B.C` means:

+ `X.Y.Z` base version
+ that parses Ruby `A.B.C`

Both versions bump separately.

## Encodings

By default `lib-ruby-parser` can only parse source files encoded in `UTF-8` or `ASCII-8BIT/BINARY`.

It's possible to pass a `decoder` function in `ParserOptions` that takes a recognized (by the library) encoding and a byte array. It must return a UTF-8 encoded byte array or an error:

```rust
use lib_ruby_parser::source::{InputError, Decoder, DecoderResult};
use lib_ruby_parser::{Parser, ParserOptions, ParserResult, LocExt};

fn decode(encoding: String, input: Vec<u8>) -> DecoderResult {
    if "US-ASCII" == encoding.to_uppercase() {
        // reencode and return Ok(result)
        return DecoderResult::Ok(b"# encoding: us-ascii\ndecoded".to_vec());
    }
    DecoderResult::Err(InputError::DecodingError(
        "only us-ascii is supported".to_string(),
    ))
}

let options = ParserOptions {
    decoder: Some(Decoder::new(Box::new(decode))),
    ..Default::default()
};
let mut parser = Parser::new(b"# encoding: us-ascii\n3 + 3".to_vec(), options);
let ParserResult { ast, input, .. } = parser.do_parse();

assert_eq!(ast.unwrap().expression().source(&input).unwrap(), "decoded".to_string())
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
+ [Ruby](https://github.com/lib-ruby-parser/ruby-bindings)
+ [WASM](https://github.com/lib-ruby-parser/wasm-bindings) (with live demo)

## Profiling

You can use `parse` example:

```sh
$ cargo run --bin parse --features=bin-parse -- --print=N --run-profiler --glob "blob/**/*.rb"
```

## Benchmarking

A codebase of 3.9M LOCs can be generated using a `download.rb` script:

```sh
$ ruby gems/download.rb
```

Then, run a script that compares `Ripper` and `lib-ruby-parser` (attached results are from Feb 2021):

```sh
$ ./scripts/bench.sh
    Finished release [optimized] target(s) in 0.08s
Running lib-ruby-parser
Run 1:
Time taken: 6.6232788220 (total files: 18018)
Run 2:
Time taken: 6.6498335800 (total files: 18018)
Run 3:
Time taken: 7.0684415810 (total files: 18018)
Run 4:
Time taken: 6.7987308510 (total files: 18018)
Run 5:
Time taken: 6.6954798760 (total files: 18018)
--------
Running MRI/ripper
Run 1:
Time taken: 22.92822499992326 (total files: 18017)
Run 2:
Time taken: 21.8613000002224 (total files: 18017)
Run 3:
Time taken: 21.96083900006488 (total files: 18017)
Run 4:
Time taken: 21.44488099985756 (total files: 18017)
Run 5:
Time taken: 21.738944000098854 (total files: 18017)
```

## Fuzz testing

First, make sure to switch to nightly:

```sh
$ rustup default nightly
```

Then install `cargo-fuzz`:

```sh
$ cargo install cargo-fuzz
```

And run the fuzzer (change the number of `--jobs` as you need or remove it to run only 1 parallel process):

```sh
$ RUST_BACKTRACE=1 cargo fuzz run parse --jobs=8 -- -max_len=50
```
