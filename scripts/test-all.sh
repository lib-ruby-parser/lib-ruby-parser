#!/bin/bash

set -eu

export RUST_BACKTRACE=1

rustup default stable
cargo test --features "onig"

rustup default nightly

export ASAN_OPTIONS=detect_leaks=1
export RUSTFLAGS=-Zsanitizer=address

export LIB_RUBY_PARSER_PTR_SIZE=changeme
export LIB_RUBY_PARSER_MAYBE_PTR_SIZE=changeme
export LIB_RUBY_PARSER_LIST_SIZE=changeme
export LIB_RUBY_PARSER_STRING_PTR_SIZE=changeme
cargo test --features "compile-with-external-structures,link-external-c-structures,nightly-features" --test parser_test
cargo test --features "compile-with-external-structures,link-external-c-structures,nightly-features" -- containers

export LIB_RUBY_PARSER_PTR_SIZE=8
export LIB_RUBY_PARSER_MAYBE_PTR_SIZE=8
export LIB_RUBY_PARSER_LIST_SIZE=24
export LIB_RUBY_PARSER_STRING_PTR_SIZE=8
cargo test --features "compile-with-external-structures,link-external-cpp-structures,nightly-features" --test parser_test
cargo test --features "compile-with-external-structures,link-external-cpp-structures,nightly-features" --lib -- containers
