#!/bin/bash

set -eux

export RUST_BACKTRACE=1

# rustup default stable
cargo test --features "nightly-features" # --features "onig"

rustup default nightly

cargo clean

run_c_tests() {
    ASAN_OPTIONS=detect_leaks=1 \
    RUSTFLAGS=-Zsanitizer=address \
    LIB_RUBY_PARSER_PTR_SIZE=8 \
    LIB_RUBY_PARSER_MAYBE_PTR_SIZE=8 \
    LIB_RUBY_PARSER_LIST_SIZE=24 \
    LIB_RUBY_PARSER_STRING_PTR_SIZE=16 \
    LIB_RUBY_PARSER_SHARED_BYTE_LIST_SIZE=16 \
    LIB_RUBY_PARSER_BYTES_SIZE=24 \
    LIB_RUBY_PARSER_TOKEN_SIZE=56 \
    LIB_RUBY_PARSER_SOURCE_LINE_SIZE=24 \
        cargo test --features "compile-with-external-structures,link-with-external-c-structures,nightly-features" "$@"
}

# Linking fails on building doctests with ASAN enabled
run_c_tests --test parser_test
run_c_tests --test lexer_test
run_c_tests --lib

cargo clean

run_cpp_tests() {
    ASAN_OPTIONS=detect_leaks=1 \
    RUSTFLAGS=-Zsanitizer=address \
    LIB_RUBY_PARSER_PTR_SIZE=8 \
    LIB_RUBY_PARSER_MAYBE_PTR_SIZE=8 \
    LIB_RUBY_PARSER_LIST_SIZE=24 \
    LIB_RUBY_PARSER_STRING_PTR_SIZE=8 \
    LIB_RUBY_PARSER_SHARED_BYTE_LIST_SIZE=16 \
    LIB_RUBY_PARSER_BYTES_SIZE=24 \
    LIB_RUBY_PARSER_TOKEN_SIZE=56 \
    LIB_RUBY_PARSER_SOURCE_LINE_SIZE=24 \
        cargo test --features "compile-with-external-structures,link-with-external-cpp-structures,nightly-features" "$@"
}

# Linking fails on building doctests with ASAN enabled
run_cpp_tests --test parser_test
run_cpp_tests --test lexer_test
run_cpp_tests --lib
