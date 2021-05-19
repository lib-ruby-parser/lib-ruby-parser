#!/bin/bash

set -eux

export RUST_BACKTRACE=1

rustup default stable
cargo test --features "onig"

rustup default nightly

export ASAN_OPTIONS=detect_leaks=1
export RUSTFLAGS=-Zsanitizer=address

cargo clean

run_c_tests() {
    LIB_RUBY_PARSER_PTR_SIZE=8 \
    LIB_RUBY_PARSER_MAYBE_PTR_SIZE=8 \
    LIB_RUBY_PARSER_LIST_SIZE=24 \
    LIB_RUBY_PARSER_STRING_PTR_SIZE=16 \
    LIB_RUBY_PARSER_SHARED_BYTE_LIST_SIZE=16 \
        cargo test --features "compile-with-external-structures,link-external-c-structures,nightly-features" "$@"
}

run_c_tests --test parser_test
run_c_tests --test lexer_test
run_c_tests --lib

cargo clean

run_cpp_tests() {
    LIB_RUBY_PARSER_PTR_SIZE=8 \
    LIB_RUBY_PARSER_MAYBE_PTR_SIZE=8 \
    LIB_RUBY_PARSER_LIST_SIZE=24 \
    LIB_RUBY_PARSER_STRING_PTR_SIZE=8 \
    LIB_RUBY_PARSER_SHARED_BYTE_LIST_SIZE=16 \
        cargo test --features "compile-with-external-structures,link-external-cpp-structures,nightly-features" "$@"
}

run_cpp_tests --test parser_test
run_cpp_tests --test lexer_test
run_cpp_tests --lib
